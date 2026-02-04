use anchor_lang::prelude::*;
use crate::state::{CpeState, CpeBundle, ConflictRecord, EpochTracker};

#[derive(Accounts)]
#[instruction(max_depth: u8, epoch: u64)]
pub struct InitializeCpeState<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + CpeState::LEN,
        seeds = [b"cpe_state", authority.key().as_ref(), &epoch.to_le_bytes()],
        bump,
    )]
    pub cpe_state: Account<'info, CpeState>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + EpochTracker::LEN,
        seeds = [b"epoch_tracker", authority.key().as_ref()],
        bump,
    )]
    pub epoch_tracker: Account<'info, EpochTracker>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitCpeBundle<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = !cpe_state.is_finalized,
    )]
    pub cpe_state: Account<'info, CpeState>,

    #[account(
        init,
        payer = authority,
        space = 8 + CpeBundle::LEN,
        seeds = [
            b"cpe_bundle",
            cpe_state.key().as_ref(),
            &cpe_state.bundle_count.to_le_bytes(),
        ],
        bump,
    )]
    pub cpe_bundle: Account<'info, CpeBundle>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ValidateParallelExecution<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub cpe_state: Account<'info, CpeState>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct FinalizeEpoch<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub cpe_state: Account<'info, CpeState>,

    #[account(
        mut,
        seeds = [b"epoch_tracker", authority.key().as_ref()],
        bump,
    )]
    pub epoch_tracker: Account<'info, EpochTracker>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(conflict_id: u64)]
pub struct ResolveConflict<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub cpe_state: Account<'info, CpeState>,

    #[account(
        init,
        payer = authority,
        space = 8 + ConflictRecord::LEN,
        seeds = [
            b"conflict",
            cpe_state.key().as_ref(),
            &conflict_id.to_le_bytes(),
        ],
        bump,
    )]
    pub conflict_record: Account<'info, ConflictRecord>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BurnMonoFee<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub cpe_state: Account<'info, CpeState>,

    #[account(
        mut,
        seeds = [b"epoch_tracker", authority.key().as_ref()],
        bump,
    )]
    pub epoch_tracker: Account<'info, EpochTracker>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

