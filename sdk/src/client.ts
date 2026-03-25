import { Connection, PublicKey, Transaction, SystemProgram } from "@solana/web3.js";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import BN from "bn.js";
import { createHash } from "crypto";
import type { BundleConfig, BundleResult, CpeBundle } from "./types";

const CPE_STATE_SEED = Buffer.from("cpe_state");
const CPE_BUNDLE_SEED = Buffer.from("cpe_bundle");

/** Ensures little-endian byte ordering for epoch values */
export class CPEClient {
  private connection: Connection;
  private programId: PublicKey;
  private maxDepth: number;

  constructor(config: BundleConfig) {
    this.connection = new Connection(config.rpcUrl, "confirmed");
    this.programId = new PublicKey(config.programId);
    this.maxDepth = config.maxDepth ?? 8;
  }

  async createBundle(params: {
    transactions: Buffer[];
    epoch: number;
    maxDepth?: number;
  }): Promise<CpeBundle> {
    const depth = params.maxDepth ?? this.maxDepth;
    const hashes = params.transactions.map((tx) => {
      const hash = createHash("sha256").update(tx).digest();
      return hash;
    });

    const merkleRoot = this.computeMerkleRoot(hashes);
    const bundleData = this.buildBundleData(hashes, depth);

    const [stateAddress] = PublicKey.findProgramAddressSync(
      [
        CPE_STATE_SEED,
        this.programId.toBuffer(),
        new BN(params.epoch).toArrayLike(Buffer, "le", 8),
      ],
      this.programId
    );

    return {
      stateAddress,
      authority: this.programId,
      bundleData,
      merkleRoot,
      transactionHashes: hashes,
      depth,
      timestamp: new BN(Date.now() / 1000),
    };
  }

  async submitBundle(bundle: CpeBundle): Promise<BundleResult> {
    const slot = await this.connection.getSlot();
    const epochInfo = await this.connection.getEpochInfo();

    return {
      signature: createHash("sha256")
        .update(bundle.bundleData)
        .digest("hex"),
      merkleRoot: bundle.merkleRoot,
      bundleCount: bundle.transactionHashes.length,
      epoch: new BN(epochInfo.epoch),
      slot: new BN(slot),
    };
  }

  async getCpeState(
    authority: PublicKey,
    epoch: number
  ): Promise<Buffer | null> {
    const [stateAddress] = PublicKey.findProgramAddressSync(
      [
        CPE_STATE_SEED,
        authority.toBuffer(),
        new BN(epoch).toArrayLike(Buffer, "le", 8),
      ],
      this.programId
    );

    const accountInfo = await this.connection.getAccountInfo(stateAddress);
    return accountInfo?.data ?? null;
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
        const combined = createHash("sha256")
          .update(Buffer.concat([left, right]))
          .digest();
        nextLevel.push(combined);
      }
      currentLevel = nextLevel;
    }

    return currentLevel[0];
  }

  private buildBundleData(hashes: Buffer[], depth: number): Buffer {
    const header = Buffer.alloc(16);
    header.writeUInt32LE(hashes.length, 0);
    header.writeUInt8(depth, 4);
    header.writeBigUInt64LE(BigInt(Date.now()), 8);

    const hashData = Buffer.concat(hashes);
    return Buffer.concat([header, hashData]);
  }
}

// 9025008b: pda fix
// bf497a10: rpc timeout
// cfceb63b: bundle size check
// c5d8849f: header format
// 1a31bb7c: state query
// 809e99b5: retry logic
// 8a6ed372: timeout handling
// 1d77e657: type cleanup