//! Error types

use num_derive::FromPrimitive;
use safecoin_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the StakePool program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum StakePoolError {
    // 0.
    /// The account cannot be initialized because it is already being used.
    #[error("AlreadyInUse")]
    AlreadyInUse,
    /// The program address provided doesn't match the value generated by the program.
    #[error("InvalidProgramAddress")]
    InvalidProgramAddress,
    /// The stake pool state is invalid.
    #[error("InvalidState")]
    InvalidState,
    /// The calculation failed.
    #[error("CalculationFailure")]
    CalculationFailure,
    /// Stake pool fee > 1.
    #[error("FeeTooHigh")]
    FeeTooHigh,

    // 5.
    /// Token account is associated with the wrong mint.
    #[error("WrongAccountMint")]
    WrongAccountMint,
    /// Wrong pool manager account.
    #[error("WrongManager")]
    WrongManager,
    /// Required signature is missing.
    #[error("SignatureMissing")]
    SignatureMissing,
    /// Invalid validator stake list account.
    #[error("InvalidValidatorStakeList")]
    InvalidValidatorStakeList,
    /// Invalid manager fee account.
    #[error("InvalidFeeAccount")]
    InvalidFeeAccount,

    // 10.
    /// Specified pool mint account is wrong.
    #[error("WrongPoolMint")]
    WrongPoolMint,
    /// Stake account is not in the state expected by the program.
    #[error("WrongStakeState")]
    WrongStakeState,
    /// User stake is not active
    #[error("UserStakeNotActive")]
    UserStakeNotActive,
    /// Stake account voting for this validator already exists in the pool.
    #[error("ValidatorAlreadyAdded")]
    ValidatorAlreadyAdded,
    /// Stake account for this validator not found in the pool.
    #[error("ValidatorNotFound")]
    ValidatorNotFound,

    // 15.
    /// Stake account address not properly derived from the validator address.
    #[error("InvalidStakeAccountAddress")]
    InvalidStakeAccountAddress,
    /// Identify validator stake accounts with old balances and update them.
    #[error("StakeListOutOfDate")]
    StakeListOutOfDate,
    /// First update old validator stake account balances and then pool stake balance.
    #[error("StakeListAndPoolOutOfDate")]
    StakeListAndPoolOutOfDate,
    /// Validator stake account is not found in the list storage.
    #[error("UnknownValidatorStakeAccount")]
    UnknownValidatorStakeAccount,
    /// Wrong minting authority set for mint pool account
    #[error("WrongMintingAuthority")]
    WrongMintingAuthority,

    // 20.
    /// The size of the given validator stake list does match the expected amount
    #[error("UnexpectedValidatorListAccountSize")]
    UnexpectedValidatorListAccountSize,
    /// Wrong pool staker account.
    #[error("WrongStaker")]
    WrongStaker,
    /// Pool token supply is not zero on initialization
    #[error("NonZeroPoolTokenSupply")]
    NonZeroPoolTokenSupply,
    /// The lamports in the validator stake account is not equal to the minimum
    #[error("StakeLamportsNotEqualToMinimum")]
    StakeLamportsNotEqualToMinimum,
    /// The provided deposit stake account is not delegated to the preferred deposit vote account
    #[error("IncorrectDepositVoteAddress")]
    IncorrectDepositVoteAddress,

    // 25.
    /// The provided withdraw stake account is not the preferred deposit vote account
    #[error("IncorrectWithdrawVoteAddress")]
    IncorrectWithdrawVoteAddress,
    /// The mint has an invalid freeze authority
    #[error("InvalidMintFreezeAuthority")]
    InvalidMintFreezeAuthority,
    /// Proposed fee increase exceeds stipulated ratio
    #[error("FeeIncreaseTooHigh")]
    FeeIncreaseTooHigh,
    /// Not enough pool tokens provided to withdraw stake with one lamport
    #[error("WithdrawalTooSmall")]
    WithdrawalTooSmall,
    /// Not enough lamports provided for deposit to result in one pool token
    #[error("DepositTooSmall")]
    DepositTooSmall,

    // 30.
    /// Provided stake deposit authority does not match the program's
    #[error("InvalidStakeDepositAuthority")]
    InvalidStakeDepositAuthority,
    /// Provided sol deposit authority does not match the program's
    #[error("InvalidSafeDepositAuthority")]
    InvalidSafeDepositAuthority,
    /// Provided preferred validator is invalid
    #[error("InvalidPreferredValidator")]
    InvalidPreferredValidator,
    /// Provided validator stake account already has a transient stake account in use
    #[error("TransientAccountInUse")]
    TransientAccountInUse,
    /// Provided sol withdraw authority does not match the program's
    #[error("InvalidSafeWithdrawAuthority")]
    InvalidSafeWithdrawAuthority,

    // 35.
    /// Too much SAFE withdrawn from the stake pool's reserve account
    #[error("SafeWithdrawalTooLarge")]
    SafeWithdrawalTooLarge,
}
impl From<StakePoolError> for ProgramError {
    fn from(e: StakePoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for StakePoolError {
    fn type_of() -> &'static str {
        "Stake Pool Error"
    }
}
