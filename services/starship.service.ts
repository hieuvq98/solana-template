import {
  Connection,
  Keypair,
  PublicKey,
  Transaction
} from '@solana/web3.js';
import BN from 'bn.js';
import moment from 'moment';
import { HashService, BorshService, SolanaService } from '@coin98/solana-support-library';
import { MerkleNode, MerkleTree, sendTransaction } from '@coin98/solana-support-library/core';
import { VaultService } from '@coin98/vault-js';
import {
  Launchpad,
  StarshipInstructionService,
  Whitelist,
  WHITELIST_LAYOUT
} from './starship_instruction.service';

export class StarshipService {
  static async createLaunchpad(
    connection: Connection,
    rootAccount: Keypair,
    ownerAccount: Keypair,
    launchpadName: string,
    tokenMint: PublicKey,
    priceN: BN,
    priceD: BN,
    minPerTx: BN,
    maxPerUser: BN,
    limitSale: BN,
    registerStartTimestamp: BN,
    registerEndTimestamp: BN,
    redeemStartTimestamp: BN,
    redeemEndTimestamp: BN,
    privateSaleSignature: Buffer | null,
    protocolFee: BN,
    sharingFee: BN,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

    const [launchpadAddress]: [PublicKey, number] = StarshipInstructionService.findLaunchpadAddress(launchpadName, starshipProgramId);

    if (!(await SolanaService.isAddressInUse(connection, launchpadAddress))) {
      const launchpadDerivationPath = StarshipInstructionService.findLaunchpadDerivationPath(launchpadName);
      const createLaunchpadInstruction = StarshipInstructionService.createLaunchpadInstruction(
        rootAccount.publicKey,
        launchpadDerivationPath,
        tokenMint,
        ownerAccount.publicKey,
        protocolFee,
        sharingFee,
        starshipProgramId
      );
      transaction.add(createLaunchpadInstruction);
    }
    const setLaunchpadInstruction = StarshipInstructionService.setLaunchpadInstruction(
      ownerAccount.publicKey,
      launchpadAddress,
      priceN,
      priceD,
      minPerTx,
      maxPerUser,
      limitSale,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
      privateSaleSignature,
      starshipProgramId
    );
    transaction.add(setLaunchpadInstruction);

    const txSign = await sendTransaction(connection, transaction, [
      rootAccount,
      ownerAccount,
    ]);
    console.info(`Created Launchpad ${launchpadAddress.toBase58()}`, '---', txSign, '\n');
    return launchpadAddress;
  }

  static async updateProtocolFee(
    connection: Connection,
    rootAccount: Keypair,
    launchpadAddress: PublicKey,
    protocolFee: BN,
    starshipProgramId: PublicKey
  ): Promise<string> {
    const transaction: Transaction = new Transaction()

    const updateProtocolFeeInstruction = StarshipInstructionService.updateProtocolFeeInstruction(
      rootAccount.publicKey,
      launchpadAddress,
      protocolFee,
      starshipProgramId
    )

    transaction.add(updateProtocolFeeInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      rootAccount,
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
    tokenMint: PublicKey,
    priceN: BN,
    priceD: BN,
    minPerTx: BN,
    maxPerUser: BN,
    limitSale: BN,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();
    const [lauchpadPurchaseAddress]: [PublicKey, number] = StarshipInstructionService.findLaunchpadPurchaseAddress(launchpadAddress, tokenMint, starshipProgramId)

    if (!(await SolanaService.isAddressInUse(connection, lauchpadPurchaseAddress))) {
      const createLaunchpadPurchaseInstruction = StarshipInstructionService.createLaunchpadPurchaseInstruction(
        ownerAccount.publicKey,
        launchpadAddress,
        tokenMint,
        starshipProgramId
      );
      transaction.add(createLaunchpadPurchaseInstruction);
    }
    const setLaunchpadPurchaseInstruction = StarshipInstructionService.setLaunchpadPurchaseInstruction(
      ownerAccount.publicKey,
      launchpadAddress,
      lauchpadPurchaseAddress,
      priceN,
      priceD,
      minPerTx,
      maxPerUser,
      limitSale,
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

    const [userProfileAddress]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(
      userAddress,
      launchpadAddress,
      starshipProgramId
    );

    const instruction = StarshipInstructionService.createUserProfileInstruction(
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
    payerAccount: Keypair,
    index: number,
    proofs: Buffer[],
    launchpadAddress: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

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
    const registerInstruction = StarshipInstructionService.registerInstruction(
      launchpadAddress,
      payerAccount.publicKey,
      index,
      proofs,
      starshipProgramId
    );
    transaction.add(registerInstruction);

    const txSign = await sendTransaction(connection, transaction, [
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
    amount: BN,
    starshipProgramId: PublicKey
  ): Promise<boolean> {
    const transaction = new Transaction();

    const withdrawTokenInstruction = StarshipInstructionService.withdrawTokenInstruction(
      rootAccount.publicKey,
      launchpadAddress,
      from,
      to,
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

  static async getLaunchpadAccountInfo(
    connection: Connection,
    launchpadAddress: PublicKey
  ): Promise<Launchpad> {
    const accountInfo = await connection.getAccountInfo(launchpadAddress);
    const data = StarshipInstructionService.decodeLaunchpadData(
      accountInfo.data
    );
    const [signerAddress] = await VaultService.findVaultSignerAddress(
      launchpadAddress,
      accountInfo.owner
    );
    data.signer = signerAddress;
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
    console.info('--- LAUNCHPAD ACCOUNT INFO ---');
    console.info(`Address:            ${launchpadAddress.toBase58()} -- ${launchpadAddress.toBuffer().toString('hex')}`);
    console.info(`Signer:             ${accountData.signer.toBase58()} -- ${accountData.signer.toBuffer().toString('hex')}`);
    console.info(`Nonce:              ${accountData.nonce}`);
    console.info(`Price in SOL:       ${accountData.priceN.toString()} / ${accountData.priceD.toString()} = ${accountData.priceN.div(accountData.priceD).toNumber()}`);
    console.info(`Private Signature:  ${accountData.privateSaleRoot.toString('hex')} - ${accountData.privateSaleRoot.toJSON().data}`);
    console.info(`Min per tx:         ${accountData.minPerTx.toNumber()}`);
    console.info(`Max per user:       ${accountData.maxPerUser.toNumber()}`);
    console.info(`Limit sale:         ${accountData.limitSale.toNumber()}`);
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

  static hashWhiteLists(whitelists: Whitelist[]): Buffer[] {
    return whitelists.map((item) => {
      const bytes = BorshService.serialize(WHITELIST_LAYOUT, item, 40);
      return HashService.keckka256(bytes);
    });
  }

  static getProof(tree: MerkleTree, index: number): MerkleNode[] {
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

export { Whitelist };

