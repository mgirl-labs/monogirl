use anchor_lang::prelude::*;

#[error_code]
pub enum MonoGirlError {
    #[msg("Bundle depth exceeds maximum allowed depth")]
    BundleDepthExceeded,

    #[msg("Epoch has already been finalized")]
    EpochAlreadyFinalized,

    #[msg("Invalid Merkle root provided")]
    InvalidMerkleRoot,

    #[msg("Transaction set is empty")]
    EmptyTransactionSet,

    #[msg("Conflict has already been resolved")]
    ConflictAlreadyResolved,

    #[msg("Invalid resolution value")]
    InvalidResolution,

    #[msg("Bundle data exceeds maximum size")]
    BundleDataTooLarge,

    #[msg("Insufficient transactions for parallel validation")]
    InsufficientTransactions,

    #[msg("Epoch mismatch between CPE state and current epoch")]
    EpochMismatch,

    #[msg("Parallel execution validation failed")]
    ValidationFailed,

    #[msg("Account sets are not independent for parallel execution")]
    AccountSetConflict,

    #[msg("Insufficient MONO burned for CPE request")]
    InsufficientMonoBurned,
}


