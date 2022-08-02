import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SolanaConfigService } from "@coin98/solana-support-library/config"
import { StarshipService } from "../services/starship.service"
import { VaultService } from "@coin98/vault-js";
import BN from "bn.js";
import { randomString, RedemptionTree, WhitelistParams } from "./utils"
import { TokenProgramService } from "@coin98/solana-support-library";
import { StarshipInstructionService } from "../services/starship_instruction.service";

const PROGRAM_ID: PublicKey = new PublicKey("Cyv1nUa7si2dds8KvoNcjyC7ey7dhsgv3cpmrTJHcyHv")

const VAULT_PROGRAM_ID: PublicKey = new PublicKey("5WxdfYhjwLxL5aJb2J5EC8JXjxk6La5zmFaXq1eSS5UY")

describe("Profile Test",() => {
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
  const registerStartTimestamp = new BN(currentTime)
  const registerEndTimestamp = new BN(currentTime + 5)
  const redeemStartTimestamp = new BN(currentTime + 5)
  const redeemEndTimestamp = new BN(currentTime + 10)

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

  it("Create Global Profile!", async () => {
    const userKey: Keypair = Keypair.generate()

    await StarshipService.createGlobalProfile(
      connection,
      defaultAccount,
      userKey.publicKey,
      PROGRAM_ID
    )
  })

  it("Create Local Profile!", async () => {
    const launchpadName = randomString(10)

    await StarshipService.createLaunchpad(
      connection,
      defaultAccount,
      defaultAccount,
      launchpadName,
      token0Mint.publicKey,
      priceInSolN,
      priceInSolD,
      limitSale,
      saleLimitPerTransaction,
      saleLimitPerUser,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
      redemptiomTree.getRoot().hash,
      PROGRAM_ID
    )

    const [launchpadAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadAddress(launchpadName, PROGRAM_ID)
    await StarshipService.createLocalProfile(
      connection,
      defaultAccount,
      launchpadAddress,
      defaultAccount.publicKey,
      PROGRAM_ID
    )
  })
})
