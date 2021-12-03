#![deny(missing_docs)]

//! A lending program for the Safecoin blockchain.

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod math;
pub mod processor;
pub mod pyth;
pub mod state;

// Export current sdk types for downstream users building with a different sdk version
pub use safecoin_program;

safecoin_program::declare_id!("LENeX3L4CE1euBZp4zUNuicLP2SUZCbgXYZgBpZ9hWZ");
