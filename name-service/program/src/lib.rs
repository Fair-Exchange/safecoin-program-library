#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// Export current sdk types for downstream users building with a different sdk version
pub use safecoin_program;

safecoin_program::declare_id!("safekD6aFjBDfsKutf8guzqx8uydZMEtzTih8NvhfQ3");
