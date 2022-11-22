import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SolanaConfigService } from "@coin98/solana-support-library/config"
import { StarshipService } from "../services/starship.service"
import BN, { max } from "bn.js";
import { randomString, RedemptionTree, WhitelistParams } from "./utils"
import { TokenProgramService } from "@coin98/solana-support-library";
import assert from "assert"
import { Launchpad, StarshipInstructionService } from "../services/starship_instruction.service";

const PROGRAM_ID: PublicKey = new PublicKey("FaJtq6SLQNwGgaggr7izJMgRYkxU1xwtCjnyESSXhvHG")

describe("Launchpad Test",() => {
  let connection = new Connection("http://localhost:8899", "confirmed")

  let defaultAccount: Keypair
  const priceInSolN = new BN(1000)
  const priceInSolD = new BN(1)

  const priceInTokenN = new BN(1000)
  const priceInTokenD = new BN(1)
  
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

  const redemptiomTree = new RedemptionTree(whitelist)

  const limitSale =  new BN("1000000000000")
  const saleLimitPerTransaction = new BN(10000)
  const saleLimitPerUser = new BN(100000000000)
  const currentTime =  Math.floor((new Date()).valueOf() / 1000)
  const registerStartTimestamp = new BN(currentTime + 2)
  const registerEndTimestamp = new BN(currentTime + 5)
  const redeemStartTimestamp = new BN(currentTime + 6)
  const redeemEndTimestamp = new BN(currentTime + 100)

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

  it("Create Launchpad!", async() => {
    const launchpadName = randomString(10)

    const launchpadAddress = await StarshipService.createLaunchpad(
      connection,
      defaultAccount,
      defaultAccount,
      launchpadName,
      token0Mint.publicKey,
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
      PROGRAM_ID
    )
    const launchpadInfo:  Launchpad = await StarshipService.getLaunchpadAccountInfo(connection, launchpadAddress)

    await StarshipService.printLaunchpadAccountInfo(connection, launchpadAddress)

    assert(launchpadInfo.maxPerUser.toString() == saleLimitPerUser.toString(), "Starship: Invalid max per user")
    assert(launchpadInfo.minPerTx.toString() == saleLimitPerTransaction.toString(), "Starship: Invalid min per transaction")
  })

  it("Create Launchpad Purchase!", async() => {
    const launchpadName = randomString(10)

    const launchpadAddress = await StarshipService.createLaunchpad(
      connection,
      defaultAccount,
      defaultAccount,
      launchpadName,
      token0Mint.publicKey,
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
      PROGRAM_ID
    )

    const launchpadPurchaseAddress: PublicKey = await StarshipService.createLaunchpadPurchase(
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
    const launchpadInfo:  Launchpad = await StarshipService.getLaunchpadAccountInfo(connection, launchpadAddress)


    assert(launchpadInfo.maxPerUser.toString() == saleLimitPerUser.toString(), "Starship: Invalid max per user")
    assert(launchpadInfo.minPerTx.toString() == saleLimitPerTransaction.toString(), "Starship: Invalid min per transaction")
  })
})
