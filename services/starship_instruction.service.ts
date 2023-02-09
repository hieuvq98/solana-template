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

interface CreateLaunchpadRequest {
  launchpadPath: Buffer;
  tokenMint: PublicKey
  owner: PublicKey
  protocolFee: BN
  sharingFee: BN
}

export const WHITELIST_LAYOUT = borsh.struct<Whitelist>([
  borsh.u32('index'),
  borsh.publicKey('address'),
]);

interface CreateUserProfileRequest {
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
interface TransferLaunchpadOwnershipRequest {
  newOwner: PublicKey
}
interface AcceptLaunchpadOwnershipRequest {
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

export interface UserProfile {
  launchpad: PublicKey;
  user: PublicKey;
  isRegistered: boolean;
  redeemedAmount: BN;
}

export class StarshipInstructionService {

  static createLaunchpadInstruction(
    payerAddress: PublicKey,
    launchpadPath: Buffer,
    tokenMint: PublicKey,
    owner: PublicKey,
    protocolFee: BN,
    sharingFee: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadAddress, ]: [PublicKey, number] = StarshipInstructionService.findLaunchpadAddress(launchpadPath, starshipProgramId)
    const request: CreateLaunchpadRequest = {
      launchpadPath,
      tokenMint,
      owner,
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
    ownerAddress: PublicKey,
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
      <AccountMeta>{ pubkey: ownerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: true, },
    ];
    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static updateProtocolFeeInstruction(
    rootAddress: PublicKey,
    launchpadAddress: PublicKey,
    protocolFee: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: UpdateProtocolFeeRequest = {
      protocolFee
    };
    const data = coder.instruction.encode("updateProtocolFee", request)
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

  static updateSharingFeeInstruction(
    ownerAddress: PublicKey,
    launchpadAddress: PublicKey,
    sharingFee: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: UpdateSharingFeeRequest = {
      sharingFee
    };
    const data = coder.instruction.encode("updateSharingFee", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: ownerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: true, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static transferLaunchpadOwnershipInstruction(
    ownerAddress: PublicKey,
    launchpadAddress: PublicKey,
    newOwner: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: TransferLaunchpadOwnershipRequest = {
      newOwner
    };
    const data = coder.instruction.encode("transferLaunchpadOwnership", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: ownerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: true, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static acceptLaunchpadOwnershipInstruction(
    newOwnerAddress: PublicKey,
    launchpadAddress: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const request: AcceptLaunchpadOwnershipRequest = {
    };
    const data = coder.instruction.encode("acceptLaunchpadOwnership", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: newOwnerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: true, },
    ];

    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static createLaunchpadPurchaseInstruction(
    ownerAddress: PublicKey,
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
      <AccountMeta>{ pubkey: ownerAddress, isSigner: true, isWritable: true },
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
    ownerAddress: PublicKey,
    launchpadAddress: PublicKey,
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
      <AccountMeta>{ pubkey: ownerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: launchpadPurchaseAddress, isSigner: false, isWritable: true, },
    ];
    return new TransactionInstruction({
      keys,
      data,
      programId: starshipProgramId,
    });
  }

  static createUserProfileInstruction(
    payerAddress: PublicKey,
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userProfileAddress, ]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(userAddress, launchpadAddress, starshipProgramId)
    const request: CreateUserProfileRequest = {
      user: userAddress,
    };
    const data = coder.instruction.encode("createUserProfile", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: payerAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userProfileAddress, isSigner: false, isWritable: true, },
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
    const [userProfileAddress, ]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(userAddress, launchpadAddress, starshipProgramId)

    const [launchpadSignerAddress, ]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, starshipProgramId)

    const request: RedeemBySolRequest = {
      amount: new BN(amount),
    };

    const data = coder.instruction.encode("redeemBySol", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: launchpadSignerAddress, isSigner: false, isWritable: true, },
      <AccountMeta>{ pubkey: userAddress, isSigner: true, isWritable: true },
      <AccountMeta>{ pubkey: userProfileAddress, isSigner: false, isWritable: true, },
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
    const [userProfileAddress, ]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(userAddress, launchpadAddress, starshipProgramId)

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
      <AccountMeta>{ pubkey: userProfileAddress, isSigner: false, isWritable: true, },
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
    const [userProfileAddress, ]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(userAddress, launchpadAddress, starshipProgramId)

    const request: RegisterRequest = {
      index: userIndex,
      proofs: userProofs,
    };
    const data = coder.instruction.encode("register", request)
    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: launchpadAddress, isSigner: false, isWritable: false, },
      <AccountMeta>{ pubkey: userAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: userProfileAddress, isSigner: false, isWritable: true, },
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

  static findUserProfileAddress(
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
