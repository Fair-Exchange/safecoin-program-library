//! Program state processor

use {
    crate::{
        error::AssociatedTokenAccountError,
        instruction::AssociatedTokenAccountInstruction,
        tools::account::{create_pda_account, get_account_len},
        *,
    },
    borsh::BorshDeserialize,
    safecoin_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        system_program,
        sysvar::Sysvar,
    },
    safe_token_2022::{
        extension::{ExtensionType, StateWithExtensions},
        state::Account,
    },
};

/// Specify when to create the associated token account
#[derive(PartialEq)]
enum CreateMode {
    /// Always try to create the ATA
    Always,
    /// Only try to create the ATA if non-existent
    Idempotent,
}

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = if input.is_empty() {
        AssociatedTokenAccountInstruction::Create
    } else {
        AssociatedTokenAccountInstruction::try_from_slice(input)
            .map_err(|_| ProgramError::InvalidInstructionData)?
    };

    msg!("{:?}", instruction);

    match instruction {
        AssociatedTokenAccountInstruction::Create => {
            process_create_associated_token_account(program_id, accounts, CreateMode::Always)
        }
        AssociatedTokenAccountInstruction::CreateIdempotent => {
            process_create_associated_token_account(program_id, accounts, CreateMode::Idempotent)
        }
    }
}

/// Processes CreateAssociatedTokenAccount instruction
fn process_create_associated_token_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    create_mode: CreateMode,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let funder_info = next_account_info(account_info_iter)?;
    let associated_token_account_info = next_account_info(account_info_iter)?;
    let wallet_account_info = next_account_info(account_info_iter)?;
    let safe_token_mint_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let safe_token_program_info = next_account_info(account_info_iter)?;
    let safe_token_program_id = safe_token_program_info.key;

    let (associated_token_address, bump_seed) = get_associated_token_address_and_bump_seed_internal(
        wallet_account_info.key,
        safe_token_mint_info.key,
        program_id,
        safe_token_program_id,
    );
    if associated_token_address != *associated_token_account_info.key {
        msg!("Error: Associated address does not match seed derivation");
        return Err(ProgramError::InvalidSeeds);
    }

    if create_mode == CreateMode::Idempotent
        && associated_token_account_info.owner == safe_token_program_id
    {
        let ata_data = associated_token_account_info.data.borrow();
        if let Ok(associated_token_account) = StateWithExtensions::<Account>::unpack(&ata_data) {
            if associated_token_account.base.owner != *wallet_account_info.key {
                let error = AssociatedTokenAccountError::InvalidOwner;
                msg!("{}", error);
                return Err(error.into());
            }
            if associated_token_account.base.mint != *safe_token_mint_info.key {
                return Err(ProgramError::InvalidAccountData);
            }
            return Ok(());
        }
    }
    if *associated_token_account_info.owner != system_program::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let rent = Rent::get()?;

    let associated_token_account_signer_seeds: &[&[_]] = &[
        &wallet_account_info.key.to_bytes(),
        &safe_token_program_id.to_bytes(),
        &safe_token_mint_info.key.to_bytes(),
        &[bump_seed],
    ];

    let account_len = get_account_len(
        safe_token_mint_info,
        safe_token_program_info,
        &[ExtensionType::ImmutableOwner],
    )?;

    create_pda_account(
        funder_info,
        &rent,
        account_len,
        safe_token_program_id,
        system_program_info,
        associated_token_account_info,
        associated_token_account_signer_seeds,
    )?;

    msg!("Initialize the associated token account");
    invoke(
        &safe_token_2022::instruction::initialize_immutable_owner(
            safe_token_program_id,
            associated_token_account_info.key,
        )?,
        &[
            associated_token_account_info.clone(),
            safe_token_program_info.clone(),
        ],
    )?;
    invoke(
        &safe_token_2022::instruction::initialize_account3(
            safe_token_program_id,
            associated_token_account_info.key,
            safe_token_mint_info.key,
            wallet_account_info.key,
        )?,
        &[
            associated_token_account_info.clone(),
            safe_token_mint_info.clone(),
            wallet_account_info.clone(),
            safe_token_program_info.clone(),
        ],
    )
}
