import {
  Connection,
  Ed25519Program,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction
} from '@solana/web3.js';
import BN from 'bn.js';
import moment from 'moment';
import { sendTransaction2, SolanaService,  } from '@coin98/solana-support-library';
import { MerkleNode, MerkleTreeKeccak, sendTransaction } from '@coin98/solana-support-library/core';
import {
  Launchpad,
  StarshipInstructionService,
  WhitelistParams,
} from './starship_instruction.service';

export class StarshipService {
  static async createLaunchpad(
    connection: Connection,
    payer: Keypair,
    launchpadName: string,
    tokenMint: PublicKey,
    priceN: BN,
    priceD: BN,
    minPerTx: BN,
    maxPerUser: BN,
    maxRegister: BN,
    totalLimit: BN,
    amountLimitInSol: BN,
    registerStartTimestamp: BN,
    registerEndTimestamp: BN,
    redeemStartTimestamp: BN,
    redeemEndTimestamp: BN,
    protocolFee: BN,
    sharingFee: BN,
    claimStartTimestamp: BN,
    whitelistAuthority: PublicKey | null,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();
    const [launchpadAddress]: [PublicKey, number] = StarshipInstructionService.findLaunchpadAddress(launchpadName, starshipProgramId);
    let isAddressUsed = await SolanaService.isAddressInUse(connection, launchpadAddress);
    if (!isAddressUsed) {
      const launchpadDerivationPath = StarshipInstructionService.findLaunchpadDerivationPath(launchpadName);
      const createLaunchpadInstruction = await StarshipInstructionService.createLaunchpadInstruction(
        payer.publicKey,
        launchpadDerivationPath,
        tokenMint,
        payer.publicKey,
        protocolFee,
        sharingFee,
        starshipProgramId
      );
      transaction.add(createLaunchpadInstruction);
    }

    const setLaunchpadInstruction = await StarshipInstructionService.setLaunchpadInstruction(
      payer.publicKey,
      launchpadAddress,
      priceN,
      priceD,
      minPerTx,
      maxPerUser,
      maxRegister,
      totalLimit,
      amountLimitInSol,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
      claimStartTimestamp,
      whitelistAuthority,
      starshipProgramId
    );

    transaction.add(setLaunchpadInstruction);
    const txSign = await sendTransaction2(connection, transaction, [
      payer,
    ]);

    console.info(`Created Launchpad ${launchpadAddress.toBase58()}`, '---', txSign[0], '\n');
    return launchpadAddress;
  }

  static async createWhitelistToken(
    connection: Connection,
    root: Keypair,
    tokenMint: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction: Transaction = new Transaction();
    const [whitelistTokenAddress, ]: [PublicKey, number] = StarshipInstructionService.findWhitelistTokenMintAddress(tokenMint,starshipProgramId)
    const createWhitelistTokenInstruction = StarshipInstructionService.createWhitelistTokenInstruction(
      root.publicKey,
      tokenMint,
      starshipProgramId
    );
    transaction.add(createWhitelistTokenInstruction);
     const txSign = await sendTransaction(connection, transaction, [
      root,
    ]);

    console.info(`Created Whitelist Token ${whitelistTokenAddress.toBase58()}`, '---', txSign, '\n');
    return whitelistTokenAddress;
  }

  static async updateProtocolFee(
    connection: Connection,
    admin: Keypair,
    launchpadAddress: PublicKey,
    protocolFee: BN,
    starshipProgramId: PublicKey
  ): Promise<string> {
    const transaction: Transaction = new Transaction()

    const updateProtocolFeeInstruction = StarshipInstructionService.updateProtocolFeeInstruction(
      admin.publicKey,
      launchpadAddress,
      protocolFee,
      starshipProgramId
    )

    transaction.add(updateProtocolFeeInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      admin,
    ]);
    console.info(`Update protocol fee ${launchpadAddress.toBase58()} - ${protocolFee.toString()}`, '---', txSign, '\n');

    return txSign
  }

  static async updateSharingFee(
    connection: Connection,
    ownerAccount: Keypair,
    launchpadAddress: PublicKey,
    sharingFee: BN,
    starshipProgramId: PublicKey
  ): Promise<string> {
    const transaction: Transaction = new Transaction()

    const updateSharingFeeInstruction = StarshipInstructionService.updateSharingFeeInstruction(
      ownerAccount.publicKey,
      launchpadAddress,
      sharingFee,
      starshipProgramId
    )

    transaction.add(updateSharingFeeInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      ownerAccount,
    ]);
    console.info(`Update sharing fee ${launchpadAddress.toBase58()} - ${sharingFee.toString()}`, '---', txSign, '\n');

    return txSign
  }

  static async transferLaunchpadOwnership(
    connection: Connection,
    ownerAccount: Keypair,
    launchpadAddress: PublicKey,
    newOwner: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<string> {
    const transaction: Transaction = new Transaction()

    const transferLaunchpadOwnershipInstruction = StarshipInstructionService.transferLaunchpadOwnershipInstruction(
      ownerAccount.publicKey,
      launchpadAddress,
      newOwner,
      starshipProgramId
    )

    transaction.add(transferLaunchpadOwnershipInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      ownerAccount,
    ]);
    console.info(`Transfer launchpad ownership ${launchpadAddress.toBase58()} - ${newOwner.toString()}`, '---', txSign, '\n');

    return txSign
  }

  static async acceptLaunchpadOwnership(
    connection: Connection,
    newOwnerAccount: Keypair,
    launchpadAddress: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<string> {
    const transaction: Transaction = new Transaction()

    const acceptLaunchpadOwnershipInstruction = StarshipInstructionService.acceptLaunchpadOwnershipInstruction(
      newOwnerAccount.publicKey,
      launchpadAddress,
      starshipProgramId
    )

    transaction.add(acceptLaunchpadOwnershipInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      newOwnerAccount,
    ]);
    console.info(`Accept launchpad ownership ${launchpadAddress.toBase58()} - ${newOwnerAccount.publicKey.toString()}`, '---', txSign, '\n');

    return txSign
  }

  static async createLaunchpadPurchase(
    connection: Connection,
    ownerAccount: Keypair,
    launchpadAddress: PublicKey,
    whitelistTokenMint: PublicKey,
    tokenMint: PublicKey,
    priceN: BN,
    priceD: BN,
    minPerTx: BN,
    maxPerUser: BN,
    amountLimitInToken: BN,
    sharingFee: BN,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();
    const [lauchpadPurchaseAddress]: [PublicKey, number] = StarshipInstructionService.findLaunchpadPurchaseAddress(launchpadAddress, tokenMint, starshipProgramId)

    if (!(await SolanaService.isAddressInUse(connection, lauchpadPurchaseAddress))) {
      const createLaunchpadPurchaseInstruction = StarshipInstructionService.createLaunchpadPurchaseInstruction(
        ownerAccount.publicKey,
        launchpadAddress,
        tokenMint,
        whitelistTokenMint,
        starshipProgramId
      );
      transaction.add(createLaunchpadPurchaseInstruction);
    }

    const setLaunchpadPurchaseInstruction = StarshipInstructionService.setLaunchpadPurchaseInstruction(
      ownerAccount.publicKey,
      launchpadAddress,
      tokenMint,
      priceN,
      priceD,
      minPerTx,
      maxPerUser,
      amountLimitInToken,
      sharingFee,
      starshipProgramId
    );

    transaction.add(setLaunchpadPurchaseInstruction);

    const txSign = await sendTransaction(connection, transaction, [
      ownerAccount,
    ]);
    console.info(`Created Launchpad purchase ${lauchpadPurchaseAddress.toBase58()} of launchpad address ${launchpadAddress.toString()} - token mint ${tokenMint.toString()}`, '---', txSign, '\n');
    return lauchpadPurchaseAddress;
  }

  static async createUserProfile(
    connection: Connection,
    payerAccount: Keypair,
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

    const [userProfileAddress]: [PublicKey, number] = await StarshipInstructionService.findUserProfileAddress(
      userAddress,
      launchpadAddress,
      starshipProgramId
    );

    const instruction = await StarshipInstructionService.createUserProfileInstruction(
      payerAccount.publicKey,
      launchpadAddress,
      userAddress,
      starshipProgramId
    );
    transaction.add(instruction);

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ]);
    console.info(`Created local profile ${userProfileAddress.toBase58()} of user ${userAddress.toBase58()}`, '---', txSign, '\n');
    return userProfileAddress;
  }

  static async redeemBySol(
    connection: Connection,
    payerAccount: Keypair,
    launchpadAddress: PublicKey,
    userTokenAddress: PublicKey,
    launchpadTokenAddress: PublicKey,
    feeOwner: PublicKey,
    appData: PublicKey,
    amount: number,
    starshipProgramId: PublicKey
  ): Promise<string> {
    const transaction = new Transaction();

    const redeemBySolInstruction = StarshipInstructionService.redeemBySolInstruction(
      payerAccount.publicKey,
      launchpadAddress,
      userTokenAddress,
      launchpadTokenAddress,
      feeOwner,
      appData,
      amount,
      starshipProgramId
    );
    transaction.add(redeemBySolInstruction);

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ]);
    console.info(`Redeemed ${amount} tokens using SOL`, '---', txSign, '\n');
    return txSign;
  }

  static async redeemByToken(
    connection: Connection,
    payerAccount: Keypair,
    launchpadAddress: PublicKey,
    launchpadPurchaseAddress: PublicKey,
    userToken0Address: PublicKey,
    userToken1Address: PublicKey,
    launchpadToken0Address: PublicKey,
    launchpadToken1Address: PublicKey,
    feeOwnerToken0Address: PublicKey,
    appData: PublicKey,
    amount: BN,
    starshipProgramId: PublicKey
  ): Promise<string> {
    const transaction = new Transaction();

    const redeemByTokenInstruction = StarshipInstructionService.redeemByTokenInstruction(
      payerAccount.publicKey,
      launchpadAddress,
      launchpadPurchaseAddress,
      userToken0Address,
      userToken1Address,
      launchpadToken0Address,
      launchpadToken1Address,
      feeOwnerToken0Address,
      appData,
      amount,
      starshipProgramId
    );
    transaction.add(redeemByTokenInstruction);

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ]);
    console.info(`Redeemed ${amount} tokens using Token0`, '---', txSign, '\n');
    return txSign;
  }

  static async register(
    connection: Connection,
    rootSigner: Keypair,
    payerAccount: Keypair,
    launchpadAddress: PublicKey,
    whitelist_id: BN,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();
    const whitelist_params: WhitelistParams = {
      launchpad: launchpadAddress,
      address: payerAccount.publicKey,
      whitelist_id: whitelist_id
    }
    const msg_hash = StarshipInstructionService.HashWhitelistMessage(whitelist_params);
    const signature = await StarshipInstructionService.signMessage(rootSigner,msg_hash);
    const validateSignatureInstruction: TransactionInstruction = Ed25519Program.createInstructionWithPublicKey({
      publicKey: rootSigner.publicKey.toBuffer(),
      message: msg_hash,
      signature
    })
    transaction.add(validateSignatureInstruction)

    const [userProfileAddress]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(
      payerAccount.publicKey,
      launchpadAddress,
      starshipProgramId
    );

    if (
      !(await SolanaService.isAddressInUse(connection, userProfileAddress))
    ) {
      const createUserProfileInstruction = StarshipInstructionService.createUserProfileInstruction(
        payerAccount.publicKey,
        launchpadAddress,
        payerAccount.publicKey,
        starshipProgramId
      );
      transaction.add(createUserProfileInstruction);
    }

    const [whitelistIdProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findWhitelistIdProfileAddress(
      launchpadAddress,
      whitelist_id,
      starshipProgramId
    );
    if (
      !(await SolanaService.isAddressInUse(connection, whitelistIdProfileAddress))
    ) {
      const createWhitelistIdProfileInstruction = StarshipInstructionService.createWhitelistIdProfileInstruction(
        launchpadAddress,
        whitelist_id,
        payerAccount.publicKey,
        starshipProgramId
      );
      transaction.add(createWhitelistIdProfileInstruction);
    }

    const registerInstruction = StarshipInstructionService.registerInstruction(
      launchpadAddress,
      payerAccount.publicKey,
      whitelist_id,
      signature,
      starshipProgramId
    );
    transaction.add(registerInstruction);

    const txSign = await sendTransaction(connection, transaction, [
      rootSigner,
      payerAccount,
    ]);
    console.info(
      `Registered user ${payerAccount.publicKey.toBase58()} to ${launchpadAddress.toBase58()}`,
      '---',
      txSign,
      '\n'
    );
    return launchpadAddress;
  }

  static async withdrawSol(
    connection: Connection,
    rootAccount: Keypair,
    launchpadAddress: PublicKey,
    amount: BN,
    starshipProgramId: PublicKey
  ): Promise<boolean> {
    const transaction = new Transaction();

    const withdrawSolInstruction = StarshipInstructionService.withdrawSolInstruction(
      rootAccount.publicKey,
      launchpadAddress,
      amount,
      starshipProgramId
    );
    transaction.add(withdrawSolInstruction);

    const txSign = await sendTransaction(connection, transaction, [
      rootAccount,
    ]);
    console.info(
      `Withdraw sol from ${launchpadAddress}`,
      '---',
      txSign,
      '\n'
    );
    return true;
  }

  static async withdrawToken(
    connection: Connection,
    rootAccount: Keypair,
    launchpadAddress: PublicKey,
    from: PublicKey,
    to: PublicKey,
    tokenMint: PublicKey,
    amount: BN,
    starshipProgramId: PublicKey
  ): Promise<boolean> {
    const transaction = new Transaction();

    const withdrawTokenInstruction = StarshipInstructionService.withdrawTokenInstruction(
      rootAccount.publicKey,
      launchpadAddress,
      from,
      to,
      tokenMint,
      amount,
      starshipProgramId
    );
    transaction.add(withdrawTokenInstruction);

    const txSign = await sendTransaction(connection, transaction, [
      rootAccount,
    ]);
    console.info(
      `Withdraw token from ${launchpadAddress}`,
      '---',
      txSign,
      '\n'
    );
    return true;
  }

  static async setAdmin(
    connection: Connection,
    rootAccount: Keypair,
    adminAddress: PublicKey,
    starshipProgramId: PublicKey
  ) : Promise<PublicKey> {
    const transaction = new Transaction();

    const [adminProfileAddress,] = StarshipInstructionService.findAdminProfileAddress(adminAddress,starshipProgramId);

    const setAdminInstruction = StarshipInstructionService.setAdminInstruction(rootAccount.publicKey,adminAddress,starshipProgramId);
    transaction.add(setAdminInstruction);
    const txSign = await sendTransaction(connection,transaction,[
      rootAccount
    ]);

    console.info(
      `Set new admin profile ${adminProfileAddress} for ${adminAddress}`,
      '---',
      txSign,
      '\n'
    );

    return adminProfileAddress;

  }

  static async getLaunchpadAccountInfo(
    connection: Connection,
    launchpadAddress: PublicKey
  ): Promise<Launchpad> {
    const accountInfo = await connection.getAccountInfo(launchpadAddress);
    console.info("accountInfo",accountInfo);
    const data = StarshipInstructionService.decodeLaunchpadData(
      accountInfo?.data
    );
    return data;
  }

  static async printLaunchpadAccountInfo(
    connection: Connection,
    launchpadAddress: PublicKey
  ): Promise<void> {
    const accountData = await this.getLaunchpadAccountInfo(
      connection,
      launchpadAddress
    );
    console.log('accountData',accountData)
    console.info('--- LAUNCHPAD ACCOUNT INFO ---');
    console.info(`Address:            ${launchpadAddress.toBase58()} -- ${launchpadAddress.toBuffer().toString('hex')}`);
    // console.info(`Signer:             ${accountData?.signer?.toBase58()} -- ${accountData?.signer?.toBuffer().toString('hex')}`);
    console.info(`Nonce:              ${accountData.nonce}`);
    console.info(`Price in SOL:       ${accountData.priceN.toString()} / ${accountData.priceD.toString()} = ${accountData.priceN.div(accountData.priceD).toNumber()}`);
    console.info(`Whitelist Authority:  ${accountData.whitelistAuthority?.toString('hex')} - ${accountData.whitelistAuthority?.toString('hex')}`);
    console.info(`Min per tx:         ${accountData.minPerTx.toNumber()}`);
    console.info(`Max per user:       ${accountData.maxPerUser.toNumber()}`);
    console.info(`Total Limit:         ${accountData.totalLimit.toNumber()}`);
    console.info(`Amount limit in sol:         ${accountData.amountLimitInSol.toNumber()}`);
    console.info(`Register time start:${moment(accountData.registerStartTimestamp.toNumber() * 1000).format('dddd, MMMM Do YYYY, hh:mm:ss')} -- ${accountData.registerStartTimestamp}`);
    console.info(`Register time end:  ${moment(accountData.registerEndTimestamp.toNumber() * 1000).format('dddd, MMMM Do YYYY, hh:mm:ss')} -- ${accountData.registerEndTimestamp}`);
    console.info(`Redeem time start:  ${moment(accountData.redeemStartTimestamp.toNumber() * 1000).format('dddd, MMMM Do YYYY, hh:mm:ss')} -- ${accountData.redeemStartTimestamp}`);
    console.info(`Redeem time end:    ${moment(accountData.redeemEndTimestamp.toNumber() * 1000).format('dddd, MMMM Do YYYY, hh:mm:ss')} -- ${accountData.redeemEndTimestamp}`);
    console.info(`Is active:          ${accountData.isActive}`);
    console.info(`Token mint:         ${accountData.tokenMint.toString()}`);
    console.info(`Owner:              ${accountData.owner.toString()}`);
    console.info(`New Owner:          ${accountData.newOwner.toString()}`);
    console.info(`Protocol Fee:       ${accountData.protocolFee.toString()}`);
    console.info(`Sharing Fee:        ${accountData.sharingFee.toString()}`);
    console.info('');
  }

  static getProof(tree: MerkleTreeKeccak, index: number): MerkleNode[] {
    const nodes = tree.nodes();
    const proofs = [];
    let currentIndex = index;
    for (let i = 0; i < nodes.length - 1; i++) {
      const proof = currentIndex % 2 == 0
        ? nodes[i][currentIndex + 1]
        : nodes[i][currentIndex - 1];
      currentIndex = (currentIndex - (currentIndex % 2)) / 2;
      proofs.push(proof);
    }

    return proofs;
  }
}

