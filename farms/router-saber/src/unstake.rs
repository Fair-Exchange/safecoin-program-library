//! Unstake LP tokens from a Saber farm instruction

use {
    solana_farm_sdk::program::{account, protocol::saber},
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        hash::Hasher,
        instruction::{AccountMeta, Instruction},
        msg,
        program::invoke,
        program_error::ProgramError,
    },
};

pub fn unstake(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("Processing AmmInstruction::Unstake");
    msg!("amount {} ", amount);

    #[allow(clippy::deprecated_cfg_attr)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    if let [
        user_account,
        user_lp_token_account,
        farm_program_id,
        _safe_token_id,
        miner,
        miner_vault,
        quarry,
        rewarder
        ] = accounts
    {
        if &quarry_mine::id() != farm_program_id.key {
            return Err(ProgramError::IncorrectProgramId);
        }
        if !account::check_token_account_owner(user_lp_token_account, user_account.key)? {
            return Err(ProgramError::IllegalOwner);
        }

        let initial_lp_token_user_balance = account::get_token_balance(user_lp_token_account)?;

        let lp_amount = if amount > 0 {
            amount
        } else {
            saber::get_stake_account_balance(miner)?
        };

        let mut hasher = Hasher::default();
        hasher.hash(b"global:withdraw_tokens");

        let mut data = hasher.result().as_ref()[..8].to_vec();
        data.extend_from_slice(&lp_amount.to_le_bytes());

        let saber_accounts = vec![
            AccountMeta::new_readonly(*user_account.key, true),
            AccountMeta::new(*miner.key, false),
            AccountMeta::new(*quarry.key, false),
            AccountMeta::new(*miner_vault.key, false),
            AccountMeta::new(*user_lp_token_account.key, false),
            AccountMeta::new_readonly(safe_token::id(), false),
            AccountMeta::new_readonly(*rewarder.key, false),
        ];

        let instruction = Instruction {
            program_id: quarry_mine::id(),
            accounts: saber_accounts,
            data,
        };

        invoke(&instruction, accounts)?;

        account::check_tokens_received(
            user_lp_token_account,
            initial_lp_token_user_balance,
            lp_amount,
        )?;
    } else {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    msg!("AmmInstruction::Unstake complete");
    Ok(())
}
