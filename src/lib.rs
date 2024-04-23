//! # wordmash - String Arithmetics Library
//!
//! This crate makes it possible to perform arithmetic operations on strings by representing them
//! as base36 numbers. This is done through the `Masher` struct, which can construct base36
//! numbers from all types implementing `Mashable` trait (currently all unsigned integer types,
//! `String` and `&str`) and perform arithmetics by reverting them back to base10.

/// Error types
pub mod errors;

/// `Mashable` trait
pub mod mashable;

/// `Masher` struct
pub mod masher;

#[cfg(test)]
mod tests;
