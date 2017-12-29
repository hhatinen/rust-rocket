//! This is the rust-rocket crate.
//! It is designed to work as a client library for GNU Rocket.

extern crate byteorder;
#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;

mod interpolation;
mod track;
mod client;
mod rocket_player;
mod error;

pub use client::{RocketEditor, Event, SyncTrackContainer};
pub use track::Track;
pub use rocket_player::RocketPlayer;
pub use error::RocketErr;
