//! # AjoChain Shared Types
//!
//! Cross-contract data structures, constants, and TTL configuration shared
//! across the entire AjoChain protocol. This crate is linked into every
//! contract to ensure type compatibility and consistent behaviour.

#![no_std]

pub mod constants;
pub mod ttl;
pub mod version;
