use anchor_lang::prelude::*;

#[event]
pub struct CpeStateInitialized {
    pub authority: Pubkey,
    pub epoch: u64,
    pub max_depth: u8,
}

#[event]
pub struct CpeBundleSubmitted {
    pub state: Pubkey,
    pub submitter: Pubkey,
    pub bundle_hash: [u8; 32],
    pub bundle_count: u32,
}

#[event]
pub struct ParallelExecutionValidated {
    pub state: Pubkey,
    pub tx_count: u32,
    pub validation_hash: [u8; 32],
}

#[event]
pub struct EpochFinalized {
    pub epoch: u64,
    pub total_bundles: u32,
    pub total_transactions: u64,
}

#[event]
pub struct ConflictResolved {
    pub state: Pubkey,
    pub conflict_id: u64,
    pub resolution: u8,
    pub resolver: Pubkey,
}

#[event]
pub struct MonoFeeBurned {
    pub state: Pubkey,
    pub amount: u64,
    pub total_burned: u64,
}

