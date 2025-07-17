use owo_colors::OwoColorize;

use crate::player::{PlayerIdent, Summoner};
use crate::intake::match_data::MatchData;
use crate::intake::timeline::Timeline;
use crate::intake::data_filter::FilteredData;

pub mod match_data;
pub mod ranked_data;
pub mod timeline;
pub mod data_filter;

use crate::StartData;

use serde::Deserialize;
use serde_json::Deserializer;
use std::error::Error;
use chrono::{NaiveDate, Utc, TimeZone};

pub struct IntakeHelper {
    game_ids: Vec<String>,
    start_data: StartData,
}

impl IntakeHelper {
    pub async fn new(start: &StartData) -> IntakeHelper {
        //let games = Self::get_games(&start.start_date, &start.end_date, &start.api_key, &start.puuid, &start.region).await.unwrap();
        IntakeHelper {
            game_ids: Vec::<String>::new(),
            start_data: start.clone(),
        }
    }

    pub async fn get_games_fresh(start: &String, api_key: &String, puuid: &String, region: &String) -> Result<Vec<String>, String> {
        let mut game_ids = Vec::new();
        let start_end = Self::get_time_range_to_now(start).await.unwrap();
        let absolute_server = region.as_str();
        let server = Self::get_server(absolute_server).await;
        
        let resp = reqwest::get
            (format!("https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids{}&api_key={}",
            server, puuid, start_end, api_key)).await.unwrap().text().await.unwrap();
        
        let mut deserialized =  Deserializer::from_str(&resp);
        println!("{}", resp.purple());
        Deserialize::deserialize_in_place(&mut deserialized, &mut game_ids).unwrap();
        Ok(game_ids)
    }

    pub async fn get_games_start_end(start: &String, end: &String, api_key: &String, puuid: &String, region: &String) -> Result<Vec<String>, String> {
        let mut game_ids = Vec::new();
        let start_end = Self::get_time_range(&start, &end).await.unwrap();
        let absolute_server = region.as_str();
        let server = IntakeHelper::get_server(absolute_server).await;
        println!("{}", format!("Between dates {} and {}", start, end).purple());
        let resp = reqwest::get
            (format!("https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids{}&api_key={}",
            server, puuid, start_end, api_key)).await.unwrap().text().await.unwrap();

        let mut deserializer = Deserializer::from_str(&resp);
        Deserialize::deserialize_in_place(&mut deserializer, &mut game_ids).unwrap();
        println!("{}", resp.purple());
        Ok(game_ids)
    }

    pub async fn get_games_utc(start: i64, puuid: &String, region: &String, api_key: &String) -> Result<Vec<String>, String>{
        let mut game_ids: Vec<String> = Vec::new();
        let now: i64 = Utc::now().timestamp();
        let start_new: i64 = start / 1000;
        let start_end = format!("?startTime={}&endTime={}", start_new + 1, now);
        println!("{start_end}");
        let absolute_server = region.as_str();
        let server = IntakeHelper::get_server(absolute_server).await;
        let resp = reqwest::get (
            format!("https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids{}&api_key={}",
            server, puuid, start_end, api_key)
            ).await
            .unwrap()
            .text()
            .await
            .unwrap();
        
        let mut deserialized = Deserializer::from_str(&resp);
        println!("{resp}");
        Deserialize::deserialize_in_place(&mut deserialized, &mut game_ids).unwrap();
        Ok(game_ids)
    }

    async fn get_time_range(start: &String, end: &String) -> Result<String, String> {
        let naive_date_start = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
            .map_err(|e| format!("Failed to parse date: {}", e))?;
        let naive_date_end = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
            .map_err(|e| format!("Failed to parse date: {}", e))?;

        let start_of_range = naive_date_start.and_hms_opt(0, 0, 0);
        let end_of_range = naive_date_end.and_hms_opt(23, 59, 59);

        let start_unix = match start_of_range {
            Some(x) => Utc.from_utc_datetime(&x).timestamp().to_string(),
            None => "".to_string(),
        };

        let end_unix = match end_of_range {
            Some(x) => Utc.from_utc_datetime(&x).timestamp().to_string(),
            None => "".to_string(),
        };

        Ok(format!("?startTime={start_unix}&endTime={end_unix}"))
    }

    async fn get_time_range_to_now(start: &String) -> Result<String, String> {
        let naive_date_start = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
            .map_err(|e| format!("Failed to parse date: {}", e))?;
        let start_range = naive_date_start.and_hms_opt(0, 0, 0);
        let start_unix = match start_range {
            Some(x) => Utc.from_utc_datetime(&x).timestamp().to_string(),
            None => "".to_string(),
        };
        let now = Utc::now().timestamp().to_string();
        Ok(format!("?startTime={start_unix}&endTime={now}"))
    }

    pub async fn get_game_data_by_list(ids: Vec<String>, api_key: &String, puuid: &String) -> Result<Vec<FilteredData>, Box<dyn Error>> { //gets game data of all games passed into the function
        println!("{}", "Getting player game data...".green());
        let mut out: Vec<FilteredData> = Vec::new();
        //let mut out: Vec<FilteredData> = self.get_game_data_from_ids(self.game_ids);
        let check_valid_game = |game: &MatchData| -> bool {
            if game.info.end_of_game_result != "GameComplete" || game.info.game_mode != "CLASSIC" || game.info.game_type != "MATCHED_GAME" {
                return false;
            }
            true
        };
        
        for id in &ids {
            let game = IntakeHelper::request_game_data(&id, &api_key).await.unwrap();
            if check_valid_game(&game) {
                let tl = Self::request_match_timeline(&id, &api_key).await.unwrap();
                let mut filtered = FilteredData::new(&game, &tl);
                filtered.find_me(&puuid);
                out.push(filtered);
            }
        }
        Ok(out)
    }

    pub async fn get_game_data_vec(&self) -> Result<Vec<FilteredData>, Box<dyn Error>> { //gets the game data of all game ids stored in the struct
        let mut out: Vec<FilteredData> = Vec::new();
        let check_valid_game = |game: &MatchData| -> bool {
            if game.info.end_of_game_result != "GameComplete" || game.info.game_mode != "CLASSIC" {
                return false;
            }
            true
        };

        for id in &self.game_ids {
            let game = Self::request_game_data(&id, &self.start_data.api_key).await.unwrap();
            if check_valid_game(&game) {
                let tl = Self::request_match_timeline(&id, &self.start_data.api_key).await.unwrap();
                let mut filtered = FilteredData::new(&game, &tl);
                filtered.find_me(&self.start_data.puuid);
                out.push(filtered);
            }
        } 
        Ok(out)
    }

    async fn request_game_data(id: &String, api_key: &String) -> Result<MatchData, Box<dyn Error>>{
        println!("{}", format!("Getting game {}", &id).green());
        let resp = reqwest::get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}?api_key={}", id, api_key)).await.unwrap().text().await.unwrap();
        // if *id == String::from("NA1_5245259095") {println!("{}", resp); }
        let game: MatchData = serde_json::from_str(&resp)?;
        Ok(game)
    }

    pub async fn request_match_timeline(id: &String, api_key: &String) -> Result<Timeline, Box<dyn Error>> {
        let resp = reqwest::get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}", id, api_key)).await.unwrap().text().await.unwrap();
        let out: Timeline = serde_json::from_str(&resp)?;
        Ok(out)
    }

    pub async fn request_player_data(game_name: String, tagline: String, region: String, api_key: &String) -> Result<PlayerIdent,  Box<dyn Error>> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Account {
            pub puuid: String,
            pub game_name: String,
            pub tag_line: String
        }
        let server = IntakeHelper::get_server(region.as_str()).await;
        let resp = reqwest::get
            (format!("https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}?api_key={}", &server, game_name, tagline, api_key))
            .await.unwrap().json::<Account>().await.unwrap(); 
        Ok(PlayerIdent { puuid: resp.puuid, game_name: resp.game_name, tagline: resp.tag_line, server: String::from(server) })
    }

    pub async fn request_summoner(puuid: &String, server: &String, api_key: &String) -> Result<Summoner, Box<dyn Error>> {
        let resp = reqwest::get
            (format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}?api_key={}", Self::get_legacy_server(server).await, puuid, api_key)) //figure out the server for the url. Maybe local match?
            .await
            .unwrap()
            .json::<Summoner>()
            .await
            .unwrap();
        Ok(resp)
    }
    
    pub async fn get_server(server: &str) -> &str {
        match server {
            "NA" => "americas",
            "BR" => "americas",
            "LAS" => "americas",
            "LAN" => "americas",
            "KR" => "asia",
            "JP" => "asia",
            "EUW" => "europe",
            "EUNE" => "europe",
            "ME" => "europe",
            "TR" => "europe",
            "RU" => "europe",
            "OCE" => "sea",
            "VN" => "sea",
            "TW" => "sea",
            &_ => "americas"
        }
    }
    
    pub async fn get_legacy_server(server: &str) -> &str {
        match server {
            "NA" => "na1",
            "BR" => "br1",
            "EUW" => "euw1",
            "EUNE" => "eun1",
            "OCE" => "oc1",
            &_ => "na1",
        }
    }
}
