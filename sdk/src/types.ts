import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

export interface BundleConfig {
  rpcUrl: string;
  programId: string;
  maxDepth?: number;
  timeout?: number;
}

export interface BundleResult {
  signature: string;
  merkleRoot: Buffer;
  bundleCount: number;
  epoch: BN;
  slot: BN;
}

export interface CpeBundle {
  stateAddress: PublicKey;
  authority: PublicKey;
  bundleData: Buffer;
  merkleRoot: Buffer;
  transactionHashes: Buffer[];
  depth: number;
  timestamp: BN;
}

export interface VerificationResult {
  isValid: boolean;
  merkleRoot: Buffer;
  validationHash: Buffer;
  transactionCount: number;
}

export interface ConflictInfo {
  conflictId: BN;
  txHashA: Buffer;
  txHashB: Buffer;
  resolution: number;
  resolvedAt: BN;
  resolver: PublicKey;
}

export interface EpochData {
  currentEpoch: BN;
  epochStartSlot: BN;
  totalBundles: BN;
  totalConflictsResolved: BN;
  totalMonoBurned: BN;
}

export interface CpeStateAccount {
  authority: PublicKey;
  epoch: BN;
  maxDepth: number;
  bundleCount: number;
  merkleRoot: Buffer;
  isFinalized: boolean;
  lastUpdateSlot: BN;
  totalTransactions: BN;
  conflictCount: number;
  monoBurned: BN;
}

export interface CpeBundleAccount {
  state: PublicKey;
  submitter: PublicKey;
  bundleHash: Buffer;
  merkleRoot: Buffer;
  slot: BN;
  timestamp: BN;
  isVerified: boolean;
  depth: number;
  txCount: number;
}
