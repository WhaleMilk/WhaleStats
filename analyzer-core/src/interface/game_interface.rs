use crate::interface::Interface;
use crate::data_processor::RawData;
use crate::interface::MatchData;
use crate::interface::Timeline;

use serde::Deserialize;
use std::error::Error;

impl Interface {
    pub async fn get_game_ids(&self, start_timestamp: &String) -> Result<Vec<String>, Box<dyn Error>> {
        todo!()
    }

    pub async fn get_match_data_collection(&self, ids: Vec<String>, puuid: &String) -> Result<Vec<RawData>, Box<dyn Error>> {
        todo!()
    }

    async fn request_game_data(id: &String) -> Result<MatchData, Box<dyn Error>> {
        todo!()
    }

    async fn request_match_timeline(id: &String) -> Result<Timeline, Box<dyn Error>> {
        todo!()
    }
}