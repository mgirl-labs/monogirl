use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct CpeState {
    pub authority: Pubkey,
    pub epoch: u64,
    pub max_depth: u8,
    pub bundle_count: u32,
    pub merkle_root: [u8; 32],
    pub is_finalized: bool,
    pub last_update_slot: u64,
    pub total_transactions: u64,
    pub conflict_count: u32,
    pub mono_burned: u64,
    pub _padding: [u8; 7],
}

impl CpeState {
    pub const LEN: usize = 32 + 8 + 1 + 4 + 32 + 1 + 8 + 8 + 4 + 8 + 7;
}

#[account]
#[derive(Default)]
pub struct CpeBundle {
    pub state: Pubkey,
    pub submitter: Pubkey,
    pub bundle_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub slot: u64,
    pub timestamp: i64,
    pub is_verified: bool,
    pub depth: u8,
    pub tx_count: u16,
    pub _padding: [u8; 4],
}

impl CpeBundle {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 2 + 4;
}

#[account]
#[derive(Default)]
pub struct ConflictRecord {
    pub state: Pubkey,
    pub conflict_id: u64,
    pub tx_hash_a: [u8; 32],
    pub tx_hash_b: [u8; 32],
    pub resolution: u8,
    pub resolved_at: i64,
    pub resolver: Pubkey,
    pub _padding: [u8; 7],
}

impl ConflictRecord {
    pub const LEN: usize = 32 + 8 + 32 + 32 + 1 + 8 + 32 + 7;
}

#[account]
#[derive(Default)]
pub struct EpochTracker {
    pub authority: Pubkey,
    pub current_epoch: u64,
    pub epoch_start_slot: u64,
    pub total_bundles: u64,
    pub total_conflicts_resolved: u64,
    pub total_mono_burned: u64,
    pub _padding: [u8; 8],
}

impl EpochTracker {
    pub const LEN: usize = 32 + 8 + 8 + 8 + 8 + 8 + 8;
}










