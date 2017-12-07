//! This is the rust-rocket crate.
//! It is designed to work as a client library for GNU Rocket.

extern crate byteorder;

mod interpolation;
mod track;
mod client;
mod rocket_player;
mod error;

pub use client::{RocketEditor, Event, SyncTrackContainer};
pub use track::Track;
pub use rocket_player::RocketPlayer;
pub use error::RocketErr;
