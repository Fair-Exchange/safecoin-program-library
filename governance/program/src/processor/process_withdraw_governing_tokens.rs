//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    error::GovernanceError,
    state::{
        realm::{get_realm_address_seeds, get_realm_data},
        realm_config::get_realm_config_data_for_realm,
        token_owner_record::{
            get_token_owner_record_address_seeds, get_token_owner_record_data_for_seeds,
        },
    },
    tools::safe_token::{get_safe_token_mint, transfer_safe_tokens_signed},
};

/// Processes WithdrawGoverningTokens instruction
pub fn process_withdraw_governing_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let realm_info = next_account_info(account_info_iter)?; // 0
    let governing_token_holding_info = next_account_info(account_info_iter)?; // 1
    let governing_token_destination_info = next_account_info(account_info_iter)?; // 2
    let governing_token_owner_info = next_account_info(account_info_iter)?; // 3
    let token_owner_record_info = next_account_info(account_info_iter)?; // 4
    let safe_token_info = next_account_info(account_info_iter)?; // 5
    let realm_config_info = next_account_info(account_info_iter)?; // 6

    if !governing_token_owner_info.is_signer {
        return Err(GovernanceError::GoverningTokenOwnerMustSign.into());
    }

    let realm_data = get_realm_data(program_id, realm_info)?;
    let governing_token_mint = get_safe_token_mint(governing_token_holding_info)?;

    realm_data.assert_is_valid_governing_token_mint_and_holding(
        program_id,
        realm_info.key,
        &governing_token_mint,
        governing_token_holding_info.key,
    )?;

    let realm_config_data =
        get_realm_config_data_for_realm(program_id, realm_config_info, realm_info.key)?;

    realm_config_data.assert_can_withdraw_governing_token(&realm_data, &governing_token_mint)?;

    let token_owner_record_address_seeds = get_token_owner_record_address_seeds(
        realm_info.key,
        &governing_token_mint,
        governing_token_owner_info.key,
    );

    let mut token_owner_record_data = get_token_owner_record_data_for_seeds(
        program_id,
        token_owner_record_info,
        &token_owner_record_address_seeds,
    )?;

    token_owner_record_data.assert_can_withdraw_governing_tokens()?;

    transfer_safe_tokens_signed(
        governing_token_holding_info,
        governing_token_destination_info,
        realm_info,
        &get_realm_address_seeds(&realm_data.name),
        program_id,
        token_owner_record_data.governing_token_deposit_amount,
        safe_token_info,
    )?;

    token_owner_record_data.governing_token_deposit_amount = 0;
    token_owner_record_data.serialize(&mut *token_owner_record_info.data.borrow_mut())?;

    Ok(())
}
