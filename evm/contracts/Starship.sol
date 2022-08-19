// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "./lib/Common.sol";
import "./lib/Token.sol";
import "./lib/Module.sol";
import "./lib/Merkle.sol";

/**
 * @dev Starship allows user to create raising fund campaign in a FIFO manner
 */
contract Starship is Ownable, Payable {

  mapping(uint256 => mapping(address => uint256)) private _allowances;
  mapping(uint256 => mapping(address => uint256)) private _exchangeTokenAllowances;
  mapping(uint256 => LaunchpadData) private _launchpadDatas;
  mapping(uint256 => mapping(address => ExchangeToken)) _exchangeTokens;
  mapping(uint256 => mapping(address => bool)) private _registrations;

  using SafeERC20 for IERC20;
  using SafeMath for uint256;

  /// @dev Initialize a new vault
  /// @param owner_ Owner of this pad
  constructor(address owner_) Ownable(owner_) {
  }

  struct LaunchpadData {
    address token;
    uint256 priceN;
    uint256 priceD;
    bytes32 privateSaleSignature;
    uint256 minPerTx;
    uint256 maxPerUser;
    uint256 maxAmount;
    uint256 registerStartTimestamp;
    uint256 registerEndTimestamp;
    uint256 redeemStartTimestamp;
    uint256 redeemEndTimestamp;
    bool isPrivateSale;
    bool isActive;
  }

  struct ExchangeToken {
    address token;
    uint256 priceN;
    uint256 priceD;
    uint256 maxAmount;
  }

  event LaunchpadUpdated(uint256 launchpadId);
  event ExchangeTokenUpdated(uint256 launchpadId, address indexed token);
  event Registered(uint256 launchpadId, address indexed recipient);
  event Redeemed(uint256 launchpadId, address indexed recipient, uint256 amount);
  event Withdrawn(address indexed owner, address indexed recipient, address indexed token, uint256 value);

  /// @dev User enroll to a particular launchpad. User must be registered in order to participate token sale
  /// @param launchpadId_ Launchpad ID
  /// @param index_ Ordinal of registration
  /// @param proofs_ Optional metadata of merkle tree to verify user is eligible to register
  function register(uint256 launchpadId_, uint256 index_, bytes32[] calldata proofs_) external {
    LaunchpadData storage launchpad = _launchpadDatas[launchpadId_];

    require(launchpad.isActive, "Starship: Invalid launchpad");
    require(launchpad.registerStartTimestamp <= block.timestamp && launchpad.registerEndTimestamp >= block.timestamp, "Starship: Not registration time");
    if(launchpad.isPrivateSale) {
      bytes32 node = keccak256(abi.encodePacked(index_, _msgSender()));
      require(MerkleProof.verify(proofs_, launchpad.privateSaleSignature, node), "Starship: Invalid proof");
    }

    _registrations[launchpadId_][_msgSender()] = true;

    emit Registered(launchpadId_, _msgSender());
  }

  /// @dev Buy token in the launchpad during redemption time
  /// @param launchpadId_ Launchpad ID
  /// @param token_ Type of token user use to exchange
  /// @param amount_ Amount of {token_} user use to exchange
  function redeem(uint256 launchpadId_, address token_, uint256 amount_) external payable {
    LaunchpadData storage launchpad = _launchpadDatas[launchpadId_];
    require(launchpad.isActive, "Starship: Invalid launchpad");
    require(launchpad.redeemStartTimestamp <= block.timestamp && launchpad.redeemEndTimestamp >= block.timestamp, "Starship: Not redemption time");
    require(_registrations[launchpadId_][_msgSender()], "Starship: Not registered");

    require(launchpad.minPerTx == 0 || amount_ >= launchpad.minPerTx, "Starship: Not meet minimum amount");
    uint256 allowance = _allowances[launchpadId_][_msgSender()];
    uint256 newAllowance = allowance.add(amount_);
    require(launchpad.maxPerUser == 0 || newAllowance <= launchpad.maxPerUser, "Starship: Allowance reached");

    if(_msgValue() > 0) {
      require(launchpad.priceN > 0, "Starship: Native token not supported");
      require(token_ == address(0), "Starship: Token not supported");
      uint256 sendingAmount = amount_.mul(launchpad.priceN);
      sendingAmount = sendingAmount.div(launchpad.priceD);
      uint256 tokenAllowance = _exchangeTokenAllowances[launchpadId_][token_];
      uint256 newTokenAllowance = tokenAllowance.add(sendingAmount);
      require(launchpad.maxAmount == 0 || newTokenAllowance <= launchpad.maxAmount, "Starship: Allowance reached");
      require(_msgValue() == sendingAmount, "Starship: Insuffient fund");
    }
    else {
      ExchangeToken storage exchangeToken = _exchangeTokens[launchpadId_][token_];
      require(exchangeToken.priceN > 0, "Starship: Token not supported");
      uint256 sendingAmount = amount_.mul(exchangeToken.priceN);
      sendingAmount = sendingAmount.div(exchangeToken.priceD);
      uint256 tokenAllowance = _exchangeTokenAllowances[launchpadId_][token_];
      uint256 newTokenAllowance = tokenAllowance.add(sendingAmount);
      require(exchangeToken.maxAmount == 0 || newTokenAllowance <= exchangeToken.maxAmount, "Starship: Allowance reached");
      IERC20(exchangeToken.token).safeTransferFrom(_msgSender(), address(this), sendingAmount);
    }

    _allowances[launchpadId_][_msgSender()] = newAllowance;
    IERC20(launchpad.token).safeTransfer(_msgSender(), amount_);

    emit Redeemed(launchpadId_, _msgSender(), amount_);
  }

  /// @dev Create/Update a launchpad. Those parameters can't be changed if the launch passed registration phase
  /// @param launchpadId_ Launchpad ID
  /// @param token_ Address of token that will be sold
  /// @param priceN_ Numerator of `token_` price in native token
  /// @param priceN_ Denominator of `token_` price in native token
  /// @param isPrivateSale_ Is this a private sale that need whitelist
  /// @param privateSaleSignature_ Root of merkle tree to prove a user to eligible to register
  /// @param minPerTx_ Minimum amount of `token_` must be executed in one transaction. 0 for unlimited
  /// @param maxPerUser_ Maximum amount of `token_` a user is allowed to buy during the sale. 0 for unlimited
  /// @param maxAmount_ Maximum amount native token can be used to buy `token_`
  /// @param timestamps_ Array of timestamps of milestones of the sale
  ///   0: Registration time start
  ///   1: Registration time end
  ///   2: Redemption time start
  ///   3: Redemption time end
  /// NOTE: This is a workaround for stack too deep error
  function setLaunchpad(
    uint256 launchpadId_, address token_, uint256 priceN_, uint256 priceD_,
    bool isPrivateSale_, bytes32 privateSaleSignature_, uint256 minPerTx_, uint256 maxPerUser_, uint256 maxAmount_,
    uint256[] memory timestamps_
  ) external onlyOwner {

    require(timestamps_.length == 4, "Starship: Invalid arguments");

    require(priceD_ > 0 || priceN_ == 0, "Starship: Invalid price");
    require(block.timestamp <= timestamps_[0], "Starship: Time must be set in the future");
    require(timestamps_[0] < timestamps_[1], "Starship: Invalid registration time");
    require(timestamps_[1] <= timestamps_[2], "Starship: Registration and sale time overlap");
    require(timestamps_[2] < timestamps_[3], "Starship: Invalid sale time");

    LaunchpadData storage launchpad = _launchpadDatas[launchpadId_];

    if (launchpad.registerStartTimestamp != 0) {
      require(launchpad.registerStartTimestamp >= block.timestamp, "Starship: Launchpad finalized");
    }

    launchpad.token = token_;
    launchpad.priceN = priceN_;
    launchpad.priceD = priceD_;
    launchpad.isPrivateSale = isPrivateSale_;
    launchpad.privateSaleSignature = privateSaleSignature_;
    launchpad.minPerTx = minPerTx_;
    launchpad.maxPerUser = maxPerUser_;
    launchpad.maxAmount = maxAmount_;
    launchpad.registerStartTimestamp = timestamps_[0];
    launchpad.registerEndTimestamp = timestamps_[1];
    launchpad.redeemStartTimestamp = timestamps_[2];
    launchpad.redeemEndTimestamp = timestamps_[3];
    launchpad.isActive = true;

    emit LaunchpadUpdated(launchpadId_);
  }

  /// @dev Get launchpad data
  /// @param launchpadId_ Launchpad ID
  function getLaunchpad(uint256 launchpadId_) external view returns (LaunchpadData memory) {
    return _launchpadDatas[launchpadId_];
  }

  /// @dev Change launchpad's status
  /// @param launchpadId_ Launchpad ID
  /// @param isActive_ Inactive/Active
  function setLaunchpadStatus(uint256 launchpadId_, bool isActive_) external onlyOwner {

    LaunchpadData storage launchpad = _launchpadDatas[launchpadId_];
    launchpad.isActive = isActive_;

    emit LaunchpadUpdated(launchpadId_);
  }

  /// @dev Change launchpad's status
  /// @param launchpadId_ Launchpad ID
  /// @param token_ Address of token that will be sold
  /// @param priceN_ Numerator of price in `token_`
  /// @param priceN_ Denominator of price in `token_`
  /// @param maxAmount_ Maximum amount of `token_` can be used to buy launchpad token
  function setExchangeToken(
    uint256 launchpadId_, address token_, uint256 priceN_, uint256 priceD_,
    uint256 maxAmount_
  ) external onlyOwner {
    require(token_ != address(0), "Starship: Token is zero address");
    require(priceD_ > 0 || priceN_ == 0, "Starship: Invalid price");

    ExchangeToken storage exchangeToken = _exchangeTokens[launchpadId_][token_];
    exchangeToken.token = token_;
    exchangeToken.priceN = priceN_;
    exchangeToken.priceD = priceD_;
    exchangeToken.maxAmount = maxAmount_;

    emit ExchangeTokenUpdated(launchpadId_, token_);
  }

  /// @dev withdraw the token in the vault, no limit
  /// @param token_ address of the token, use address(0) to withdraw gas token
  /// @param destination_ recipient address to receive the fund
  /// @param amount_ amount of fund to withdaw
  function withdraw(address token_, address destination_, uint256 amount_) external onlyOwner {
    require(destination_ != address(0), "Starship: Destination is zero address");

    uint256 availableAmount;
    if(token_ == address(0)) {
      availableAmount = address(this).balance;
    } else {
      availableAmount = IERC20(token_).balanceOf(address(this));
    }

    require(amount_ <= availableAmount, "Starship: Not enough balance");
    if(token_ == address(0)) {
      destination_.call{value:amount_}("");
    } else {
      IERC20(token_).safeTransfer(destination_, amount_);
    }

    emit Withdrawn(_msgSender(), destination_, token_, amount_);
  }

  /// @dev withdraw NFT from contract
  /// @param token_ address of the token, use address(0) to withdraw gas token
  /// @param destination_ recipient address to receive the fund
  /// @param tokenId_ ID of NFT to withdraw
  function withdrawNft(address token_, address destination_, uint256 tokenId_) external onlyOwner {
    require(destination_ != address(0), "Starship: destination is zero address");

    IERC721(token_).transferFrom(address(this), destination_, tokenId_);

    emit Withdrawn(_msgSender(), destination_, token_, 1);
  }
}

contract StarshipFactory is Ownable, Payable {

  using SafeERC20 for IERC20;

  constructor () Ownable(_msgSender()) {
  }

  /// @dev Emit `Created` when a new vault is created
  event Created(address indexed vault);
  /// @dev Emit `Withdrawn` when owner withdraw fund from the factory
  event Withdrawn(address indexed owner, address indexed recipient, address indexed token, uint256 value);

  /// @dev get contract address
  /// @param owner_ Owner of newly created pad
  /// @param salt_ an arbitrary value
  function getAddress(address owner_, uint salt_) public view returns (address) {
    bytes memory bytecode = _getBytecode(owner_);

    bytes32 hash = keccak256(abi.encodePacked(bytes1(0xff), address(this), salt_, keccak256(bytecode)));

    return address(uint160(uint(hash)));
  }


  /// @dev get byte code of launchpad Contract
  /// @param owner_ Owner of newly created pad
  function _getBytecode(address owner_) private pure returns (bytes memory) {
    bytes memory bytecode = type(Starship).creationCode;

    return abi.encodePacked(bytecode, abi.encode(owner_));
  }

  /// @dev deploy new contract with CREATE2 opcode
  /// @param code_ code of contract
  /// @param salt_ an arbitrary value
  function _deploy(bytes memory code_, uint256 salt_) private returns (address){
    address addr;
    assembly {
      addr := create2(0, add(code_, 0x20), mload(code_), salt_)
      if iszero(extcodesize(addr)) {
      revert(0, 0)
      }
    }
    return addr;
  }

  /// @dev create a new pad with CREATE2 opcode
  /// @param owner_ Owner of newly created pad
  /// @param salt_ an arbitrary value
  function createPad(address owner_, uint256 salt_) external returns (address pad) {
    bytes memory bytecode = _getBytecode(owner_);

    pad = _deploy(bytecode, salt_);

    emit Created(pad);
  }

  /// @dev withdraw token from contract
  /// @param token_ address of the token, use address(0) to withdraw gas token
  /// @param destination_ recipient address to receive the fund
  /// @param amount_ amount of fund to withdaw
  function withdraw(address token_, address destination_, uint256 amount_) external onlyOwner {
    require(destination_ != address(0), "Starship: Destination is zero address");

    uint256 availableAmount;
    if(token_ == address(0)) {
      availableAmount = address(this).balance;
    } else {
      availableAmount = IERC20(token_).balanceOf(address(this));
    }

    require(amount_ <= availableAmount, "Starship: Not enough balance");

    if(token_ == address(0)) {
      destination_.call{value:amount_}("");
    } else {
      IERC20(token_).safeTransfer(destination_, amount_);
    }

    emit Withdrawn(_msgSender(), destination_, token_, amount_);
  }

  /// @dev withdraw NFT from contract
  /// @param token_ address of the token, use address(0) to withdraw gas token
  /// @param destination_ recipient address to receive the fund
  /// @param tokenId_ ID of NFT to withdraw
  function withdrawNft(address token_, address destination_, uint256 tokenId_) external onlyOwner {
    require(destination_ != address(0), "Starship: destination is zero address");

    IERC721(token_).transferFrom(address(this), destination_, tokenId_);

    emit Withdrawn(_msgSender(), destination_, token_, 1);
  }
}
