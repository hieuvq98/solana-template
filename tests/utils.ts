import { BorshService, HashService, MerkleNode, MerkleTreeKeccak } from "@coin98/solana-support-library"
import * as borsh from '@project-serum/borsh';
import { PublicKey,Keypair } from "@solana/web3.js"
import * as ed from "@noble/ed25519"
export interface WhitelistParams {
  index: number
  address: PublicKey
}

const WHITELIST_LAYOUT = borsh.struct<WhitelistParams>([
  borsh.u32('index'),
  borsh.publicKey('address'),
]);

export function randomString(length: number): string {
    var result           = '';
    var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var charactersLength = characters.length;
    for ( var i = 0; i < length; i++ ) {
      result += characters.charAt(Math.floor(Math.random() *
 charactersLength));
   }
   return result;
}

export class RedemptionTree{
  private redemptionTree: MerkleTreeKeccak

  constructor(redemptions: WhitelistParams[]) {
    const hashes = redemptions.map(item => {
      const bytes = BorshService.serialize(WHITELIST_LAYOUT, item, 40)
      return HashService.keckka256(bytes)
    })
    this.redemptionTree = new MerkleTreeKeccak(hashes)
  }

  getRoot(): MerkleNode {
    return this.redemptionTree.root()
  }

  getProof(index: number): MerkleNode[] {
    const nodes = this.redemptionTree.nodes();
    const proofs = [];
    let currentIndex = index;
    for (let i = 0; i < nodes.length - 1; i++) {
      const proof = currentIndex % 2 == 0
        ? nodes[i][currentIndex + 1]
        : nodes[i][currentIndex - 1];
      currentIndex = (currentIndex - (currentIndex % 2)) / 2;
      proofs.push(proof);
    }

    return proofs;
  }

}


export async function wait(milliSeconds: number) {
  console.log("Waiting:", milliSeconds)
  await new Promise(resolve => setTimeout(resolve, milliSeconds));
}
