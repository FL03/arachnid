/*
    Appellation: arachnid <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Arachnid
//!
//!

#[cfg(feature = "core")]
pub use arachnid_core as core;
#[cfg(feature = "rat")]
pub use arachnid_rat as rat;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use crate::core::prelude::*;
    #[cfg(feature = "rat")]
    pub use crate::rat::prelude::*;
}
