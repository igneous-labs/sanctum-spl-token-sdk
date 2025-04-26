//! Uninitialized state is generally treated as illegal and unrepresentable in this library
//! because this this library is meant to be for users of the token program.
//! Which is also why this crate focuses mainly on readonly functionality instead of mutation.
//!
//! If this was an SDK to be used to build the token program it would likely look different.

mod coption;

pub mod account;
