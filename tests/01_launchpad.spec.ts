import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SolanaConfigService } from "@coin98/solana-support-library/config"
import { StarshipService } from "../services/starship.service"
import BN from "bn.js";
import { randomString, RedemptionTree, WhitelistParams } from "./utils"
import { SystemProgramService, TokenProgramService } from "@coin98/solana-support-library";
import assert from "assert"
import { Launchpad } from "../services/starship_instruction.service";
import SecretKey from "./default/id.json"

const PROGRAM_ID: PublicKey = new PublicKey("D511gCoGjpKRLJtbsXCMMUuyJjeX3x2qPoJBqqgPNRVC")

describe("Launchpad Test", () => {
  let connection = new Connection("http://localhost:8899", "confirmed")
  let defaultAccount: Keypair
  const priceInSolN = new BN(1000)
  const priceInSolD = new BN(1)

  const priceInTokenN = new BN(1000)
  const priceInTokenD = new BN(1)

  const adminAccount: Keypair = Keypair.generate()
  const testAccount1: Keypair = Keypair.generate()
  const testAccount2: Keypair = Keypair.generate()
  const testAccount3: Keypair = Keypair.generate()

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
    <WhitelistParams>{
      index: 2,
      address: testAccount3.publicKey,
    },
  ]

  const totalLimit = new BN("1000000000000")
  const amountLimitInSol = new BN(100000000)
  const amountLimitInToken = new BN(100000000)
  const saleLimitPerTransaction = new BN(10000)
  const saleLimitPerUser = new BN(100000000000)
  const maxRegister = new BN(200)


  before(async () => {
    defaultAccount = await Keypair.fromSecretKey(Uint8Array.from(SecretKey))
    console.log("defaultAccount", defaultAccount.publicKey);
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
    console.log(`Created mints ${token0Mint.publicKey} and ${token1Mint.publicKey}`)

  })

  it("Create Launchpad!", async () => {

    const currentTime = Math.floor((new Date()).valueOf() / 1000)
    const registerStartTimestamp = new BN(currentTime + 2)
    const registerEndTimestamp = new BN(currentTime + 5)
    const redeemStartTimestamp = new BN(currentTime + 6)
    const redeemEndTimestamp = new BN(currentTime + 100)
    const claimStartTimestamp = new BN(currentTime + 105)

    const launchpadName = randomString(30)
    const launchpadAddress = await StarshipService.createLaunchpad(
      connection,
      defaultAccount,
      launchpadName,
      token0Mint.publicKey,
      priceInSolN,
      priceInSolD,
      saleLimitPerTransaction,
      saleLimitPerUser,
      maxRegister,
      totalLimit,
      amountLimitInSol,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
      new BN(1000),
      new BN(10),
      claimStartTimestamp,
      null,
      PROGRAM_ID
    )


    const launchpadInfo: Launchpad = await StarshipService.getLaunchpadAccountInfo(connection, launchpadAddress)
    await StarshipService.printLaunchpadAccountInfo(connection, launchpadAddress)

    assert(launchpadInfo.maxPerUser.toString() == saleLimitPerUser.toString(), "Starship: Invalid max per user")
    assert(launchpadInfo.minPerTx.toString() == saleLimitPerTransaction.toString(), "Starship: Invalid min per transaction")
  })

  it("Create Launchpad Purchase!", async () => {

    const currentTime = Math.floor((new Date()).valueOf() / 1000)
    const registerStartTimestamp = new BN(currentTime + 2)
    const registerEndTimestamp = new BN(currentTime + 5)
    const redeemStartTimestamp = new BN(currentTime + 6)
    const redeemEndTimestamp = new BN(currentTime + 100)
    const claimStartTimestamp = new BN(currentTime + 105)
    const launchpadName = randomString(10)
    const sharingFee = new BN(1000)

    const launchpadAddress = await StarshipService.createLaunchpad(
      connection,
      defaultAccount,
      launchpadName,
      token0Mint.publicKey,
      priceInSolN,
      priceInSolD,
      saleLimitPerTransaction,
      saleLimitPerUser,
      maxRegister,
      totalLimit,
      amountLimitInSol,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
      new BN(2000),
      new BN(10),
      claimStartTimestamp,
      null,
      PROGRAM_ID
    )

    let whitelistTokenMint: PublicKey = await StarshipService.createWhitelistToken(connection, defaultAccount, token0Mint.publicKey, PROGRAM_ID);
    const launchpadPurchaseAddress: PublicKey = await StarshipService.createLaunchpadPurchase(
      connection,
      defaultAccount,
      launchpadAddress,
      whitelistTokenMint,
      token0Mint.publicKey,
      priceInTokenN,
      priceInTokenD,
      saleLimitPerTransaction,
      saleLimitPerUser,
      amountLimitInToken,
      sharingFee,
      PROGRAM_ID
    )
    const launchpadInfo: Launchpad = await StarshipService.getLaunchpadAccountInfo(connection, launchpadAddress)


    assert(launchpadInfo.maxPerUser.toString() == saleLimitPerUser.toString(), "Starship: Invalid max per user")
    assert(launchpadInfo.minPerTx.toString() == saleLimitPerTransaction.toString(), "Starship: Invalid min per transaction")
  })

  it("Transfer ownership", async () => {

    const currentTime =  Math.floor((new Date()).valueOf() / 1000)
    const registerStartTimestamp = new BN(currentTime + 2)
    const registerEndTimestamp = new BN(currentTime + 5)
    const redeemStartTimestamp = new BN(currentTime + 6)
    const redeemEndTimestamp = new BN(currentTime + 100)
    const launchpadName = randomString(10)
    const claimStartTimestamp = new BN(currentTime + 105)


    const launchpadAddress = await StarshipService.createLaunchpad(
      connection,
      defaultAccount,
      launchpadName,
      token0Mint.publicKey,
      priceInSolN,
      priceInSolD,
      saleLimitPerTransaction,
      saleLimitPerUser,
      maxRegister,
      totalLimit,
      amountLimitInSol,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
      new BN(2000),
      new BN(10),
      claimStartTimestamp,
      null,
      PROGRAM_ID
    )

    const newOwner = Keypair.generate()

    await SystemProgramService.transfer(
      connection,
      defaultAccount,
      newOwner.publicKey,
      1000000000
    )

    await StarshipService.transferLaunchpadOwnership(
      connection,
      defaultAccount,
      launchpadAddress,
      newOwner.publicKey,
      PROGRAM_ID
    )

    await StarshipService.acceptLaunchpadOwnership(
      connection,
      newOwner,
      launchpadAddress,
      PROGRAM_ID
    )
  })
})
