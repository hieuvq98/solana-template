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
import { VaultService } from '../vault/vault.service';
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

    const [globalProfileAddress, globalProfileNonce]: [PublicKey, number] = await this.findUserGlobalProfileAddress(userAddress, starshipProgramId);

    const instruction = StarshipInstructionService.createGlobalProfile(
      payerAccount.publicKey,
      userAddress,
      globalProfileAddress,
      globalProfileNonce,
      starshipProgramId
    );
    transaction.add(instruction);

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
    priceInSolN: BN,
    priceInSolD: BN,
    priceInTokenN: BN,
    priceInTokenD: BN,
    token0MintAddress: PublicKey,
    token1MintAddress: PublicKey,
    vaultProgramId: PublicKey,
    vaultAddress: PublicKey,
    vaultSignerAddress: PublicKey,
    vaultToken0Address: PublicKey,
    vaultToken1Address: PublicKey,
    isPrivateSale: boolean,
    privateSaleSignature: Buffer,
    saleLimitPerTransaction: number,
    saleLimitPerUser: number,
    registerStartTimestamp: number,
    registerEndTimestamp: number,
    redeemStartTimestamp: number,
    redeemEndTimestamp: number,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

    const [launchpadAddress, launchpadNonce]: [PublicKey, number] = await this.findLaunchpadAddress(launchpadName, starshipProgramId);
    const [, signerNonce]: [PublicKey, number] = await this.findLaunchpadSignerAddress(
      launchpadAddress,
      starshipProgramId
    );
    if (!(await SolanaService.isAddressInUse(connection, launchpadAddress))) {
      const launchpadDerivationPath = this.findLaunchpadDerivationPath(launchpadName);
      const createLaunchpadInstruction = StarshipInstructionService.createLaunchpad(
        payerAccount.publicKey,
        launchpadAddress,
        launchpadDerivationPath,
        launchpadNonce,
        signerNonce,
        starshipProgramId
      );
      transaction.add(createLaunchpadInstruction);
    }
    const setLaunchpadInstruction = StarshipInstructionService.setLaunchpad(
      launchpadAddress,
      payerAccount.publicKey,
      priceInSolN,
      priceInSolD,
      priceInTokenN,
      priceInTokenD,
      token0MintAddress,
      token1MintAddress,
      vaultProgramId,
      vaultAddress,
      vaultSignerAddress,
      vaultToken0Address,
      vaultToken1Address,
      isPrivateSale,
      privateSaleSignature,
      saleLimitPerTransaction,
      saleLimitPerUser,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
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

  static async createLocalProfile(
    connection: Connection,
    payerAccount: Keypair,
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

    const [localProfileAddress, localProfileNonce]: [PublicKey, number] = await this.findUserLocalProfileAddress(
      userAddress,
      launchpadAddress,
      starshipProgramId
    );

    const instruction = StarshipInstructionService.createLocalProfile(
      payerAccount.publicKey,
      launchpadAddress,
      userAddress,
      localProfileAddress,
      localProfileNonce,
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
    userToken1Address: PublicKey,
    vaultAddress: PublicKey,
    vaultToken1Address: PublicKey,
    amount: number,
    vaultProgrammId: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

    const [launchpadSignerAddress]: [PublicKey, number] = await this.findLaunchpadSignerAddress(
      launchpadAddress,
      starshipProgramId
    );
    const [vaultSignerAddress]: [PublicKey, number] = await VaultService.findVaultSignerAddress(vaultAddress, vaultProgrammId);
    const [globalProfileAddress]: [PublicKey, number] = await this.findUserGlobalProfileAddress(
      payerAccount.publicKey,
      starshipProgramId
    );
    const [localProfileAddress]: [PublicKey, number] = await this.findUserLocalProfileAddress(
      payerAccount.publicKey,
      launchpadAddress,
      starshipProgramId
    );

    const instruction = StarshipInstructionService.redeemBySol(
      launchpadAddress,
      launchpadSignerAddress,
      payerAccount.publicKey,
      globalProfileAddress,
      localProfileAddress,
      userToken1Address,
      vaultAddress,
      vaultSignerAddress,
      vaultToken1Address,
      amount,
      vaultProgrammId,
      starshipProgramId
    );
    transaction.add(instruction);

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
      payerAccount,
    ]);
    console.info(`Redeemed ${amount} tokens using SOL`, '---', txSign, '\n');
    return localProfileAddress;
  }

  static async redeemByToken(
    connection: Connection,
    payerAccount: Keypair,
    launchpadAddress: PublicKey,
    userToken0Address: PublicKey,
    userToken1Address: PublicKey,
    vaultAddress: PublicKey,
    vaultToken0Address: PublicKey,
    vaultToken1Address: PublicKey,
    amount: number,
    vaultProgrammId: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<PublicKey> {
    const transaction = new Transaction();

    const [launchpadSignerAddress]: [PublicKey, number] = await this.findLaunchpadSignerAddress(
      launchpadAddress,
      starshipProgramId
    );
    const [vaultSignerAddress]: [PublicKey, number] = await VaultService.findVaultSignerAddress(vaultAddress, vaultProgrammId);
    const [globalProfileAddress]: [PublicKey, number] = await this.findUserGlobalProfileAddress(
      payerAccount.publicKey,
      starshipProgramId
    );
    const [localProfileAddress]: [PublicKey, number] = await this.findUserLocalProfileAddress(
      payerAccount.publicKey,
      launchpadAddress,
      starshipProgramId
    );

    const instruction = StarshipInstructionService.redeemByToken(
      launchpadAddress,
      launchpadSignerAddress,
      payerAccount.publicKey,
      globalProfileAddress,
      localProfileAddress,
      userToken0Address,
      userToken1Address,
      vaultAddress,
      vaultSignerAddress,
      vaultToken0Address,
      vaultToken1Address,
      amount,
      vaultProgrammId,
      starshipProgramId
    );
    transaction.add(instruction);

    const txSign = await sendAndConfirmTransaction(connection, transaction, [
      payerAccount,
    ]);
    console.info(`Redeemed ${amount} tokens using Token0`, '---', txSign, '\n');
    return localProfileAddress;
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

    const [globalProfileAddress, globalProfileNonce]: [PublicKey, number] = await this.findUserGlobalProfileAddress(
      payerAccount.publicKey,
      starshipProgramId
    );
    const [localProfileAddress, localProfileNonce]: [PublicKey, number] = await this.findUserLocalProfileAddress(
      payerAccount.publicKey,
      launchpadAddress,
      starshipProgramId
    );

    if (
      !(await SolanaService.isAddressInUse(connection, globalProfileAddress))
    ) {
      const createGlobalProfileInstruction = StarshipInstructionService.createGlobalProfile(
        payerAccount.publicKey,
        payerAccount.publicKey,
        globalProfileAddress,
        globalProfileNonce,
        starshipProgramId
      );
      transaction.add(createGlobalProfileInstruction);
    }

    if (
      !(await SolanaService.isAddressInUse(connection, localProfileAddress))
    ) {
      const createLocalProfileInstruction = StarshipInstructionService.createLocalProfile(
        payerAccount.publicKey,
        launchpadAddress,
        payerAccount.publicKey,
        localProfileAddress,
        localProfileNonce,
        starshipProgramId
      );
      transaction.add(createLocalProfileInstruction);
    }
    const registerInstruction = StarshipInstructionService.register(
      launchpadAddress,
      payerAccount.publicKey,
      index,
      proofs,
      globalProfileAddress,
      localProfileAddress,
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

    const [globalProfileAddress, globalProfileNonce]: [PublicKey, number] = await this.findUserGlobalProfileAddress(userAddress, starshipProgramId);

    if (
      !(await SolanaService.isAddressInUse(connection, globalProfileAddress))
    ) {
      const createGlobalProfileInstruction = StarshipInstructionService.createGlobalProfile(
        payerAccount.publicKey,
        userAddress,
        globalProfileAddress,
        globalProfileNonce,
        starshipProgramId
      );
      transaction.add(createGlobalProfileInstruction);
    }

    const setBlacklistInstruction = StarshipInstructionService.setBlacklist(
      payerAccount.publicKey,
      userAddress,
      globalProfileAddress,
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

  static findLaunchpadDerivationPath(identifier: string): Buffer {
    return HashService.sha256(identifier);
  }

  static async findLaunchpadAddress(
    identifier: string,
    starshipProgramId: PublicKey
  ): Promise<[PublicKey, number]> {
    const prefix: Buffer = HashService.sha256('Launchpad').slice(0, 8);
    const derivationPath: Buffer = this.findLaunchpadDerivationPath(identifier);
    return PublicKey.findProgramAddress(
      [prefix, derivationPath],
      starshipProgramId
    );
  }

  static async findLaunchpadSignerAddress(
    launchpadAddress: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<[PublicKey, number]> {
    const prefix: Buffer = HashService.sha256('Signer').slice(0, 8);
    return PublicKey.findProgramAddress(
      [prefix, launchpadAddress.toBuffer()],
      starshipProgramId
    );
  }

  static async findUserGlobalProfileAddress(
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<[PublicKey, number]> {
    const prefix: Buffer = HashService.sha256('GlobalProfile').slice(0, 8);
    const prefix2: Buffer = HashService.sha256('Lunapad').slice(0, 8);
    return PublicKey.findProgramAddress(
      [prefix, prefix2, userAddress.toBuffer()],
      starshipProgramId
    );
  }

  static async findUserLocalProfileAddress(
    userAddress: PublicKey,
    launchpadAddress: PublicKey,
    starshipProgramId: PublicKey
  ): Promise<[PublicKey, number]> {
    const prefix: Buffer = HashService.sha256('LocalProfile').slice(0, 8);
    return PublicKey.findProgramAddress(
      [prefix, launchpadAddress.toBuffer(), userAddress.toBuffer()],
      starshipProgramId
    );
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
    console.info(`Price in SOL:       ${accountData.priceInSolN.toString()} / ${accountData.priceInSolD.toString()} = ${accountData.priceInSolN.div(accountData.priceInSolD).toNumber()}`);
    console.info(`Price in Token:     ${accountData.priceInTokenN.toString()} / ${accountData.priceInTokenD.toString()} = ${accountData.priceInTokenN.div(accountData.priceInTokenD).toNumber()}`);
    console.info(`Token0 Mint:        ${accountData.token0Mint.toBase58()} -- ${accountData.token0Mint.toBuffer().toString('hex')}`);
    console.info(`Token1 Mint:        ${accountData.token1Mint.toBase58()} -- ${accountData.token1Mint.toBuffer().toString('hex')}`);
    console.info(`Vault:              ${accountData.vault.toBase58()} -- ${accountData.vault.toBuffer().toString('hex')}`);
    console.info(`Vault Signer:       ${accountData.vaultSigner.toBase58()} -- ${accountData.vaultSigner.toBuffer().toString('hex')}`);
    console.info(`Vault Token0:       ${accountData.vaultToken0.toBase58()} -- ${accountData.vaultToken0.toBuffer().toString('hex')}`);
    console.info(`Vault Token1:       ${accountData.vaultToken1.toBase58()} -- ${accountData.vaultToken1.toBuffer().toString('hex')}`);
    console.info(`Is Private:         ${accountData.isPrivateSale}`);
    console.info(`Private Signature:  ${accountData.privateSaleSignature.toString('hex')} - ${accountData.privateSaleSignature.toJSON().data}`);
    console.info(`Min per tx:         ${accountData.minPerTransaction.toNumber()}`);
    console.info(`Max per user:       ${accountData.maxPerUser.toNumber()}`);
    console.info(`Register time start:${moment(accountData.registerStartTimestamp.toNumber() * 1000).format('dddd, MMMM Do YYYY, hh:mm:ss')} -- ${accountData.registerStartTimestamp}`);
    console.info(`Register time end:  ${moment(accountData.registerEndTimestamp.toNumber() * 1000).format('dddd, MMMM Do YYYY, hh:mm:ss')} -- ${accountData.registerEndTimestamp}`);
    console.info(`Redeem time start:  ${moment(accountData.redeemStartTimestamp.toNumber() * 1000).format('dddd, MMMM Do YYYY, hh:mm:ss')} -- ${accountData.redeemStartTimestamp}`);
    console.info(`Redeem time end:    ${moment(accountData.redeemEndTimestamp.toNumber() * 1000).format('dddd, MMMM Do YYYY, hh:mm:ss')} -- ${accountData.redeemEndTimestamp}`);
    console.info(`Is active:          ${accountData.isActive}`);
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

