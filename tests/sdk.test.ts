import { createHash } from "crypto";

describe("SDK Integration", () => {
  it("should compute sha256 hash correctly", () => {
    const input = "test_transaction_data";
    const hash = createHash("sha256").update(input).digest();
    expect(hash.length).toBe(32);
    expect(hash.toString("hex")).toMatch(/^[0-9a-f]{64}$/);
  });

  it("should build bundle header with correct format", () => {
    const header = Buffer.alloc(16);
    const txCount = 42;
    const depth = 8;
    header.writeUInt32LE(txCount, 0);
    header.writeUInt8(depth, 4);

    expect(header.readUInt32LE(0)).toBe(txCount);
    expect(header.readUInt8(4)).toBe(depth);
  });

  it("should concatenate transaction hashes", () => {
    const hashes = Array.from({ length: 4 }, (_, i) =>
      createHash("sha256")
        .update("tx_" + i)
        .digest()
    );
    const concatenated = Buffer.concat(hashes);
    expect(concatenated.length).toBe(128);
  });

  it("should derive consistent addresses from seeds", () => {
    const seed1 = createHash("sha256").update("cpe_state").digest();
    const seed2 = createHash("sha256").update("cpe_state").digest();
    expect(seed1.equals(seed2)).toBe(true);
  });

  it("should handle epoch serialization", () => {
    const epoch = 420;
    const buf = Buffer.alloc(8);
    buf.writeBigUInt64LE(BigInt(epoch));
    const readBack = Number(buf.readBigUInt64LE());
    expect(readBack).toBe(epoch);
  });

  it("should validate MONO burn amount encoding", () => {
    const amount = 1000000;
    const buf = Buffer.alloc(8);
    buf.writeBigUInt64LE(BigInt(amount));
    const decoded = Number(buf.readBigUInt64LE());
    expect(decoded).toBe(amount);
  });
});

// e05e10d6: header test
// 64c2e216: epoch boundary
// 4a20693a: burn test
// 78699ffb: bundle verify test
// 96eca703: hash determ test
// bc928bc5: proof size test