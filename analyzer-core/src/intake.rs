use std::sync::mpsc;
use threadpool::ThreadPool;
use owo_colors::OwoColorize;

use crate::intake::match_data::MatchData;
use crate::intake::timeline::Timeline;
//use crate::intake::ranked_data::RankedData;
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
    pub fn new(start: &StartData) -> IntakeHelper {
        let games = Self::get_games(&start).unwrap();
        IntakeHelper {
            game_ids: games,
            start_data: start.clone(),
        }
    }
    //method for player info, ranked data, etc

    fn get_games(start: &StartData) -> Result<Vec<String>, String> {
        let mut game_ids = Vec::new();
        let start_end = Self::get_time_range(&start.start_date, &start.end_date).unwrap();

        let resp = reqwest::blocking::get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids{}&api_key={}", start.puuid, start_end, start.api_key)).unwrap().text().unwrap();

        let mut deserializer = Deserializer::from_str(&resp);
        Deserialize::deserialize_in_place(&mut deserializer, &mut game_ids).unwrap();

        Ok(game_ids)
    }

    fn get_time_range(start: &String, end: &String) -> Result<String, String> {
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

    pub fn get_game_data_vec(&self) -> Result<Vec<FilteredData>, Box<dyn Error>> {
        println!("{}", "Getting player game data...".green());
        let mut out: Vec<FilteredData> = Vec::new();
        let (tx, rx) = mpsc::channel();
        let parent_filtered = FilteredData::default();
        let parent_start = self.start_data.clone();
        let games = self.game_ids.clone();
        //let mut handles = vec![];

        let worker_pool = 4;
        let pool = ThreadPool::new(worker_pool);

        for id in games {
            let tx = tx.clone();
            let mut filtered = parent_filtered.clone();
            let start = parent_start.clone();
            
            pool.execute(move || {
                let game = Self::request_game_data(&id, &start.api_key).unwrap();
                if !(game.info.game_duration / 60 < 15) {
                    let tl = Self::request_match_timeline(&id, &start.api_key).unwrap();
                    filtered = FilteredData::new(&game, &tl);
                    filtered.find_me(&start.puuid);
                    tx.send(filtered).unwrap();
                }
            });
            // let handle = thread::Builder::new().name(id.clone()).spawn(move || {
            //     let game = Self::request_game_data(&id, &start.api_key).unwrap();
            //     if !(game.info.game_duration / 60 < 15) {
            //         let tl = Self::request_match_timeline(&id, &start.api_key).unwrap();
            //         filtered = FilteredData::new(&game, &tl);
            //         filtered.find_me(&start.puuid);
            //         tx.send(filtered).unwrap();
            //     }
            // }).unwrap();
            // handles.push(handle);
        }

        drop(tx);
        pool.join();

        // for handle in handles {
        //     handle.join().unwrap();
        // }

        for recieved in rx {
            out.push(recieved);
        }
        Ok(out)
    }

    fn request_game_data(id: &String, api_key: &String) -> Result<MatchData, Box<dyn Error>>{
        let resp = reqwest::blocking::get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}?api_key={}", id, api_key))?.text().unwrap();
        // if *id == String::from("NA1_5245259095") {println!("{}", resp); }
        let game: MatchData = serde_json::from_str(&resp)?;
        Ok(game)
    }

    pub fn request_match_timeline(id: &String, api_key: &String) -> Result<Timeline, Box<dyn Error>> {
        let resp = reqwest::blocking::get(format!("https://americas.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}", id, api_key))?.text().unwrap();
        let out: Timeline = serde_json::from_str(&resp)?;
        Ok(out)
    }

    
}