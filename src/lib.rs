// lib.rs

pub mod msg;
pub mod shared;

#[cfg(feature = "client")]
pub mod render;

#[cfg(feature = "server")]
pub mod srv;
