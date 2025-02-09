// #![deny(missing_docs)]

//! An Uniswap-like program for the Solana blockchain.
#[macro_use]
pub mod log;
pub mod error;
pub mod instruction;
pub mod invokers;
pub mod math;
pub mod processor;
pub mod state;

pub use solana_program;

