// import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
// import { SolanaConfigService } from "@coin98/solana-support-library/config"
// import { StarshipService } from "../services/starship.service"
// import BN from "bn.js";
// import { randomString, RedemptionTree, WhitelistParams, wait } from "./utils"
// import { SolanaService, SystemProgramService, TokenProgramService } from "@coin98/solana-support-library";
// import { StarshipInstructionService } from "../services/starship_instruction.service";
// import SecretKey from "./default/id.json"

// const PROGRAM_ID: PublicKey = new PublicKey("D511gCoGjpKRLJtbsXCMMUuyJjeX3x2qPoJBqqgPNRVC")

// const FEE_OWNER: PublicKey = new PublicKey("1RbBCDnDt7NkrjndCnwjMuJ9vJbx81pKT1ts1x8SeQq")

// describe("Profile Test", () => {
//   let connection = new Connection("http://localhost:8899", "confirmed")

//   let defaultAccount: Keypair
//   const priceInSolN = new BN(1000)
//   const priceInSolD = new BN(1)

//   const priceInTokenN = new BN(1000)
//   const priceInTokenD = new BN(1)

//   const testAccount1: Keypair = Keypair.generate()
//   const testAccount2: Keypair = Keypair.generate()
//   const adminAccount: Keypair = Keypair.generate()

//   const token0Mint: Keypair = Keypair.generate()
//   const token1Mint: Keypair = Keypair.generate()


//   const whitelist = [
//     <WhitelistParams>{
//       index: 0,
//       address: testAccount1.publicKey,
//     },
//     <WhitelistParams>{
//       index: 1,
//       address: testAccount2.publicKey,
//     },
//   ]

//   const redemptiomTree = new RedemptionTree(whitelist)

//   const limitSale = new BN("1000000000000")
//   const saleLimitPerTransaction = new BN(10000)
//   const saleLimitPerUser = new BN(100000000000)
//   const maxRegister = new BN(200)

//   let launchpadAddress: PublicKey
//   let launchpadPurchaseAddress: PublicKey
//   let launchpadToken0Address: PublicKey
//   let launchpadToken1Address: PublicKey
//   let whitelistToken0 : PublicKey
//   let whitelistToken1 : PublicKey
//   let appDataAddress : PublicKey

//   before(async () => {
//     defaultAccount = await Keypair.fromSecretKey(Uint8Array.from(SecretKey))

//     await TokenProgramService.createTokenMint(
//       connection,
//       defaultAccount,
//       token0Mint,
//       6,
//       defaultAccount.publicKey,
//       null
//     )

//     await TokenProgramService.createTokenMint(
//       connection,
//       defaultAccount,
//       token1Mint,
//       6,
//       defaultAccount.publicKey,
//       null
//     )
//   })

//   beforeEach(async () => {
//     const currentTime = Math.floor((new Date()).valueOf() / 1000)
//     const launchpadName = randomString(10)
//     const registerStartTimestamp = new BN(currentTime + 2)
//     const registerEndTimestamp = new BN(currentTime + 5)
//     const redeemStartTimestamp = new BN(currentTime + 6)
//     const redeemEndTimestamp = new BN(currentTime + 100)
//     const claimStartTimestamp = new BN(currentTime + 105)
//     const totalLimit = new BN("1000000000000")
//     const amountLimitInSol = new BN(1000000000000)

//     const amountLimitInToken = new BN(100000000)

//     await StarshipService.setAdmin(connection,defaultAccount,adminAccount.publicKey,PROGRAM_ID);

//     await SystemProgramService.transfer(
//       connection,
//       defaultAccount,
//       adminAccount.publicKey,
//       1000000000
//     )

//     launchpadAddress = await StarshipService.createLaunchpad(
//       connection,
//       adminAccount,
//       defaultAccount,
//       launchpadName,
//       token1Mint.publicKey,
//       priceInSolN,
//       priceInSolD,
//       saleLimitPerTransaction,
//       saleLimitPerUser,
//       maxRegister,
//       totalLimit,
//       amountLimitInSol,
//       registerStartTimestamp,
//       registerEndTimestamp,
//       redeemStartTimestamp,
//       redeemEndTimestamp,
//       new BN(2000),
//       new BN(10),
//       claimStartTimestamp,
//       testAccount2.publicKey,
//       PROGRAM_ID
//     )
//     const [whitelistTokenAddress, ]: [PublicKey, number] = StarshipInstructionService.findWhitelistTokenMintAddress(token0Mint.publicKey,PROGRAM_ID);

//     if (!await SolanaService.isAddressInUse(connection,whitelistTokenAddress)) {
//       whitelistToken0 = await StarshipService.createWhitelistToken(
//         connection,
//         adminAccount,
//         token0Mint.publicKey,
//         PROGRAM_ID
//       )
//     };

//     launchpadPurchaseAddress = await StarshipService.createLaunchpadPurchase(
//       connection,
//       defaultAccount,
//       launchpadAddress,
//       whitelistToken0,
//       token0Mint.publicKey,
//       priceInTokenN,
//       priceInTokenD,
//       saleLimitPerTransaction,
//       saleLimitPerUser,
//       amountLimitInToken,
//       new BN(10),
//       PROGRAM_ID
//     )

//     await StarshipService.printLaunchpadAccountInfo(connection, launchpadAddress)
//     const [launchpadSignerAddress,]: [PublicKey, number] = StarshipInstructionService.findLaunchpadSignerAddress(launchpadAddress, PROGRAM_ID)

//     launchpadToken0Address = await TokenProgramService.createAssociatedTokenAccount(
//       connection,
//       defaultAccount,
//       launchpadSignerAddress,
//       token0Mint.publicKey,
//     )
//     launchpadToken1Address = await TokenProgramService.createAssociatedTokenAccount(
//       connection,
//       defaultAccount,
//       launchpadSignerAddress,
//       token1Mint.publicKey,
//     )

//     await TokenProgramService.mint(
//       connection,
//       defaultAccount,
//       defaultAccount,
//       token1Mint.publicKey,
//       launchpadToken1Address,
//       new BN("1000000000000")
//     )

//     appDataAddress = await StarshipService.createApp(connection,defaultAccount,FEE_OWNER,PROGRAM_ID);
//   })

//   it("Register!", async () => {
//     await SystemProgramService.transfer(
//       connection,
//       defaultAccount,
//       testAccount1.publicKey,
//       1000000000
//     )

//     await SystemProgramService.transfer(
//       connection,
//       defaultAccount,
//       testAccount2.publicKey,
//       1000000000
//     )

//     await StarshipService.printLaunchpadAccountInfo(connection, launchpadAddress)

//     await StarshipService.register(
//       connection,
//       testAccount2,
//       testAccount1,
//       launchpadAddress,
//       new BN(190),
//       PROGRAM_ID
//     )
//   })

//   it("Redeem With Sol!", async () => {
//     await SystemProgramService.transfer(
//       connection,
//       defaultAccount,
//       testAccount1.publicKey,
//       1000000000
//     )

//     await SystemProgramService.transfer(
//       connection,
//       defaultAccount,
//       testAccount2.publicKey,
//       1000000000
//     )
//     await wait(2000)

//     await StarshipService.register(
//       connection,
//       testAccount2,
//       testAccount1,
//       launchpadAddress,
//       new BN(101),
//       PROGRAM_ID
//     )

//     const testAccount1Token1Address: PublicKey = await TokenProgramService.createAssociatedTokenAccount(
//       connection,
//       defaultAccount,
//       testAccount1.publicKey,
//       token1Mint.publicKey,
//     )
//     await wait(10000)

//     await StarshipService.redeemBySol(
//       connection,
//       testAccount1,
//       launchpadAddress,
//       testAccount1Token1Address,
//       launchpadToken1Address,
//       FEE_OWNER,
//       appDataAddress,
//       100000,
//       PROGRAM_ID
//     )

//     await StarshipService.withdrawSol(
//       connection,
//       defaultAccount,
//       launchpadAddress,
//       new BN(1),
//       PROGRAM_ID
//     )
//   })

//   it("Redeem With Token!", async () => {
//     await SystemProgramService.transfer(
//       connection,
//       defaultAccount,
//       testAccount1.publicKey,
//       1000000000
//     )

//     await SystemProgramService.transfer(
//       connection,
//       defaultAccount,
//       testAccount2.publicKey,
//       1000000000
//     )

//     await wait(2000)

//     await StarshipService.register(
//       connection,
//       testAccount2,
//       testAccount1,
//       launchpadAddress,
//       new BN(101),
//       PROGRAM_ID
//     )

//     const testAccount1Token0Address: PublicKey = await TokenProgramService.createAssociatedTokenAccount(
//       connection,
//       defaultAccount,
//       testAccount1.publicKey,
//       token0Mint.publicKey,
//     )

//     const feeOwnerToken0Address: PublicKey = await TokenProgramService.createAssociatedTokenAccount(
//       connection,
//       defaultAccount,
//       FEE_OWNER,
//       token0Mint.publicKey,
//     )

//     await TokenProgramService.mint(
//       connection,
//       defaultAccount,
//       defaultAccount,
//       token0Mint.publicKey,
//       testAccount1.publicKey,
//       new BN(10000000000)
//     )

//     const testAccount1Token1Address: PublicKey = await TokenProgramService.createAssociatedTokenAccount(
//       connection,
//       defaultAccount,
//       testAccount1.publicKey,
//       token1Mint.publicKey,
//     )

//     await wait(10000)

//     await StarshipService.redeemByToken(
//       connection,
//       testAccount1,
//       launchpadAddress,
//       launchpadPurchaseAddress,
//       testAccount1Token0Address,
//       testAccount1Token1Address,
//       launchpadToken0Address,
//       launchpadToken1Address,
//       feeOwnerToken0Address,
//       appDataAddress,
//       new BN(1000000),
//       PROGRAM_ID
//     )

//     await StarshipService.withdrawToken(
//       connection,
//       defaultAccount,
//       launchpadAddress,
//       launchpadToken1Address,
//       testAccount1Token1Address,
//       token1Mint.publicKey,
//       new BN(1),
//       PROGRAM_ID
//     )
//   })
// })
