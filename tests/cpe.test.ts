import { createHash } from "crypto";

function computeMerkleRoot(leaves: Buffer[]): Buffer {
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

function verifyMerkleProof(
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

function checkAccountIndependence(
  setA: Buffer[],
  setB: Buffer[]
): boolean {
  const hexA = new Set(setA.map((b) => b.toString("hex")));
  for (const b of setB) {
    if (hexA.has(b.toString("hex"))) {
      return false;
    }
  }
  return true;
}

describe("Merkle Tree", () => {
  it("should compute root for single leaf", () => {
    const leaf = createHash("sha256").update("tx1").digest();
    const root = computeMerkleRoot([leaf]);
    expect(root).not.toEqual(Buffer.alloc(32));
  });

  it("should compute deterministic root", () => {
    const leaves = [
      createHash("sha256").update("tx1").digest(),
      createHash("sha256").update("tx2").digest(),
      createHash("sha256").update("tx3").digest(),
      createHash("sha256").update("tx4").digest(),
    ];
    const root1 = computeMerkleRoot(leaves);
    const root2 = computeMerkleRoot(leaves);
    expect(root1.equals(root2)).toBe(true);
  });

  it("should return zero buffer for empty leaves", () => {
    const root = computeMerkleRoot([]);
    expect(root).toEqual(Buffer.alloc(32));
  });

  it("should produce different roots for different inputs", () => {
    const leaves1 = [createHash("sha256").update("a").digest()];
    const leaves2 = [createHash("sha256").update("b").digest()];
    const root1 = computeMerkleRoot(leaves1);
    const root2 = computeMerkleRoot(leaves2);
    expect(root1.equals(root2)).toBe(false);
  });

  it("should handle power-of-two leaf count", () => {
    const leaves = Array.from({ length: 8 }, (_, i) =>
      createHash("sha256")
        .update("tx" + i)
        .digest()
    );
    const root = computeMerkleRoot(leaves);
    expect(root.length).toBe(32);
  });

  it("should handle non-power-of-two leaf count", () => {
    const leaves = Array.from({ length: 5 }, (_, i) =>
      createHash("sha256")
        .update("tx" + i)
        .digest()
    );
    const root = computeMerkleRoot(leaves);
    expect(root.length).toBe(32);
  });
});

describe("Parallel Execution Proof Verification", () => {
  it("should verify a valid proof", () => {
    const leaves = [
      createHash("sha256").update("tx0").digest(),
      createHash("sha256").update("tx1").digest(),
    ];
    const root = computeMerkleRoot(leaves);

    const proof = [leaves[1]];
    const isValid = verifyMerkleProof(root, leaves[0], proof, 0);
    expect(isValid).toBe(true);
  });

  it("should reject an invalid proof", () => {
    const leaves = [
      createHash("sha256").update("tx0").digest(),
      createHash("sha256").update("tx1").digest(),
    ];
    const root = computeMerkleRoot(leaves);
    const fakeProof = [createHash("sha256").update("fake").digest()];
    const isValid = verifyMerkleProof(root, leaves[0], fakeProof, 0);
    expect(isValid).toBe(false);
  });
});

describe("Account Set Independence", () => {
  it("should detect independent account sets", () => {
    const setA = [
      createHash("sha256").update("account1").digest(),
      createHash("sha256").update("account2").digest(),
    ];
    const setB = [
      createHash("sha256").update("account3").digest(),
      createHash("sha256").update("account4").digest(),
    ];
    expect(checkAccountIndependence(setA, setB)).toBe(true);
  });

  it("should detect conflicting account sets", () => {
    const shared = createHash("sha256").update("shared_account").digest();
    const setA = [
      createHash("sha256").update("account1").digest(),
      shared,
    ];
    const setB = [
      shared,
      createHash("sha256").update("account3").digest(),
    ];
    expect(checkAccountIndependence(setA, setB)).toBe(false);
  });
});

describe("CPE Bundle Data", () => {
  it("should generate valid bundle data buffer", () => {
    const txCount = 10;
    const hashes = Array.from({ length: txCount }, (_, i) =>
      createHash("sha256")
        .update("concurrent_tx_" + i)
        .digest()
    );

    const header = Buffer.alloc(16);
    header.writeUInt32LE(txCount, 0);
    header.writeUInt8(8, 4);
    header.writeBigUInt64LE(BigInt(Date.now()), 8);

    const bundleData = Buffer.concat([header, ...hashes]);
    expect(bundleData.length).toBe(16 + txCount * 32);

    const readCount = bundleData.readUInt32LE(0);
    expect(readCount).toBe(txCount);
  });

  it("should produce unique hashes per transaction", () => {
    const hashes = Array.from({ length: 100 }, (_, i) =>
      createHash("sha256")
        .update("tx_" + i)
        .digest()
        .toString("hex")
    );
    const unique = new Set(hashes);
    expect(unique.size).toBe(100);
  });
});

// 4db8a4e6: empty tree test
// 1bd90d05: conflict test
// 73bd8946: group count test
// 010fa1e6: acct analysis test
// 4f80770b: e2e test
// ebbcd5ba: overlap test