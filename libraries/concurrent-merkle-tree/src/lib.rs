#![allow(clippy::integer_arithmetic)]
//! # Concurrent Merkle Tree
//!
//! This crate is a Safecoin-optimized implementation of the
//! concurrent merkle tree data structure introduced in [this
//! whitepaper](https://drive.google.com/file/d/1BOpa5OFmara50fTvL0VIVYjtg-qzHCVc/view)
//!
//! The core implementation of CMT can be found in [concurrent_merkle_tree]

/// Private macros to enable logging in the Safecoin runtime
#[macro_use]
mod log;
/// Changelog implementation to keep track of information necessary to fast forward proofs
pub mod changelog;
/// Core implementation of the concurrent merkle tree structure
pub mod concurrent_merkle_tree;
/// Descriptive errors
pub mod error;
/// Hashing utils to support merkle tree operations
pub mod hash;
/// Node implementation and utils
pub mod node;
/// Path implementation
pub mod path;
