import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SolanaConfigService } from "@coin98/solana-support-library/config"
import { StarshipService } from "../services/starship.service"
import BN from "bn.js";
import { randomString, RedemptionTree, WhitelistParams, wait } from "./utils"
import { SolanaService, SystemProgramService, TokenProgramService } from "@coin98/solana-support-library";
import { StarshipInstructionService } from "../services/starship_instruction.service";

const PROGRAM_ID: PublicKey = new PublicKey("ASMck7GjbLUkmsesypj4mA9s3ye311AqfAk7tFjHmaSh")

const FEE_OWNER: PublicKey = new PublicKey("GnzQDYm2gvwZ8wRVmuwVAeHx5T44ovC735vDgSNhumzQ")

describe("Profile Test",() => {
  let connection = new Connection("http://localhost:8899", "confirmed")

  let defaultAccount: Keypair
  const priceInSolN = new BN(1000)
  const priceInSolD = new BN(1)

  const priceInTokenN = new BN(1000)
  const priceInTokenD = new BN(1)
  
  const testAccount1: Keypair = Keypair.generate()
  const testAccount2: Keypair = Keypair.generate()

  const token0Mint: Keypair = Keypair.generate()
  const token1Mint: Keypair = Keypair.generate()

  const whitelist = [
    <WhitelistParams>{
      index: 0,
      address: testAccount1.publicKey,
    },
    <WhitelistParams>{
      index: 1,
      address: testAccount2.publicKey,
    },
  ]

  const redemptiomTree = new RedemptionTree(whitelist)

  const limitSale =  new BN("1000000000000")
  const saleLimitPerTransaction = new BN(10000)
  const saleLimitPerUser = new BN(100000000000)
  const currentTime =  Math.floor((new Date()).valueOf() / 1000)

  let launchpadAddress: PublicKey
  let launchpadPurchaseAddress: PublicKey
  let launchpadToken0Address: PublicKey
  let launchpadToken1Address: PublicKey

  before(async () => {
    defaultAccount = await SolanaConfigService.getDefaultAccount()

    await TokenProgramService.createTokenMint(
      connection,
      defaultAccount,
      token0Mint,
      2,
      defaultAccount.publicKey,
      null
    )

    await TokenProgramService.createTokenMint(
      connection,
      defaultAccount,
      token1Mint,
      2,
      defaultAccount.publicKey,
      null
    )
  }) 

  beforeEach(async () => {
    const launchpadName = randomString(10)
    const registerStartTimestamp = new BN(currentTime + 10)
    const registerEndTimestamp = new BN(currentTime + 20)
    const redeemStartTimestamp = new BN(currentTime + 21)
    const redeemEndTimestamp = new BN(currentTime + 100)
    launchpadAddress = await StarshipService.createLaunchpad(
      connection,
      defaultAccount,
      defaultAccount,
      launchpadName,
      token1Mint.publicKey,
      priceInSolN,
      priceInSolD,
      saleLimitPerTransaction,
      saleLimitPerUser,
      limitSale,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
      redemptiomTree.getRoot().hash,
      new BN(1000),
      new BN(10),
      PROGRAM_ID
    )
    launchpadPurchaseAddress = await StarshipService.createLaunchpadPurchase(
      connection,
      defaultAccount,
      launchpadAddress,
      token0Mint.publicKey,
      priceInTokenN,
      priceInTokenD,
      saleLimitPerTransaction,
      saleLimitPerUser,
      limitSale,
      PROGRAM_ID
    )

    await StarshipService.printLaunchpadAccountInfo(connection, launchpadAddress)
    const [launchpadSignerAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, PROGRAM_ID)

    launchpadToken0Address = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      launchpadSignerAddress,
      token0Mint.publicKey,
    )
    launchpadToken1Address = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      launchpadSignerAddress,
      token1Mint.publicKey,
    )

    await TokenProgramService.mint(
      connection,
      defaultAccount,
      token1Mint.publicKey,
      launchpadToken1Address,
      new BN("1000000000000")
    )
  })

  it("Register!", async () => {
    await SystemProgramService.transfer(
      connection,
      defaultAccount,
      testAccount1.publicKey,
      1000000000
    )

    const proofs = redemptiomTree.getProof(0)

    await StarshipService.register(
      connection,
      testAccount1,
      0,
      proofs.map(item => item.hash),
      launchpadAddress,
      PROGRAM_ID
    )
  })

  it("Redeem With Sol!", async () => {
    await SystemProgramService.transfer(
      connection,
      defaultAccount,
      testAccount1.publicKey,
      1000000000
    )

    const proofs = redemptiomTree.getProof(0)

    await StarshipService.register(
      connection,
      testAccount1,
      0,
      proofs.map(item => item.hash),
      launchpadAddress,
      PROGRAM_ID
    )

    const testAccount1Token1Address: PublicKey = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      testAccount1.publicKey,
      token1Mint.publicKey,
    )

    await StarshipService.redeemBySol(
      connection,
      testAccount1,
      launchpadAddress,
      testAccount1Token1Address,
      launchpadToken1Address,
      FEE_OWNER,
      100000,
      PROGRAM_ID
    )

    await StarshipService.withdrawSol(
      connection,
      defaultAccount,
      launchpadAddress,
      new BN(1),
      PROGRAM_ID
    )
  })

  it("Redeem With Token!", async () => {
    await SystemProgramService.transfer(
      connection,
      defaultAccount,
      testAccount1.publicKey,
      1000000000
    )

    const proofs = redemptiomTree.getProof(0)

    await StarshipService.register(
      connection,
      testAccount1,
      0,
      proofs.map(item => item.hash),
      launchpadAddress,
      PROGRAM_ID
    )

    const testAccount1Token0Address: PublicKey = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      testAccount1.publicKey,
      token0Mint.publicKey,
    )

    const feeOwnerToken0Address: PublicKey = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      FEE_OWNER,
      token0Mint.publicKey,
    )

    await TokenProgramService.mint(
      connection,
      defaultAccount,
      token0Mint.publicKey,
      testAccount1.publicKey,
      new BN(10000)
    )

    const testAccount1Token1Address: PublicKey = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      testAccount1.publicKey,
      token1Mint.publicKey,
    )

    await StarshipService.redeemByToken(
      connection,
      testAccount1,
      launchpadAddress,
      launchpadPurchaseAddress,
      testAccount1Token0Address,
      testAccount1Token1Address,
      launchpadToken0Address,
      launchpadToken1Address,
      feeOwnerToken0Address,
      new BN(10000),
      PROGRAM_ID
    )

    await StarshipService.withdrawToken(
      connection,
      defaultAccount,
      launchpadAddress,
      launchpadToken1Address,
      testAccount1Token1Address,
      new BN(1),
      PROGRAM_ID
    )
  })
})
