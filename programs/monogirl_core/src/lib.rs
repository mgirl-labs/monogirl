use anchor_lang::prelude::*;

pub mod contexts;
pub mod errors;
pub mod events;
pub mod processor;
pub mod state;
pub mod utils;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod monogirl_core {
    use super::*;

    pub fn initialize_cpe_state(
        ctx: Context<contexts::InitializeCpeState>,
        max_depth: u8,
        epoch: u64,
    ) -> Result<()> {
        processor::handle_initialize_cpe_state(ctx, max_depth, epoch)
    }

    pub fn submit_cpe_bundle(
        ctx: Context<contexts::SubmitCpeBundle>,
        bundle_data: Vec<u8>,
        merkle_root: [u8; 32],
    ) -> Result<()> {
        processor::handle_submit_cpe_bundle(ctx, bundle_data, merkle_root)
    }

    pub fn validate_parallel_execution(
        ctx: Context<contexts::ValidateParallelExecution>,
        transaction_hashes: Vec<[u8; 32]>,
    ) -> Result<()> {
        processor::handle_validate_parallel_execution(ctx, transaction_hashes)
    }

    pub fn finalize_epoch(
        ctx: Context<contexts::FinalizeEpoch>,
    ) -> Result<()> {
        processor::handle_finalize_epoch(ctx)
    }

    pub fn resolve_conflict(
        ctx: Context<contexts::ResolveConflict>,
        conflict_id: u64,
        resolution: u8,
    ) -> Result<()> {
        processor::handle_resolve_conflict(ctx, conflict_id, resolution)
    }

    pub fn burn_mono_fee(
        ctx: Context<contexts::BurnMonoFee>,
        amount: u64,
    ) -> Result<()> {
        processor::handle_burn_mono_fee(ctx, amount)
    }
}
