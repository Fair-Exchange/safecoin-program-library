use safecoin_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, instruction::Instruction,
    pubkey::Pubkey,
};

declare_id!("WRAPYChf58WFCnyjXKJHtrPgzKXgHp6MD9aVDqJBbGh");

#[cfg(not(feature = "no-entrypoint"))]
use safecoin_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(noop);

pub fn noop(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

pub fn instruction(data: Vec<u8>) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![],
        data,
    }
}
