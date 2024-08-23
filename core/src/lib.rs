/*
    Appellation: slant <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # slant
//!
//! slant is a mathematical sandbox focused on interactive visualizations and animations.
#![allow(unused_imports)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    error::Error,
    traits::*,
    types::*,
};
#[macro_use]
pub(crate) mod macros;

pub mod error;
pub mod traits;
pub mod types;

pub mod prelude {}
