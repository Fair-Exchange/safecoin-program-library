//! Record program
#![deny(missing_docs)]

mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// Export current SDK types for downstream users building with a different SDK version
pub use safecoin_program;

safecoin_program::declare_id!("recj3dqMtYuYPCW1VXmhDZQJ5k8o5JurzER4pYCRtK6");
