//! binary oracle pair
#![deny(missing_docs)]

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

// Binary Oracle Pair id
solana_program::declare_id!("bop7P4FFBNz6jgKJqgo3UmA2MuHR6K1D6a3RfRRg5Jo");
