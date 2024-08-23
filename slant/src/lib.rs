/*
    Appellation: slant <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # slant
//!
//! slant is a mathematical sandbox focused on interactive visualizations and animations.
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "slant"]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use slant_core::*;

#[cfg(feature = "anim")]
pub use slant_anim as anim;

#[allow(unused_imports)]
pub mod prelude {
    pub use slant_core::prelude::*;
    #[cfg(feature = "anim")]
    pub use crate::anim::prelude::*;
}
