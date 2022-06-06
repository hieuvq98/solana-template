import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SolanaConfigService } from "@coin98/solana-support-library/config"
import { StarshipService } from "../services/starship.service"
import { VaultService } from "@coin98/vault-js";
import BN from "bn.js";
import { randomString, RedemptionTree, WhitelistParams, wait } from "./utils"
import { SolanaService, SystemProgramService, TokenProgramService } from "@coin98/solana-support-library";

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

  const isPrivateSale: boolean = true
  const saleLimitPerTransaction = 1
  const saleLimitPerUser = 10

  let vaultAddress: PublicKey
  let vaultSignerAddress: PublicKey

  let vaultToken1Address: PublicKey
  let vaultToken0Address: PublicKey

  let launchpadAddress: PublicKey

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
    
    vaultToken0Address = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      vaultSignerAddress,
      token0Mint.publicKey
    )

    await SystemProgramService.transfer(
      connection,
      defaultAccount,
      vaultSignerAddress,
      1000000000
    )
    
    vaultToken1Address = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      vaultSignerAddress,
      token1Mint.publicKey
    )

    await TokenProgramService.mint(
      connection,
      defaultAccount,
      token1Mint.publicKey,
      vaultToken1Address,
      new BN(10000000)
    )
  }) 

  beforeEach(async () => {
    const launchpadName = randomString(10)

    const currentBlock = await connection.getSlot()
    const currentTime = await connection.getBlockTime(currentBlock)

    const registerStartTimestamp = currentTime + 2
    const registerEndTimestamp = currentTime + 10
    const redeemStartTimestamp = currentTime + 10
    const redeemEndTimestamp = currentTime + 1000


    launchpadAddress = (await StarshipService.findLaunchpadAddress(launchpadName, PROGRAM_ID))[0]

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

    const [launchpadSignerAddress, ]: [PublicKey, number] = await StarshipService.findLaunchpadSignerAddress(launchpadAddress, PROGRAM_ID)

    await VaultService.setVault(
      connection,
      defaultAccount,
      vaultAddress,
      [launchpadSignerAddress],
      VAULT_PROGRAM_ID
    )

    await StarshipService.createLocalProfile(
      connection,
      defaultAccount,
      launchpadAddress,
      testAccount1.publicKey,
      PROGRAM_ID
    )

    const [userGlobalProfileAddress]: [PublicKey, number] = await StarshipService.findUserGlobalProfileAddress(testAccount1.publicKey, PROGRAM_ID)

    if (await SolanaService.isAddressAvailable(connection, userGlobalProfileAddress)) {
      await StarshipService.createGlobalProfile(
        connection,
        defaultAccount,
        testAccount1.publicKey,
        PROGRAM_ID
      )
    }
  })

  it("Register!", async () => {
    await SystemProgramService.transfer(
      connection,
      defaultAccount,
      testAccount1.publicKey,
      1000000000
    )

    const proofs = redemptiomTree.getProof(0)

    await wait(2000)

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

    await wait(2000)
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

    await wait(20000)

    await StarshipService.redeemBySol(
      connection,
      testAccount1,
      launchpadAddress,
      testAccount1Token1Address,
      vaultAddress,
      vaultToken1Address,
      4,
      VAULT_PROGRAM_ID,
      PROGRAM_ID
    )
  })
})
