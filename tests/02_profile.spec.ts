import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SolanaConfigService } from "@coin98/solana-support-library/config"
import { StarshipService } from "../services/starship.service"
import { VaultService } from "@coin98/vault-js";
import BN from "bn.js";
import { randomString, RedemptionTree, WhitelistParams } from "./utils"
import { TokenProgramService } from "@coin98/solana-support-library";

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

  const isPrivateSale: boolean = true
  const saleLimitPerTransaction = 10
  const saleLimitPerUser = 10
  const currentTime =  Math.floor((new Date()).valueOf() / 1000)
  const registerStartTimestamp = currentTime
  const registerEndTimestamp = currentTime + 5
  const redeemStartTimestamp = currentTime + 5
  const redeemEndTimestamp = currentTime + 10

  let vaultAddress: PublicKey
  let vaultSignerAddress: PublicKey

  before(async () => {
    defaultAccount = await SolanaConfigService.getDefaultAccount()
    const vaultName = randomString(10)
    
    await VaultService.createVault(
      connection,
      defaultAccount,
      vaultName,
      VAULT_PROGRAM_ID
    )

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

    vaultAddress = (await VaultService.findVaultAddress(vaultName, VAULT_PROGRAM_ID))[0]
    vaultSignerAddress = (await VaultService.findVaultSignerAddress(vaultAddress, VAULT_PROGRAM_ID))[0]

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

    const vaultToken0Address: PublicKey = await TokenProgramService.findAssociatedTokenAddress(
      vaultSignerAddress,
      token0Mint.publicKey
    )

    const vaultToken1Address: PublicKey = await TokenProgramService.findAssociatedTokenAddress(
      vaultSignerAddress,
      token1Mint.publicKey
    )

    const [launchpadAddress,]: [PublicKey, number] = await StarshipService.findLaunchpadAddress(launchpadName, PROGRAM_ID)
    await StarshipService.createLaunchpad(
      connection,
      defaultAccount,
      defaultAccount,
      launchpadName,
      priceInSolN,
      priceInSolD,
      priceInTokenN,
      priceInTokenD,
      token0Mint.publicKey,
      token1Mint.publicKey,
      VAULT_PROGRAM_ID,
      vaultAddress,
      vaultSignerAddress,
      vaultToken0Address,
      vaultToken1Address,
      isPrivateSale,
      redemptiomTree.getRoot().hash,
      saleLimitPerTransaction,
      saleLimitPerUser,
      registerStartTimestamp,
      registerEndTimestamp,
      redeemStartTimestamp,
      redeemEndTimestamp,
      PROGRAM_ID
    )

    await StarshipService.createLocalProfile(
      connection,
      defaultAccount,
      launchpadAddress,
      defaultAccount.publicKey,
      PROGRAM_ID
    )
  })
})
