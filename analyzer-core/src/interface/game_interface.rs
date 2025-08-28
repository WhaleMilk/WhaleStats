use crate::data_processor::Game;
use crate::interface::Interface;
use crate::data_processor::RawData;
use crate::interface::MatchData;
use crate::interface::Timeline;

use serde::Deserialize;
use serde_json::Deserializer;
use std::error::Error;

impl Interface {
    pub async fn get_game_ids(&self, start_timestamp: &String, puuid: &String) -> Result<Vec<String>, Box<dyn Error>> {
        //todo!();
        let mut ids = Vec::new();
        let resp = reqwest::get(
            format!(
                "https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids{}api_key={}",
                self.server, puuid, start_timestamp, self.api_key)
            ).await.unwrap().text().await.unwrap();
        let mut deserializer = Deserializer::from_str(&resp);
        Deserialize::deserialize_in_place(&mut deserializer, &mut ids).unwrap();
        Ok(ids)
    }

    pub async fn get_match_data_collection(&self, ids: Vec<String>, puuid: &String) -> Result<Vec<RawData>, Box<dyn Error>> {
        // println!("{}", "Getting player game data...".green());
        // let mut out: Vec<FilteredData> = Vec::new();
        // //let mut out: Vec<FilteredData> = self.get_game_data_from_ids(self.game_ids);
        // let check_valid_game = |game: &MatchData| -> bool {
        //     if game.info.end_of_game_result != "GameComplete" || game.info.game_mode != "CLASSIC" || game.info.game_type != "MATCHED_GAME" {
        //         return false;
        //     }
        //     true
        // };
        
        // for id in &ids {
        //     let game = IntakeHelper::request_game_data(&id, &api_key).await.unwrap();
        //     if check_valid_game(&game) {
        //         let tl = Self::request_match_timeline(&id, &api_key).await.unwrap();
        //         let mut filtered = FilteredData::new(&game, &tl);
        //         filtered.find_me(&puuid);
        //         out.push(filtered);
        //     }
        // }
        // Ok(out)
        let mut out: Vec<RawData> = Vec::new();
        let check_valid_game = |game: &MatchData| -> bool {
            if game.info.end_of_game_result != "GameComplete" || game.info.game_mode != "CLASSIC" || game.info.game_type != "MATCHED_GAME" {
                return false;
            }
            true
        };

        for id in &ids {
            let game_data = self.request_game_data(id).await.unwrap();
            if check_valid_game(&game_data) {
                let game_tl = self.request_match_timeline(id).await.unwrap();
                let mut raw = RawData::new(&game_data, &game_tl);
                raw.find_me(puuid);
                out.push(raw);
            }
        }
        Ok(out)
    }

    async fn request_game_data(&self, id: &String) -> Result<MatchData, Box<dyn Error>> {
        let resp = reqwest::get(
            format!("https://{}.api.riotgames.com/lol/match/v5/matches/{}?api_key={}", self.server, id, self.api_key)
        ).await
            .unwrap()
            .text()
            .await
            .unwrap();
        let game: MatchData = serde_json::from_str(&resp)?;
        Ok(game)
    }

    async fn request_match_timeline(&self, id: &String) -> Result<Timeline, Box<dyn Error>> {
        let resp = reqwest::get(format!("https://{}.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}", self.server, id, self.api_key)).await.unwrap().text().await.unwrap();
        let out: Timeline = serde_json::from_str(&resp)?;
        Ok(out)
    }
}