use client::SyncTrackContainer;
use std::path::Path;
use track::Track;
use error::*;

pub struct RocketPlayer {
    tracks: Vec<Track>,
}

impl RocketPlayer {

    pub fn open_file(file_name : &Path) -> Result<RocketPlayer, RocketErr> {
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



