import { Keypair } from '@solana/web3.js';
import { BorshCoder, Idl } from "@project-serum/anchor"
import * as borsh from '@project-serum/borsh';
import * as ed from "@noble/ed25519"
import {
  AccountMeta,
  PublicKey,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  TransactionInstruction,
} from '@solana/web3.js';
import BN from 'bn.js';
import { HashService, TOKEN_PROGRAM_ID } from '@coin98/solana-support-library';
import { IdlParserService } from "@coin98/solana-support-library";
import StarshipIdl from "../target/idl/coin98_starship.json";

const coder = new BorshCoder(StarshipIdl as Idl)
const parser = new IdlParserService(StarshipIdl as Idl) as any;
export interface WhitelistParams {
  launchpad: PublicKey,
  address: PublicKey,
  whitelist_id: BN
}

export const WHITELIST_LAYOUT = borsh.struct<WhitelistParams>([
  borsh.publicKey('launchpad'),
  borsh.publicKey('address'),
  borsh.u64("whitelist_id")
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
  priceN: BN
  priceD: BN
  minPerTx: BN
  maxPerUser: BN
  amountLimitInToken: BN
  sharingFee: BN
}

interface WithdrawRequest {
  amount: BN
}

export interface GlobalProfile {
  user: PublicKey;
  isBlacklisted: boolean;
}
export interface Launchpad {
  nonce: number;
  signerNonce: number;
  isActive: boolean;
  priceN: BN;
  priceD: BN;
  minPerTx: BN;
  maxPerUser: BN;
  totalLimit: BN,
  totalSold: BN,
  totalClaimed: BN,
  amountSoldInSol: BN,
  amountLimitInSol: BN,
  registerStartTimestamp: BN;
  registerEndTimestamp: BN;
  redeemStartTimestamp: BN;
  redeemEndTimestamp: BN;
  claimStartTimestamp: BN;
  whitelistAuthority: Buffer
  tokenMint: Buffer;
  owner: Buffer;
  newOwner: Buffer;
  protocolFee: BN
  sharingFee: BN,
}

export interface TimestampAccount {
  cur: BN
}

export interface UserProfile {
  launchpad: PublicKey;
  user: PublicKey;
  isRegistered: boolean;
  redeemedAmount: BN;
}

export class StarshipInstructionService {

  static createLaunchpadInstruction(
    payerAccount: PublicKey,
    launchpadPath: Buffer,
    tokenMint: PublicKey,
    owner: PublicKey,
    protocolFee: BN,
    sharingFee: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadAccount,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadAddress(launchpadPath, starshipProgramId)
    const args = {
      launchpadPath,
      tokenMint,
      owner,
      protocolFee,
      sharingFee
    };
    const accounts = {
      root: payerAccount,
      launchpad: launchpadAccount,
      systemProgram: SystemProgram.programId
    }
    return parser.createLaunchpad(
      args,
      accounts,
      starshipProgramId
    );
  }

  static createWhitelistTokenInstruction(
    rootAddress: PublicKey,
    tokenMint: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [whitelistTokenAddress,]: [PublicKey, number] = StarshipInstructionService.findWhitelistTokenMintAddress(tokenMint, starshipProgramId)

    const args = {
      tokenMint
    };

    const accounts = {
      root: rootAddress,
      whitelist: whitelistTokenAddress,
      systemProgram: SystemProgram.programId
    }
    return parser.createWhitelistToken(
      args,
      accounts,
      starshipProgramId
    )
  }

  static createWhitelistIdProfileInstruction(
    launchpadAddress: PublicKey,
    whitelist_id: BN,
    payerAccount: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [whitelistIdProfileAddress]: [PublicKey, number] = StarshipInstructionService.findWhitelistIdProfileAddress(
      launchpadAddress,
      whitelist_id,
      starshipProgramId
    );

    const args = {
      whitelistId: whitelist_id
    };

    const accounts = {
      payer: payerAccount,
      launchpad: launchpadAddress,
      whitelistIdProfile: whitelistIdProfileAddress,
      systemProgram: SystemProgram.programId
    }

    return parser.createWhitelistIdProfile(
      args,
      accounts,
      starshipProgramId
    )
  }

  static setLaunchpadInstruction(
    ownerAddress: PublicKey,
    launchpadAddress: PublicKey,
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
    claimStartTimestamp: BN,
    whitelistAuthority: PublicKey | null,
    starshipProgramId: PublicKey
  ): TransactionInstruction {

    const args = {
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
      whitelistAuthority
    };
    const accounts = {
      owner: ownerAddress,
      launchpad: launchpadAddress
    }
    return parser.setLaunchpad(args, accounts, starshipProgramId);
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
    whitelistTokenMintAddress: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadPurchaseAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadPurchaseAddress(launchpadAddress, tokenMint, starshipProgramId);
    const request: CreateLaunchpadPurchaseRequest = {
      tokenMint,
    };
    const accounts = {
      owner: ownerAddress,
      launchpad: launchpadAddress,
      whitelistTokenMint: whitelistTokenMintAddress,
      launchpadPurchase: launchpadPurchaseAddress,
      systemProgram: SystemProgram.programId
    }
    return parser.createLaunchpadPurchase(request, accounts, starshipProgramId)

  }

  static setLaunchpadPurchaseInstruction(
    ownerAddress: PublicKey,
    launchpadAddress: PublicKey,
    tokenMint: PublicKey,
    priceN: BN,
    priceD: BN,
    minPerTx: BN,
    maxPerUser: BN,
    amountLimitInToken: BN,
    sharingFee: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadPurchaseAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadPurchaseAddress(launchpadAddress, tokenMint, starshipProgramId);

    const request: SetLaunchpadPurchaseRequest = {
      priceN,
      priceD,
      minPerTx,
      maxPerUser,
      amountLimitInToken,
      sharingFee
    };
    const accounts = {
      owner: ownerAddress,
      launchpad: launchpadAddress,
      launchpadPurchase: launchpadPurchaseAddress
    }
    return parser.setLaunchpadPurchase(request, accounts, starshipProgramId);
  }

  static createUserProfileInstruction(
    payerAddress: PublicKey,
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(userAddress, launchpadAddress, starshipProgramId)
    const request: CreateUserProfileRequest = {
      user: userAddress,
    };

    const accounts = {
      payer: payerAddress,
      launchpad: launchpadAddress,
      userProfile: userProfileAddress,
      systemProgram: SystemProgram.programId
    }

    return parser.createUserProfile(request, accounts, starshipProgramId);
  }

  static decodeLaunchpadData(data: Buffer): Launchpad {
    return coder.accounts.decode("Launchpad", data)
  }

  static decodeTimestampData(data: Buffer): TimestampAccount {
    return coder.accounts.decode("TimestampAccount", data)
  }

  static redeemBySolInstruction(
    userAddress: PublicKey,
    launchpadAddress: PublicKey,
    userTokenAddress: PublicKey,
    launchpadTokenAddress: PublicKey,
    feeOwnerAddress: PublicKey,
    appDataAddress: PublicKey,
    amount: number,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(userAddress, launchpadAddress, starshipProgramId)

    const [launchpadSignerAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, starshipProgramId)

    const request: RedeemBySolRequest = {
      amount: new BN(amount),
    };

    const accounts = {
      launchpad: launchpadAddress,
      launchpadSigner: launchpadSignerAddress,
      user: userAddress,
      userProfile: userProfileAddress,
      userTokenAccount: userTokenAddress,
      launchpadTokenAccount: launchpadTokenAddress,
      feeOwner: feeOwnerAddress,
      appData: appDataAddress,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID
    }
    return parser.redeemBySol(request, accounts, starshipProgramId)
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
    appDataAddress: PublicKey,
    amount: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(userAddress, launchpadAddress, starshipProgramId)

    const [launchpadSignerAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, starshipProgramId)

    const request: RedeemByTokenRequest = {
      amount,
    };

    const accounts = {
      launchpad: launchpadAddress,
      launchpadPurchase: launchpadPurchaseAddress,
      launchpadSigner: launchpadSignerAddress,
      user: userAddress,
      userProfile: userProfileAddress,
      userToken0Account: userToken0Address,
      userToken1Account: userToken1Address,
      launchpadToken0Account: launchpadToken0Address,
      launchpadToken1Account: launchpadToken1Address,
      feeOwnerToken0Account: feeOwnerToken0Address,
      appData: appDataAddress,
      tokenProgram: TOKEN_PROGRAM_ID,
    }

    return parser.redeemByToken(request, accounts, starshipProgramId);
  }

  static registerInstruction(
    launchpadAddress: PublicKey,
    userAddress: PublicKey,
    whitelist_id: BN,
    whitelistSignature: Buffer,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [userProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findUserProfileAddress(userAddress, launchpadAddress, starshipProgramId)

    const [whitelistIdAddress,]: [PublicKey, number] = StarshipInstructionService.findWhitelistIdProfileAddress(launchpadAddress, whitelist_id, starshipProgramId)
    const args = {
      whitelistSignature: whitelistSignature
    }

    const accounts = {
      launchpad: launchpadAddress,
      user: userAddress,
      userProfile: userProfileAddress,
      whitelistIdProfile: whitelistIdAddress,
      sysvarProgram: SYSVAR_INSTRUCTIONS_PUBKEY
    }

    return parser.register(
      args,
      accounts,
      starshipProgramId
    )
  }

  static withdrawSolInstruction(
    ownerAddress: PublicKey,
    launchpadAddress: PublicKey,
    amount: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadSignerAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, starshipProgramId)
    const request: WithdrawRequest = {
      amount
    };
    const accounts = {
      owner: ownerAddress,
      launchpad: launchpadAddress,
      launchpadSigner: launchpadSignerAddress,
      systemProgram: SystemProgram.programId
    }

    return parser.withdrawSol(request, accounts, starshipProgramId)
  }

  static withdrawTokenInstruction(
    ownerAddress: PublicKey,
    launchpadAddress: PublicKey,
    from: PublicKey,
    to: PublicKey,
    tokenMint: PublicKey,
    amount: BN,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [launchpadSignerAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, starshipProgramId)
    const request = {
      tokenMint,
      amount
    };

    const accounts = {
      owner: ownerAddress,
      launchpad: launchpadAddress,
      launchpadSigner: launchpadSignerAddress,
      from: from,
      to: to,
      tokenProgram: TOKEN_PROGRAM_ID
    }

    return parser.withdrawToken(request, accounts, starshipProgramId);
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

  static findWhitelistTokenMintAddress(
    tokenMint: PublicKey,
    starshipProgramId: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from([237, 187, 186, 94, 223, 196, 119, 229]), tokenMint.toBuffer()],
      starshipProgramId
    )
  }

  static findWhitelistIdProfileAddress(
    launchpadAddress: PublicKey,
    whitelist_id: BN,
    starshipProgramId: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from([212, 134, 165, 23, 111, 165, 116, 210]), launchpadAddress.toBuffer(), whitelist_id.toBuffer()],
      starshipProgramId
    )
  }

  static HashWhitelistMessage(
    message: WhitelistParams
  ): Buffer {
    const buffer = Buffer.alloc(1000)
    const span = WHITELIST_LAYOUT.encode(message, buffer)
    const serialize = buffer.subarray(0, span)

    return HashService.keckka256(serialize)
  }

  static signMessage = async (signer: Keypair, message: Buffer): Promise<Buffer> => {
    const signature = await ed.sign(message, signer.secretKey.slice(0, 32))

    return Buffer.from(signature)
  }

  static findAdminProfileAddress(
    adminAddress: PublicKey,
    starshipProgramId: PublicKey
  ): [PublicKey,number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from([188, 48, 130, 220, 238, 157, 235, 190]), adminAddress.toBuffer()],
      starshipProgramId
    )
  }

  static findAppDataAddress(
    starshipProgramId: PublicKey
  ): [PublicKey,number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from([100, 255, 255, 110, 22, 96, 128, 193])],
      starshipProgramId
    )
  }

  static setAdminInstruction(
    rootAddress: PublicKey,
    adminAddress: PublicKey,
    starshipProgramId: PublicKey
  ): TransactionInstruction {
    const [adminProfileAddress,]: [PublicKey, number] = StarshipInstructionService.findAdminProfileAddress(adminAddress, starshipProgramId)
    const request = {
      admin: adminAddress,
    };

    const accounts = {
      root: rootAddress,
      adminProfile: adminProfileAddress,
      systemProgram: SystemProgram.programId
    }

    return parser.setAdmin(
      request, accounts, starshipProgramId);
  }
}
