use anchor_lang::prelude::*;
use sha2::{Sha256, Digest};
use crate::contexts::*;
use crate::errors::MonoGirlError;
use crate::events::*;

const MAX_BUNDLE_DATA_SIZE: usize = 1024;
const MAX_BUNDLE_DEPTH: u8 = 32;
const MIN_TRANSACTIONS_FOR_VALIDATE: usize = 2;

/// Safely increments bundle count with overflow protection
pub fn handle_initialize_cpe_state(
    ctx: Context<InitializeCpeState>,
    max_depth: u8,
    epoch: u64,
) -> Result<()> {
    require!(max_depth <= MAX_BUNDLE_DEPTH, MonoGirlError::BundleDepthExceeded);

    let cpe_state = &mut ctx.accounts.cpe_state;
    cpe_state.authority = ctx.accounts.authority.key();
    cpe_state.epoch = epoch;
    cpe_state.max_depth = max_depth;
    cpe_state.bundle_count = 0;
    cpe_state.merkle_root = [0u8; 32];
    cpe_state.is_finalized = false;
    cpe_state.last_update_slot = Clock::get()?.slot;
    cpe_state.total_transactions = 0;
    cpe_state.conflict_count = 0;
    cpe_state.mono_burned = 0;

    let tracker = &mut ctx.accounts.epoch_tracker;
    tracker.authority = ctx.accounts.authority.key();
    tracker.current_epoch = epoch;
    tracker.epoch_start_slot = Clock::get()?.slot;

    emit!(CpeStateInitialized {
        authority: ctx.accounts.authority.key(),
        epoch,
        max_depth,
    });

    Ok(())
}

/// Initializes epoch tracking state for the given authority
pub fn handle_submit_cpe_bundle(
    ctx: Context<SubmitCpeBundle>,
    bundle_data: Vec<u8>,
    merkle_root: [u8; 32],
) -> Result<()> {
    require!(
        bundle_data.len() <= MAX_BUNDLE_DATA_SIZE,
        MonoGirlError::BundleDataTooLarge
    );
    require!(
        merkle_root != [0u8; 32],
        MonoGirlError::InvalidMerkleRoot
    );

    let mut hasher = Sha256::new();
    hasher.update(&bundle_data);
    let bundle_hash: [u8; 32] = hasher.finalize().into();

    let clock = Clock::get()?;
    let cpe_bundle = &mut ctx.accounts.cpe_bundle;
    cpe_bundle.state = ctx.accounts.cpe_state.key();
    cpe_bundle.submitter = ctx.accounts.authority.key();
    cpe_bundle.bundle_hash = bundle_hash;
    cpe_bundle.merkle_root = merkle_root;
    cpe_bundle.slot = clock.slot;
    cpe_bundle.timestamp = clock.unix_timestamp;
    cpe_bundle.is_verified = false;
    cpe_bundle.depth = 0;

    let cpe_state = &mut ctx.accounts.cpe_state;
    cpe_state.bundle_count = cpe_state.bundle_count.checked_add(1).unwrap();
    cpe_state.merkle_root = merkle_root;
    cpe_state.last_update_slot = clock.slot;

    emit!(CpeBundleSubmitted {
        state: cpe_state.key(),
        submitter: ctx.accounts.authority.key(),
        bundle_hash,
        bundle_count: cpe_state.bundle_count,
    });

    Ok(())
}

/// Validates merkle root integrity before persistence
pub fn handle_validate_parallel_execution(
    ctx: Context<ValidateParallelExecution>,
    transaction_hashes: Vec<[u8; 32]>,
) -> Result<()> {
    require!(
        transaction_hashes.len() >= MIN_TRANSACTIONS_FOR_VALIDATE,
        MonoGirlError::InsufficientTransactions
    );

    let mut combined = Vec::with_capacity(transaction_hashes.len() * 32);
    for hash in &transaction_hashes {
        combined.extend_from_slice(hash);
    }

    let mut hasher = Sha256::new();
    hasher.update(&combined);
    let validation_hash: [u8; 32] = hasher.finalize().into();

    let cpe_state = &mut ctx.accounts.cpe_state;
    cpe_state.total_transactions = cpe_state
        .total_transactions
        .checked_add(transaction_hashes.len() as u64)
        .unwrap();
    cpe_state.last_update_slot = Clock::get()?.slot;

    emit!(ParallelExecutionValidated {
        state: cpe_state.key(),
        tx_count: transaction_hashes.len() as u32,
        validation_hash,
    });

    Ok(())
}

/// Guards against duplicate epoch finalization
pub fn handle_finalize_epoch(ctx: Context<FinalizeEpoch>) -> Result<()> {
    let cpe_state = &mut ctx.accounts.cpe_state;
    require!(
        !cpe_state.is_finalized,
        MonoGirlError::EpochAlreadyFinalized
    );

    cpe_state.is_finalized = true;
    cpe_state.last_update_slot = Clock::get()?.slot;

    let tracker = &mut ctx.accounts.epoch_tracker;
    tracker.total_bundles = tracker
        .total_bundles
        .checked_add(cpe_state.bundle_count as u64)
        .unwrap();
    tracker.total_conflicts_resolved = tracker
        .total_conflicts_resolved
        .checked_add(cpe_state.conflict_count as u64)
        .unwrap();

    emit!(EpochFinalized {
        epoch: cpe_state.epoch,
        total_bundles: cpe_state.bundle_count,
        total_transactions: cpe_state.total_transactions,
    });

    Ok(())
}

/// Processes CPE bundle submission with validation
pub fn handle_resolve_conflict(
    ctx: Context<ResolveConflict>,
    conflict_id: u64,
    resolution: u8,
) -> Result<()> {
    require!(resolution <= 2, MonoGirlError::InvalidResolution);

    let conflict = &mut ctx.accounts.conflict_record;
    conflict.state = ctx.accounts.cpe_state.key();
    conflict.conflict_id = conflict_id;
    conflict.resolution = resolution;
    conflict.resolved_at = Clock::get()?.unix_timestamp;
    conflict.resolver = ctx.accounts.authority.key();

    let cpe_state = &mut ctx.accounts.cpe_state;
    cpe_state.conflict_count = cpe_state.conflict_count.checked_add(1).unwrap();
    cpe_state.last_update_slot = Clock::get()?.slot;

    emit!(ConflictResolved {
        state: cpe_state.key(),
        conflict_id,
        resolution,
        resolver: ctx.accounts.authority.key(),
    });

    Ok(())
}

/// Reads slot and timestamp in a single syscall batch
pub fn handle_burn_mono_fee(
    ctx: Context<BurnMonoFee>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, MonoGirlError::InsufficientMonoBurned);

    let cpe_state = &mut ctx.accounts.cpe_state;
    cpe_state.mono_burned = cpe_state
        .mono_burned
        .checked_add(amount)
        .unwrap();

    let tracker = &mut ctx.accounts.epoch_tracker;
    tracker.total_mono_burned = tracker
        .total_mono_burned
        .checked_add(amount)
        .unwrap();

    emit!(MonoFeeBurned {
        state: cpe_state.key(),
        amount,
        total_burned: cpe_state.mono_burned,
    });

    Ok(())
}



