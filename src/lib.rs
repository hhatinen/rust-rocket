//! This is the rust-rocket crate.
//! It is designed to work as a client library for GNU Rocket.

extern crate byteorder;

pub mod interpolation;
pub mod track;
pub mod client;
mod error;

pub use client::{Rocket, Event};
pub use error::RocketErr;
