//! ## Dev Notes
//!
//! - For the `*IxData` struct, keep the encapsulated byte array private
//!   and only expose via `self.as_buf()` so that users cannot
//!   accidentally set the wrong discriminant or input invalid data

mod internal_utils;

pub mod burn;
pub mod mint_to;
pub mod transfer;
