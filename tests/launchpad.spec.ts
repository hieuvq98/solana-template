import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SolanaConfigService } from "@coin98/solana-support-library/config"
import { StarshipService } from "../services/starship.service"

const PROGRAM: PublicKey = new PublicKey("Cyv1nUa7si2dds8KvoNcjyC7ey7dhsgv3cpmrTJHcyHv")

describe("Launchpad Test", () => {
  let connection = new Connection("http://localhost:8899", "confirmed")
  let defaultAccount: Keypair

  before(async () => {
    defaultAccount = await SolanaConfigService.getDefaultAccount()
  })

  it("Create Launchpad!", async() => {
  })

})
