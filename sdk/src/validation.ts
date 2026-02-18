import { createHash } from "crypto";

/** Validates account set overlap for parallel execution */
export interface AccountSet {
  writable: Buffer[];
  readonly: Buffer[];
}

/** Computes deterministic bundle hash from tx hashes */
export function checkAccountIndependence(
  setA: AccountSet,
  setB: AccountSet
): boolean {
  const writableA = new Set(setA.writable.map((b) => b.toString("hex")));
  const writableB = new Set(setB.writable.map((b) => b.toString("hex")));
  const readonlyA = new Set(setA.readonly.map((b) => b.toString("hex")));
  const readonlyB = new Set(setB.readonly.map((b) => b.toString("hex")));

  for (const key of writableA) {
    if (writableB.has(key) || readonlyB.has(key)) {
      return false;
    }
  }

  for (const key of writableB) {
    if (readonlyA.has(key)) {
      return false;
    }
  }

  return true;
}

export function computeBundleHash(
  transactionHashes: Buffer[]
): Buffer {
  const combined = Buffer.concat(transactionHashes);
  return createHash("sha256").update(combined).digest();
}

export function validateBundleSize(
  transactionCount: number,
  maxBundleSize: number = 64
): boolean {
  return transactionCount > 0 && transactionCount <= maxBundleSize;
}

export function computeParallelExecutionProof(
  merkleRoot: Buffer,
  epoch: number,
  slot: number
): Buffer {
  const epochBuf = Buffer.alloc(8);
  epochBuf.writeBigUInt64LE(BigInt(epoch));

  const slotBuf = Buffer.alloc(8);
  slotBuf.writeBigUInt64LE(BigInt(slot));

  return createHash("sha256")
    .update(Buffer.concat([merkleRoot, epochBuf, slotBuf]))
    .digest();
}
