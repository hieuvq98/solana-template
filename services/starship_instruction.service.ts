import { BorshCoder, Idl } from "@project-serum/anchor"
import * as borsh from '@project-serum/borsh';
import {
  AccountMeta,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js';
import BN from 'bn.js';
import { HashService, TOKEN_PROGRAM_ID } from '@coin98/solana-support-library';
import StarshipIdl from "../target/idl/coin98_starship.json"

const coder = new BorshCoder(StarshipIdl as Idl)

export interface Whitelist {
  index: number;
  address: PublicKey;
}

interface CreateGlobalProfileRequest {
  user: PublicKey;
}

interface CreateLaunchpadRequest {
  launchpadPath: Buffer;
  tokenMint: PublicKey
  protocolFee: BN
  sharingFee: BN
}

export const WHITELIST_LAYOUT = borsh.struct<Whitelist>([
  borsh.u32('index'),
  borsh.publicKey('address'),
]);

interface CreateLocalProfileRequest {
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
  priceN: BN;
  priceD: BN;
  minPerTx: BN;
  maxPerUser: BN;
  limitSale: BN;
  registerStartTimestamp: BN;
  registerEndTimestamp: BN;
  redeemStartTimestamp: BN;
  redeemEndTimestamp: BN;
  privateSaleRoot: Buffer | null;
}

interface UpdateProtocolFeeRequest {
  protocolFee: BN
}
interface UpdateSharingFeeRequest {
  sharingFee: BN
}

interface CreateLaunchpadPurchaseRequest {
  tokenMint: PublicKey
}

interface SetLaunchpadPurchaseRequest {
  limitSale: BN
  priceN: BN
  priceD: BN
  minPerTx: BN
  maxPerUser: BN
}

interface WithdrawRequest {
  amount: BN
}

export interface GlobalProfile {
  user: PublicKey;
  isBlacklisted: boolean;
}

export interface Launchpad {
  signer?: PublicKey;
  nonce: number;
  signerNonce: number;
  isActive: boolean;
  priceN: BN;
  priceD: BN;
  minPerTx: BN;
  maxPerUser: BN;
  limitSale: BN;
  registerStartTimestamp: BN;
  registerEndTimestamp: BN;
  redeemStartTimestamp: BN;
  redeemEndTimestamp: BN;
  privateSaleRoot: Buffer;
  tokenMint: Buffer;
  owner: Buffer;
  newOwner: Buffer;
  protocolFee: BN
  sharingFee: BN
}

export interface LocalProfile {
  launchpad: PublicKey;
  user: PublicKey;
  isRegistered: boolean;
  redeemedAmount: BN;
}

export class StarshipInstructionService {
  static createGlobalProfileInstruction(
    payerAddress: PublicKey,
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userGlobalProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findUserGlobalProfileAddress(userAddress, starshipProgramId)

    const request: CreateGlobalProfileRequest = {
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

  static createLaunchpadInstruction(
    payerAddress: PublicKey,
    launchpadPath: Buffer,
    tokenMint: PublicKey,
    protocolFee: BN,
    sharingFee: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadAddress, ]: [PublicKey, number] = StarshipInstructionService.findLaunchpadAddress(launchpadPath, starshipProgramId)
    const request: CreateLaunchpadRequest = {
      launchpadPath,
      tokenMint,
      protocolFee,
      sharingFee
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

  static setLaunchpadInstruction(
    rootAddress: PublicKey,
    launchpadAddress: PublicKey,
    priceN: BN,
    priceD: BN,
    minPerTx: BN,
    maxPerUser: BN,
    limitSale: BN,
    registerStartTimestamp: BN,
    registerEndTimestamp: BN,
    redeemStartTimestamp: BN,
    redeemEndTimestamp: BN,
    privateSaleRoot: Buffer | null,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: SetLaunchpadRequest = {
      priceN,
      priceD,
      minPerTx,
      maxPerUser,
      limitSale,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
      privateSaleRoot
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

  static updateProtocolFeeInstruction(
    payerAddress: PublicKey,
    launchpadAddress: PublicKey,
    protocolFee: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: UpdateProtocolFeeRequest = {
      protocolFee
    };
    const data = coder.instruction.encode("updateProtocolFee", request)
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

  static updateSharingFeeInstruction(
    payerAddress: PublicKey,
    launchpadAddress: PublicKey,
    sharingFee: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: UpdateSharingFeeRequest = {
      sharingFee
    };
    const data = coder.instruction.encode("updateSharingFee", request)
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

  static createLaunchpadPurchaseInstruction(
    payerAddress: PublicKey,
    launchpadAddress: PublicKey,
    tokenMint: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadPurchaseAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadPurchaseAddress(launchpadAddress, tokenMint, starshipProgramId)
    const request: CreateLaunchpadPurchaseRequest = {
      tokenMint,
    };
    const data = coder.instruction.encode("createLaunchpadPurchase", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: payerAddress, isSigner: true, isWritable: true },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: launchpadPurchaseAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static setLaunchpadPurchaseInstruction(
    rootAddress: PublicKey,
    launchpadPurchaseAddress: PublicKey,
    limitSale: BN,
    priceN: BN,
    priceD: BN,
    minPerTx: BN,
    maxPerUser: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: SetLaunchpadPurchaseRequest = {
      priceN,
      priceD,
      minPerTx,
      maxPerUser,
      limitSale,
    };

    const data = coder.instruction.encode("setLaunchpadPurchase", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadPurchaseAddress, isSigner: false, isWritable: true, },
    ];
    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static createLocalProfileInstruction(
    payerAddress: PublicKey,
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userLocalProfileAddress, ]: [PublicKey, number] = StarshipInstructionService.findUserLocalProfileAddress(userAddress, launchpadAddress, starshipProgramId)
    const request: CreateLocalProfileRequest = {
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

  static redeemBySolInstruction(
    userAddress: PublicKey,
    launchpadAddress: PublicKey,
    userTokenAddress: PublicKey,
    launchpadTokenAddress: PublicKey,
    feeOwnerAddress: PublicKey,
    amount: number,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userGlobalProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findUserGlobalProfileAddress(userAddress, starshipProgramId)
    const [userLocalProfileAddress, ]: [PublicKey, number] = StarshipInstructionService.findUserLocalProfileAddress(userAddress, launchpadAddress, starshipProgramId)

    const [launchpadSignerAddress, ]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, starshipProgramId)

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
      <AccountMeta>{ pubkey: userTokenAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: launchpadTokenAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: feeOwnerAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static redeemByTokenInstruction(
    userAddress: PublicKey,
    launchpadAddress: PublicKey,
    launchpadPurchaseAddress: PublicKey,
    userToken0Address: PublicKey,
    userToken1Address: PublicKey,
    launchpadToken0Address: PublicKey,
    launchpadToken1Address: PublicKey,
    feeOwnerToken0Address: PublicKey,
    amount: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userGlobalProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findUserGlobalProfileAddress(userAddress, starshipProgramId)
    const [userLocalProfileAddress, ]: [PublicKey, number] = StarshipInstructionService.findUserLocalProfileAddress(userAddress, launchpadAddress, starshipProgramId)

    const [launchpadSignerAddress, ]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, starshipProgramId)

    const request: RedeemByTokenRequest = {
      amount,
    };
    const data = coder.instruction.encode("redeemByToken", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: launchpadPurchaseAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: launchpadSignerAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userAddress, isSigner: true, isWritable: true },
      <AccountMeta>{ pubkey: userGlobalProfileAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userLocalProfileAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: userToken0Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: userToken1Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: launchpadToken0Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: launchpadToken1Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: feeOwnerToken0Address, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static registerInstruction(
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    userIndex: number,
    userProofs: Buffer[],
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userGlobalProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findUserGlobalProfileAddress(userAddress, starshipProgramId)
    const [userLocalProfileAddress, ]: [PublicKey, number] = StarshipInstructionService.findUserLocalProfileAddress(userAddress, launchpadAddress, starshipProgramId)

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

  static setBlacklistInstruction(
    ownerAddress: PublicKey,
    userAddress: PublicKey,
    isBlacklisted: boolean,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userGlobalProfileAddress, ] = StarshipInstructionService.findUserGlobalProfileAddress(userAddress, starshipProgramId)
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

  static withdrawSolInstruction(
    ownerAddress: PublicKey,
    launchpadAddress: PublicKey,
    amount: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadSignerAddress, ]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, starshipProgramId)
    const request: WithdrawRequest = {
      amount
    };
    const data = coder.instruction.encode("withdrawSol", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: ownerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: launchpadSignerAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static withdrawTokenInstruction(
    ownerAddress: PublicKey,
    launchpadAddress: PublicKey,
    from: PublicKey,
    to: PublicKey,
    amount: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadSignerAddress, ]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, starshipProgramId)
    const request: WithdrawRequest = {
      amount
    };
    const data = coder.instruction.encode("withdrawToken", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: ownerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: launchpadSignerAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: from, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: to, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static findLaunchpadDerivationPath(identifier: string): Buffer {
    return HashService.sha256(identifier);
  }
  
  static findLaunchpadAddress(
    identifier: string | Buffer,
    starshipProgramId: PublicKey
  ): [PublicKey, number] {
    const derivationPath: Buffer = typeof identifier == 'string' ? StarshipInstructionService.findLaunchpadDerivationPath(identifier) : identifier
    return PublicKey.findProgramAddressSync(
      [Buffer.from([8, 201, 24, 140, 93, 100, 30, 148]), derivationPath],
      starshipProgramId
    );
  }

  static findLaunchpadPurchaseAddress(
    launchpadAddress: PublicKey,
    tokenMint: PublicKey,
    starshipProgramId: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from([68, 70, 141, 93, 102, 104, 120, 59, 54]), launchpadAddress.toBuffer(), tokenMint.toBuffer()],
      starshipProgramId
    );
  }

  static findLaunchpadSignerAddress(
    launchpadAddress: PublicKey,
    starshipProgramId: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from([2, 151, 229, 53, 244, 77, 229, 7]), launchpadAddress.toBuffer()],
      starshipProgramId
    );
  }

  static findUserGlobalProfileAddress(
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from([139, 126, 195, 157, 204, 134, 142, 146]), Buffer.from([32, 40, 118, 173, 164, 46, 192, 86]), userAddress.toBuffer()],
      starshipProgramId
    );
  }

  static findUserLocalProfileAddress(
    userAddress: PublicKey,
    launchpadAddress: PublicKey,
    starshipProgramId: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from([133, 177, 201, 78, 13, 152, 198, 180]), launchpadAddress.toBuffer(), userAddress.toBuffer()],
      starshipProgramId
    );
  }

}
