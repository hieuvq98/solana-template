import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction
} from '@solana/web3.js';
import BN from 'bn.js';
import moment from 'moment';
import { HashService, BorshService, SolanaService } from '@coin98/solana-support-library';
import { MerkleNode, MerkleTree } from '@coin98/solana-support-library/core';
import { VaultService } from '@coin98/vault-js';
import {
  Launchpad,
  StarshipInstructionService,
  Whitelist,
  WHITELIST_LAYOUT
} from './starship_instruction.service';

export class StarshipService {
  static async createGlobalProfile(
    connection: Connection,
    payerAccount: Keypair,
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

    const [globalProfileAddress]: [PublicKey, number] = StarshipInstructionService.findUserGlobalProfileAddress(userAddress, starshipProgramId);

    const createGlobalProfileInstruction = StarshipInstructionService.createGlobalProfileInstruction(
      payerAccount.publicKey,
      userAddress,
      starshipProgramId
    );
    transaction.add(createGlobalProfileInstruction);

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
      payerAccount,
    ]);
    console.info(`Created global profile ${globalProfileAddress.toBase58()} of user ${userAddress.toBase58()}`, '---', txSign, '\n');
    return globalProfileAddress;
  }

  static async createLaunchpad(
    connection: Connection,
    payerAccount: Keypair,
    rootAccount: Keypair,
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
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

    const [launchpadAddress]: [PublicKey, number] = StarshipInstructionService.findLaunchpadAddress(launchpadName, starshipProgramId);

    if (!(await SolanaService.isAddressInUse(connection, launchpadAddress))) {
      const launchpadDerivationPath = StarshipInstructionService.findLaunchpadDerivationPath(launchpadName);
      const createLaunchpadInstruction = StarshipInstructionService.createLaunchpadInstruction(
        payerAccount.publicKey,
        launchpadDerivationPath,
        tokenMint,
        starshipProgramId
      );
      transaction.add(createLaunchpadInstruction);
    }
    const setLaunchpadInstruction = StarshipInstructionService.setLaunchpadInstruction(
      payerAccount.publicKey,
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

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
      payerAccount,
      rootAccount,
    ]);
    console.info(`Created Launchpad ${launchpadAddress.toBase58()}`, '---', txSign, '\n');
    return launchpadAddress;
  }

  static async createLaunchpadPurchase(
    connection: Connection,
    rootAccount: Keypair,
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
        rootAccount.publicKey,
        launchpadAddress,
        tokenMint,
        starshipProgramId
      );
      transaction.add(createLaunchpadPurchaseInstruction);
    }
    const setLaunchpadPurchaseInstruction = StarshipInstructionService.setLaunchpadPurchaseInstruction(
      rootAccount.publicKey,
      lauchpadPurchaseAddress,
      priceN,
      priceD,
      minPerTx,
      maxPerUser,
      limitSale,
      starshipProgramId
    );
    transaction.add(setLaunchpadPurchaseInstruction);

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
      rootAccount,
    ]);
    console.info(`Created Launchpad purchase ${lauchpadPurchaseAddress.toBase58()} of launchpad address ${launchpadAddress.toString()} - token mint ${tokenMint.toString()}`, '---', txSign, '\n');
    return lauchpadPurchaseAddress;
  }

  static async createLocalProfile(
    connection: Connection,
    payerAccount: Keypair,
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

    const [localProfileAddress]: [PublicKey, number] = StarshipInstructionService.findUserLocalProfileAddress(
      userAddress,
      launchpadAddress,
      starshipProgramId
    );

    const instruction = StarshipInstructionService.createLocalProfileInstruction(
      payerAccount.publicKey,
      launchpadAddress,
      userAddress,
      starshipProgramId
    );
    transaction.add(instruction);

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
      payerAccount,
    ]);
    console.info(`Created local profile ${localProfileAddress.toBase58()} of user ${userAddress.toBase58()}`, '---', txSign, '\n');
    return localProfileAddress;
  }

  static async redeemBySol(
    connection: Connection,
    payerAccount: Keypair,
    launchpadAddress: PublicKey,
    userTokenAddress: PublicKey,
    launchpadTokenAddress: PublicKey,
    amount: number,
    starshipProgramId: PublicKey
  ): Promise<string> {
    const transaction = new Transaction();

    const redeemBySolInstruction = StarshipInstructionService.redeemBySolInstruction(
      payerAccount.publicKey,
      launchpadAddress,
      userTokenAddress,
      launchpadTokenAddress,
      amount,
      starshipProgramId
    );
    transaction.add(redeemBySolInstruction);

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
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
      amount,
      starshipProgramId
    );
    transaction.add(redeemByTokenInstruction);

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
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

    const [globalProfileAddress]: [PublicKey, number] = StarshipInstructionService.findUserGlobalProfileAddress(
      payerAccount.publicKey,
      starshipProgramId
    );
    const [localProfileAddress]: [PublicKey, number] = StarshipInstructionService.findUserLocalProfileAddress(
      payerAccount.publicKey,
      launchpadAddress,
      starshipProgramId
    );

    if (
      !(await SolanaService.isAddressInUse(connection, globalProfileAddress))
    ) {
      const createGlobalProfileInstruction = StarshipInstructionService.createGlobalProfileInstruction(
        payerAccount.publicKey,
        payerAccount.publicKey,
        starshipProgramId
      );
      transaction.add(createGlobalProfileInstruction);
    }

    if (
      !(await SolanaService.isAddressInUse(connection, localProfileAddress))
    ) {
      const createLocalProfileInstruction = StarshipInstructionService.createLocalProfileInstruction(
        payerAccount.publicKey,
        launchpadAddress,
        payerAccount.publicKey,
        starshipProgramId
      );
      transaction.add(createLocalProfileInstruction);
    }
    const registerInstruction = StarshipInstructionService.registerInstruction(
      launchpadAddress,
      payerAccount.publicKey,
      index,
      proofs,
      starshipProgramId
    );
    transaction.add(registerInstruction);

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
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

  static async setBlacklistlist(
    connection: Connection,
    payerAccount: Keypair,
    userAddress: PublicKey,
    isBlacklisted: boolean,
    starshipProgramId: PublicKey
  ): Promise<boolean> {
    const transaction = new Transaction();

    const [globalProfileAddress]: [PublicKey, number] = StarshipInstructionService.findUserGlobalProfileAddress(userAddress, starshipProgramId);

    if (
      !(await SolanaService.isAddressInUse(connection, globalProfileAddress))
    ) {
      const createGlobalProfileInstruction = StarshipInstructionService.createGlobalProfileInstruction(
        payerAccount.publicKey,
        userAddress,
        starshipProgramId
      );
      transaction.add(createGlobalProfileInstruction);
    }

    const setBlacklistInstruction = StarshipInstructionService.setBlacklistInstruction(
      payerAccount.publicKey,
      userAddress,
      isBlacklisted,
      starshipProgramId
    );
    transaction.add(setBlacklistInstruction);

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
      payerAccount,
    ]);
    console.info(
      `Set blacklist for user ${payerAccount.publicKey.toBase58()}`,
      '---',
      txSign,
      '\n'
    );
    return true;
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

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
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

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
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

