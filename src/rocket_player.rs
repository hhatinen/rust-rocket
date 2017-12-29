use client::SyncTrackContainer;
use std::path::Path;
use track::Track;
use error::*;
use serde_xml_rs;

pub struct RocketPlayer {
    tracks: Vec<Track>,
}

#[derive(Deserialize)]
pub struct Sync {
    rows : i32,
    tracks: Vec<Track>,
}

impl RocketPlayer {

    pub fn new_xml(xml : &str) -> Result<RocketPlayer, RocketErr> {
        let sync : Sync = serde_xml_rs::deserialize(xml.as_bytes()).unwrap();
        Ok(RocketPlayer {
            tracks: Vec::new()
        })
    }

    pub fn new(sync_data : Sync) -> Result<RocketPlayer, RocketErr> {
        Ok(RocketPlayer {
            tracks: Vec::new()
        })
    }

}

impl SyncTrackContainer for RocketPlayer {
    fn get_track(&self, name: &str) -> Option<&Track> {
        self.tracks.iter().find(|t| t.get_name() == name)
    }
}



