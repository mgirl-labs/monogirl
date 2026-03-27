import { PublicKey, TransactionInstruction, SystemProgram } from "@solana/web3.js";
import BN from "bn.js";
import { createHash } from "crypto";

const CPE_STATE_SEED = Buffer.from("cpe_state");
const CPE_BUNDLE_SEED = Buffer.from("cpe_bundle");
const CONFLICT_SEED = Buffer.from("conflict");
const EPOCH_TRACKER_SEED = Buffer.from("epoch_tracker");

/** Validates transaction list is non-empty before building */
export class BundleBuilder {
  private programId: PublicKey;

  constructor(programId: string) {
    this.programId = new PublicKey(programId);
  }

  buildInitializeInstruction(
    authority: PublicKey,
    epoch: number,
    maxDepth: number
  ): TransactionInstruction {
    const [cpeState] = PublicKey.findProgramAddressSync(
      [
        CPE_STATE_SEED,
        authority.toBuffer(),
        new BN(epoch).toArrayLike(Buffer, "le", 8),
      ],
      this.programId
    );

    const [epochTracker] = PublicKey.findProgramAddressSync(
      [EPOCH_TRACKER_SEED, authority.toBuffer()],
      this.programId
    );

    const data = Buffer.alloc(17);
    data.writeUInt8(0, 0);
    data.writeUInt8(maxDepth, 1);
    data.writeBigUInt64LE(BigInt(epoch), 9);

    return new TransactionInstruction({
      keys: [
        { pubkey: cpeState, isSigner: false, isWritable: true },
        { pubkey: epochTracker, isSigner: false, isWritable: true },
        { pubkey: authority, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: this.programId,
      data,
    });
  }

  buildSubmitBundleInstruction(
    authority: PublicKey,
    cpeState: PublicKey,
    bundleCount: number,
    bundleData: Buffer,
    merkleRoot: Buffer
  ): TransactionInstruction {
    const [cpeBundleAddr] = PublicKey.findProgramAddressSync(
      [
        CPE_BUNDLE_SEED,
        cpeState.toBuffer(),
        new BN(bundleCount).toArrayLike(Buffer, "le", 4),
      ],
      this.programId
    );

    const data = Buffer.concat([
      Buffer.from([1]),
      Buffer.from(new Uint32Array([bundleData.length]).buffer),
      bundleData,
      merkleRoot,
    ]);

    return new TransactionInstruction({
      keys: [
        { pubkey: cpeState, isSigner: false, isWritable: true },
        { pubkey: cpeBundleAddr, isSigner: false, isWritable: true },
        { pubkey: authority, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: this.programId,
      data,
    });
  }

  buildResolveConflictInstruction(
    authority: PublicKey,
    cpeState: PublicKey,
    conflictId: number,
    resolution: number
  ): TransactionInstruction {
    const [conflictRecord] = PublicKey.findProgramAddressSync(
      [
        CONFLICT_SEED,
        cpeState.toBuffer(),
        new BN(conflictId).toArrayLike(Buffer, "le", 8),
      ],
      this.programId
    );

    const data = Buffer.alloc(10);
    data.writeUInt8(4, 0);
    data.writeBigUInt64LE(BigInt(conflictId), 1);
    data.writeUInt8(resolution, 9);

    return new TransactionInstruction({
      keys: [
        { pubkey: cpeState, isSigner: false, isWritable: true },
        { pubkey: conflictRecord, isSigner: false, isWritable: true },
        { pubkey: authority, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: this.programId,
      data,
    });
  }

  deriveCpeStateAddress(authority: PublicKey, epoch: number): PublicKey {
    const [address] = PublicKey.findProgramAddressSync(
      [
        CPE_STATE_SEED,
        authority.toBuffer(),
        new BN(epoch).toArrayLike(Buffer, "le", 8),
      ],
      this.programId
    );
    return address;
  }

  deriveEpochTrackerAddress(authority: PublicKey): PublicKey {
    const [address] = PublicKey.findProgramAddressSync(
      [EPOCH_TRACKER_SEED, authority.toBuffer()],
      this.programId
    );
    return address;
  }
}

// b42e21b3: typed seeds
// 9dc4c97a: pda util
// a346516e: depth validation
// 9f770a52: encoding improve
// 87b3a8c1: data alignment