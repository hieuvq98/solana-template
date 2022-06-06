import { BorshCoder, Idl } from "@project-serum/anchor"
import * as borsh from '@project-serum/borsh';
import {
  AccountMeta,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js';
import BN from 'bn.js';
import { TOKEN_PROGRAM_ID } from '@coin98/solana-support-library';
import StarshipIdl from "../target/idl/coin98_starship.json"

const coder = new BorshCoder(StarshipIdl as Idl)

export interface Whitelist {
  index: number;
  address: PublicKey;
}

interface CreateGlobalProfileRequest {
  nonce: number;
  user: PublicKey;
}

interface CreateLaunchpadRequest {
  launchpadPath: Buffer;
  launchpadNonce: number;
  signerNonce: number;
}

export const WHITELIST_LAYOUT = borsh.struct<Whitelist>([
  borsh.u32('index'),
  borsh.publicKey('address'),
]);

interface CreateLocalProfileRequest {
  nonce: number;
  user: PublicKey;
}

interface RedeemBySolRequest {
  amount: BN;
}

interface RedeemByTokenRequest {
  amount: BN;
}

interface RegisterRequest {
  index: number;
  proofs: Buffer[];
}

interface SetBlacklistRequest {
  isBlacklisted: boolean;
}

interface SetLaunchpadRequest {
  priceInSolN: BN;
  priceInSolD: BN;
  priceInTokenN: BN;
  priceInTokenD: BN;
  tokenProgramId: PublicKey;
  token0Mint: PublicKey;
  token1Mint: PublicKey;
  vaultProgramId: PublicKey;
  vault: PublicKey;
  vaultSigner: PublicKey;
  vaultToken0: PublicKey;
  vaultToken1: PublicKey;
  isPrivateSale: boolean;
  privateSaleSignature: Buffer;
  minPerTx: BN;
  maxPerUser: BN;
  registerStartTimestamp: BN;
  registerEndTimestamp: BN;
  redeemStartTimestamp: BN;
  redeemEndTimestamp: BN;
}

export interface GlobalProfile {
  user: PublicKey;
  isBlacklisted: boolean;
}

export interface Launchpad {
  signer?: PublicKey;
  nonce: number;
  priceInSolN: BN;
  priceInSolD: BN;
  priceInTokenN: BN;
  priceInTokenD: BN;
  tokenProgramId: PublicKey;
  token0Mint: PublicKey;
  token1Mint: PublicKey;
  vaultProgramId: PublicKey;
  vault: PublicKey;
  vaultSigner: PublicKey;
  vaultToken0: PublicKey;
  vaultToken1: PublicKey;
  isPrivateSale: boolean;
  privateSaleSignature: Buffer;
  minPerTx: BN;
  maxPerUser: BN;
  registerStartTimestamp: BN;
  registerEndTimestamp: BN;
  redeemStartTimestamp: BN;
  redeemEndTimestamp: BN;
  isActive: boolean;
}

export interface LocalProfile {
  launchpad: PublicKey;
  user: PublicKey;
  isRegistered: boolean;
  redeemedAmount: BN;
}

export class StarshipInstructionService {
  static createGlobalProfile(
    payerAddress: PublicKey,
    userAddress: PublicKey,
    userGlobalProfileAddress: PublicKey,
    userGlobalProfileNonce: number,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: CreateGlobalProfileRequest = {
      nonce: userGlobalProfileNonce,
      user: userAddress,
    };
    const data = coder.instruction.encode("createGlobalProfile", request)

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: payerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: userGlobalProfileAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static createLaunchpad(
    payerAddress: PublicKey,
    launchpadAddress: PublicKey,
    launchpadPath: Buffer,
    launchpadNonce: number,
    signerNonce: number,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: CreateLaunchpadRequest = {
      launchpadPath,
      launchpadNonce: launchpadNonce,
      signerNonce: signerNonce,
    };
    const data = coder.instruction.encode("createLaunchpad", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: payerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static setLaunchpad(
    launchpadAddress: PublicKey,
    rootAddress: PublicKey,
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
    minPerTransaction: number,
    maxPerUser: number,
    registerStartTimestamp: number,
    registerEndTimestamp: number,
    redeemStartTimestamp: number,
    redeemEndTimestamp: number,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: SetLaunchpadRequest = {
      priceInSolN: priceInSolN,
      priceInSolD: priceInSolD,
      priceInTokenN: priceInTokenN,
      priceInTokenD: priceInTokenD,
      tokenProgramId: TOKEN_PROGRAM_ID,
      token0Mint: token0MintAddress,
      token1Mint: token1MintAddress,
      vaultProgramId: vaultProgramId,
      vault: vaultAddress,
      vaultSigner: vaultSignerAddress,
      vaultToken0: vaultToken0Address,
      vaultToken1: vaultToken1Address,
      isPrivateSale: isPrivateSale,
      privateSaleSignature: privateSaleSignature,
      minPerTx: new BN(minPerTransaction),
      maxPerUser: new BN(maxPerUser),
      registerStartTimestamp: new BN(registerStartTimestamp),
      registerEndTimestamp: new BN(registerEndTimestamp),
      redeemStartTimestamp: new BN(redeemStartTimestamp),
      redeemEndTimestamp: new BN(redeemEndTimestamp),
    };
    const data = coder.instruction.encode("setLaunchpad", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: true, },
    ];
    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static createLocalProfile(
    payerAddress: PublicKey,
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    userLocalProfileAddress: PublicKey,
    userLocalProfileNonce: number,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: CreateLocalProfileRequest = {
      nonce: userLocalProfileNonce,
      user: userAddress,
    };
    const data = coder.instruction.encode("createLocalProfile", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: payerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userLocalProfileAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static decodeLaunchpadData(data: Buffer): Launchpad {
    return coder.accounts.decode("Launchpad", data)
  }

  static redeemBySol(
    launchpadAddress: PublicKey,
    launchpadSignerAddress: PublicKey,
    userAddress: PublicKey,
    userGlobalProfileAddress: PublicKey,
    userLocalProfileAddress: PublicKey,
    userToken1Address: PublicKey,
    vaultAddress: PublicKey,
    vaultSignerAddress: PublicKey,
    vaultToken1Address: PublicKey,
    amount: number,
    vaultProgramId: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: RedeemBySolRequest = {
      amount: new BN(amount),
    };
    const data = coder.instruction.encode("redeemBySol", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: launchpadSignerAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: userAddress, isSigner: true, isWritable: true },
      <AccountMeta>{ pubkey: userGlobalProfileAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userLocalProfileAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: userToken1Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: vaultAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: vaultSignerAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: vaultToken1Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: vaultProgramId, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static redeemByToken(
    launchpadAddress: PublicKey,
    launchpadSignerAddress: PublicKey,
    userAddress: PublicKey,
    userGlobalProfileAddress: PublicKey,
    userLocalProfileAddress: PublicKey,
    userToken0Address: PublicKey,
    userToken1Address: PublicKey,
    vaultAddress: PublicKey,
    vaultSignerAddress: PublicKey,
    vaultToken0Address: PublicKey,
    vaultToken1Address: PublicKey,
    amount: number,
    vaultProgramId: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: RedeemByTokenRequest = {
      amount: new BN(amount),
    };
    const data = coder.instruction.encode("redeemByToken", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: launchpadSignerAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: userGlobalProfileAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userLocalProfileAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: userToken0Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: userToken1Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: vaultAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: vaultSignerAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: vaultToken0Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: vaultToken1Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: vaultProgramId, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static register(
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    userIndex: number,
    userProofs: Buffer[],
    userGlobalProfileAddress: PublicKey,
    userLocalProfileAddress: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: RegisterRequest = {
      index: userIndex,
      proofs: userProofs,
    };
    const data = coder.instruction.encode("register", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: userGlobalProfileAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userLocalProfileAddress, isSigner: false, isWritable: true, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static setBlacklist(
    ownerAddress: PublicKey,
    userAddress: PublicKey,
    userGlobalProfileAddress: PublicKey,
    isBlacklisted: boolean,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: SetBlacklistRequest = {
      isBlacklisted: isBlacklisted,
    };
    const data = coder.instruction.encode("setBlacklist", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: ownerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: userAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: userGlobalProfileAddress, isSigner: false, isWritable: true, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }
}
