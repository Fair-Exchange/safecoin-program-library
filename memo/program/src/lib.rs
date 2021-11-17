#![deny(missing_docs)]

//! A program that accepts a string of encoded characters and verifies that it parses,
//! while verifying and logging signers. Currently handles UTF-8 characters.

mod entrypoint;
pub mod processor;

// Export current sdk types for downstream users building with a different sdk version
pub use safecoin_program;
use safecoin_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

/// Legacy symbols from Memo v1
pub mod v1 {
    safecoin_program::declare_id!("MEMDqRW2fYAU19mcFnoDVoqG4Br4t7TdyWjjv38P6Nc");
}

safecoin_program::declare_id!("MEMWKbqsjEB8o972BvDHExZFSauzGZKvB4xHDVPFowh");

/// Build a memo instruction, possibly signed
///
/// Accounts expected by this instruction:
///
///   0. ..0+N. `[signer]` Expected signers; if zero provided, instruction will be processed as a
///     normal, unsigned safe-memo
///
pub fn build_memo(memo: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: memo.to_vec(),
    }
}
