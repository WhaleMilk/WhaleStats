use owo_colors::OwoColorize;

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
use std::str::FromStr;
use chrono::{NaiveDate, Utc, TimeZone};

pub struct IntakeHelper {
    game_ids: Vec<String>,
    start_data: StartData,
}

impl IntakeHelper {
    pub async fn new(start: &StartData) -> IntakeHelper {
        let games = Self::get_games(&start).await.unwrap();
        IntakeHelper {
            game_ids: games,
            start_data: start.clone(),
        }
    }

    async fn get_games(start: &StartData) -> Result<Vec<String>, String> {
        let mut game_ids = Vec::new();
        let start_end = Self::get_time_range(&start.start_date, &start.end_date).await.unwrap();
        let absolute_server = start.region.as_str();
        let server = match absolute_server {
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
        };
        let resp = reqwest::get
            (format!("https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids{}&api_key={}",
            server, start.puuid, start_end, start.api_key)).await.unwrap().text().await.unwrap();
        println!("{}", resp.purple());


        let mut deserializer = Deserializer::from_str(&resp);
        Deserialize::deserialize_in_place(&mut deserializer, &mut game_ids).unwrap();

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

    pub async fn get_game_data_vec(&self) -> Result<Vec<FilteredData>, Box<dyn Error>> {
        println!("{}", "Getting player game data...".green());
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
        println!("{}", "Debug: Entered match request".yellow());
        let resp = reqwest::get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}?api_key={}", id, api_key)).await.unwrap().text().await.unwrap();
        // if *id == String::from("NA1_5245259095") {println!("{}", resp); }
        let game: MatchData = serde_json::from_str(&resp)?;
        Ok(game)
    }

    pub async fn request_match_timeline(id: &String, api_key: &String) -> Result<Timeline, Box<dyn Error>> {
        println!("{}", "Debug: Entered tl request".yellow());
        let resp = reqwest::get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}", id, api_key)).await.unwrap().text().await.unwrap();
        //println!("{}", resp);
        let out: Timeline = serde_json::from_str(&resp)?;
        Ok(out)
    }

}

// use std::{sync::mpsc, thread::yield_now};
// use threadpool::ThreadPool;
// use owo_colors::OwoColorize;

// use crate::intake::match_data::MatchData;
// use crate::intake::timeline::Timeline;
// //use crate::intake::ranked_data::RankedData;
// use crate::intake::data_filter::FilteredData;

// pub mod match_data;
// pub mod ranked_data;
// pub mod timeline;
// pub mod data_filter;

// use crate::StartData;

// use serde::Deserialize;
// use serde_json::Deserializer;
// use std::error::Error;
// use chrono::{NaiveDate, Utc, TimeZone};
// use reqwest::{self, header::{ACCEPT_CHARSET, ACCEPT_LANGUAGE, ORIGIN, USER_AGENT}};
// use futures::executor::block_on;

// pub struct IntakeHelper {
//     game_ids: Vec<String>,
//     start_data: StartData,
//     client: reqwest::Client,
// }

// impl IntakeHelper {
//     pub async fn new(start: &StartData) -> IntakeHelper {
//         let client = reqwest::Client::new();
//         let games = Self::get_games(&start, &client).await.unwrap();
//         IntakeHelper {
//             game_ids: games,
//             start_data: start.clone(),
//             client: client,
//         }
//     }
//     //method for player info, ranked data, etc

//     async fn get_games(start: &StartData, client: &reqwest::Client) -> Result<Vec<String>, String> {
//         let mut game_ids = Vec::new();
//         let start_end = Self::get_time_range(&start.start_date, &start.end_date).unwrap();

//         let resp = client
//             .get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids{}&api_key={}", start.puuid, start_end, start.api_key))
//             .header(USER_AGENT, "rust-web-api-client")
//             .header(ACCEPT_LANGUAGE, "en-US,en;q=0.8")
//             .header(ACCEPT_CHARSET, "application/x-www-form-urlencoded; charset=UTF-8")
//             .header(ORIGIN, "https://developer.riotgames.com")
//             .send().await.unwrap().text().await.unwrap();

//         let mut deserializer = Deserializer::from_str(&resp);
//         Deserialize::deserialize_in_place(&mut deserializer, &mut game_ids).unwrap();

//         Ok(game_ids)
//     }

//     fn get_time_range(start: &String, end: &String) -> Result<String, String> {
//         let naive_date_start = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
//             .map_err(|e| format!("Failed to parse date: {}", e))?;
//         let naive_date_end = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
//             .map_err(|e| format!("Failed to parse date: {}", e))?;

//         let start_of_range = naive_date_start.and_hms_opt(0, 0, 0);
//         let end_of_range = naive_date_end.and_hms_opt(23, 59, 59);

//         let start_unix = match start_of_range {
//             Some(x) => Utc.from_utc_datetime(&x).timestamp().to_string(),
//             None => "".to_string(),
//         };

//         let end_unix = match end_of_range {
//             Some(x) => Utc.from_utc_datetime(&x).timestamp().to_string(),
//             None => "".to_string(),
//         };

//         Ok(format!("?startTime={start_unix}&endTime={end_unix}"))
//     }

//     pub fn get_game_data_vec(&self) -> Result<Vec<FilteredData>, Box<dyn Error>> {
//         //println!("{}", "Getting player game data...".green());
//         let mut out: Vec<FilteredData> = Vec::new();
//         let (tx, rx) = mpsc::channel();
//         let parent_filtered = FilteredData::default();
//         let parent_start = self.start_data.clone();
//         let parent_client = self.client.clone();
//         let games = self.game_ids.clone();
//         //let mut handles = vec![];

//         let worker_pool = 4;
//         let pool = ThreadPool::new(worker_pool);

//         for id in games {
//             let tx = tx.clone();
//             let mut filtered = parent_filtered.clone();
//             let start = parent_start.clone();
//             let client = parent_client.clone();

//             // let handle = tokio::task::spawn(async move || {
//             //     {
//             //         let game = Self::request_game_data(&id, &start.api_key, &client).await.unwrap();
//             //     }
//             //     yield_now().await;
//             // });
//             pool.execute(move || {
//                 let game = block_on(Self::request_game_data(&id, &start.api_key, &client)).unwrap();
//                 if !(game.info.game_duration / 60 < 15) {
//                     let tl = block_on(Self::request_match_timeline(&id, &start.api_key, &client)).unwrap();
//                     filtered = FilteredData::new(&game, &tl);
//                     filtered.find_me(&start.puuid);
//                     tx.send(filtered).unwrap();
//                 }
//             });
//         }

//         drop(tx);
//         pool.join();

//         // for handle in handles {
//         //     handle.join().unwrap();
//         // }

//         for recieved in rx {
//             out.push(recieved);
//         }
//         Ok(out)
//     }

//     async fn request_game_data(id: &String, api_key: &String, client: &reqwest::Client) -> Result<MatchData, Box<dyn Error>>{
//         //let resp = reqwest::blocking::get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}?api_key={}", id, api_key))?.text().unwrap(); 
//         let resp = client
//         .get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}?api_key={}", id, api_key))
//         .header(USER_AGENT, "rust-web-api-client")
//         .header(ACCEPT_LANGUAGE, "en-US,en;q=0.8")
//         .header(ACCEPT_CHARSET, "application/x-www-form-urlencoded; charset=UTF-8")
//         .header(ORIGIN, "https://developer.riotgames.com")
//         .send().await.unwrap().text().await.unwrap();

//         let game: MatchData = serde_json::from_str(&resp).unwrap();
//         Ok(game)
//         // if *id == String::from("NA1_5245259095") {println!("{}", resp); }
        
//     }

//     async fn request_match_timeline(id: &String, api_key: &String, client: &reqwest::Client) -> Result<Timeline, Box<dyn Error>> {
//         //let resp = reqwest::blocking::get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}", id, api_key))?.text().unwrap();

//         let resp = client
//         .get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}", id, api_key))
//         .header(USER_AGENT, "rust-web-api-client")
//         .header(ACCEPT_LANGUAGE, "en-US,en;q=0.8")
//         .header(ACCEPT_CHARSET, "application/x-www-form-urlencoded; charset=UTF-8")
//         .header(ORIGIN, "https://developer.riotgames.com")
//         .send().await.unwrap().text().await.unwrap();
            
//         let out: Timeline = serde_json::from_str(&resp)?;
//         Ok(out)
//     }

    
// }