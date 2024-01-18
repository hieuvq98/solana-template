import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SolanaConfigService } from "@coin98/solana-support-library/config"
import { StarshipService } from "../services/starship.service"
import BN from "bn.js";
import { randomString, RedemptionTree, WhitelistParams } from "./utils"
import { SystemProgramService, TokenProgramService } from "@coin98/solana-support-library";
import SecretKey from "./default/id.json"


const PROGRAM_ID: PublicKey = new PublicKey("D511gCoGjpKRLJtbsXCMMUuyJjeX3x2qPoJBqqgPNRVC")

describe("Profile Test",() => {
  let connection = new Connection("http://localhost:8899", "confirmed")

  let defaultAccount: Keypair
  const priceInSolN = new BN(1000)
  const priceInSolD = new BN(10)

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

  const redemptiomTree = new RedemptionTree(whitelist)

  const totalLimit =  new BN("1000000000000")
  const saleLimitPerTransaction = new BN(10000)
  const saleLimitPerUser = new BN(100000000000)
  const amountLimitInSol = new BN(1000000000000)
  const currentTime =  Math.floor((new Date()).getTime() / 1000)
  const registerStartTimestamp = new BN(currentTime + 100)
  const registerEndTimestamp = new BN(currentTime + 105)
  const redeemStartTimestamp = new BN(currentTime + 110)
  const redeemEndTimestamp = new BN(currentTime + 120)
  const claimStartTimestamp = new BN(currentTime + 130)
  const maxRegister = new BN(200)

  let launchpadAddress: PublicKey

  before(async () => {
    defaultAccount = await Keypair.fromSecretKey(Uint8Array.from(SecretKey))
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

    const launchpadName = randomString(10)

    launchpadAddress = await StarshipService.createLaunchpad(
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

  })

  it("Create User Profile!", async () => {
    await StarshipService.createUserProfile(
      connection,
      defaultAccount,
      launchpadAddress,
      defaultAccount.publicKey,
      PROGRAM_ID
    )
  })
})
