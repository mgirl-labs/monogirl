import { Connection, PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { createHash } from "crypto";
import type { VerificationResult } from "./types";

export class ProofVerifier {
  private connection: Connection;
  private programId: PublicKey;

  constructor(rpcUrl: string, programId: string) {
    this.connection = new Connection(rpcUrl, "confirmed");
    this.programId = new PublicKey(programId);
  }

  async verifyParallelExecution(
    merkleRoot: Buffer,
    transactionHashes: Buffer[]
  ): Promise<VerificationResult> {
    const computedRoot = this.computeMerkleRoot(transactionHashes);
    const isValid = computedRoot.equals(merkleRoot);

    const validationHash = createHash("sha256")
      .update(Buffer.concat(transactionHashes))
      .digest();

    return {
      isValid,
      merkleRoot: computedRoot,
      validationHash,
      transactionCount: transactionHashes.length,
    };
  }

  async verifyOnChain(
    stateAddress: PublicKey
  ): Promise<boolean> {
    const accountInfo = await this.connection.getAccountInfo(stateAddress);
    if (!accountInfo) {
      return false;
    }

    const isFinalized = accountInfo.data[72] === 1;
    return isFinalized;
  }

  verifyMerkleProof(
    root: Buffer,
    leaf: Buffer,
    proof: Buffer[],
    index: number
  ): boolean {
    let computed = leaf;
    let idx = index;

    for (const sibling of proof) {
      const hasher = createHash("sha256");
      if (idx % 2 === 0) {
        hasher.update(Buffer.concat([computed, sibling]));
      } else {
        hasher.update(Buffer.concat([sibling, computed]));
      }
      computed = hasher.digest();
      idx = Math.floor(idx / 2);
    }

    return computed.equals(root);
  }

  private computeMerkleRoot(leaves: Buffer[]): Buffer {
    if (leaves.length === 0) {
      return Buffer.alloc(32);
    }

    let currentLevel = [...leaves];
    while (currentLevel.length > 1) {
      const nextLevel: Buffer[] = [];
      for (let i = 0; i < currentLevel.length; i += 2) {
        const left = currentLevel[i];
        const right =
          i + 1 < currentLevel.length ? currentLevel[i + 1] : currentLevel[i];
        nextLevel.push(
          createHash("sha256")
            .update(Buffer.concat([left, right]))
            .digest()
        );
      }
      currentLevel = nextLevel;
    }

    return currentLevel[0];
  }
}
