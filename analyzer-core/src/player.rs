use analysis::GameStatistics;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use crate::intake::data_filter::FilteredData;
use crate::intake::IntakeHelper;
use crate::StartData;

pub mod analysis;

pub struct Player{ 
    pub raw_data: Vec<FilteredData>,
    start: StartData,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
pub struct PlayerIdent {
    pub puuid: String,
    pub game_name: String,
    pub tagline: String,
    pub server: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub id: String,
    pub account_id: String,
    pub puuid: String,
    pub profile_icon_id: i64,
    pub revision_date: i64,
    pub summoner_level: i64,
}

impl Player {
    pub async fn new(run_data: StartData) -> Player {
        Player {
            raw_data: Vec::<FilteredData>::new(),
            start: run_data,
        }
    }

    //we need: load data in from vec, load data from servers, process games from server
    pub async fn get_recent_game_ids(&mut self, new_player: bool) -> Result<Vec<String>, String> {
        let mut games = Vec::<String>::new();
        if new_player {
            games = IntakeHelper::get_games_fresh(&self.start.start_date, &self.start.api_key, &self.start.puuid, &self.start.region).await.unwrap();
            //self.games_played = games;
            Ok(games)
        } else {
            games = IntakeHelper::get_games_start_end(&self.start.start_date, &self.start.end_date, &self.start.api_key, &self.start.puuid, &self.start.region).await.unwrap();
            Ok(games)
        }
    } 

    pub async fn get_recent_games_from_utc(&mut self, utc_time: i64) -> Result<Vec<String>, String> {
        let games = IntakeHelper::get_games_utc(utc_time, &self.start.puuid, &self.start.region, &self.start.api_key).await.unwrap();
        Ok(games)
    }

    pub async fn process_games(&self, games: Vec<FilteredData>) -> Vec<GameStatistics> {
        let mut processed_games = Vec::<GameStatistics>::new();
        for game in &games {
            processed_games.push(self.process(game));
        }
        return processed_games
    }

    pub async fn process_all_player_games(&self) -> Vec<GameStatistics> {
        let mut processed_games = Vec::<GameStatistics>::new();
        for game in &self.raw_data {
            processed_games.push(self.process(game));
        }
        return processed_games
    }

    pub async fn load_raw(&mut self, data: Vec<FilteredData>) {
        self.raw_data = data;
    }

    pub async fn gen_raw(&mut self) {
        let games = self.get_recent_game_ids(true).await.unwrap();
        self.raw_data = IntakeHelper::get_game_data_by_list(games, &self.start.api_key, &self.start.puuid).await.unwrap();
        //self.raw_data = intake.get_game_data_vec().await.unwrap();
    }

    pub async fn get_raw(&self) -> Vec<FilteredData> {
        self.raw_data.clone()
    }
    
    pub async fn get_ident_from_str(player: &str, api_key: &String) -> Result<PlayerIdent, Box<dyn std::error::Error>>{ 
        let p: Vec<&str> = player.split("_").collect(); //format of USERNAME_TAG_SERVER
        let player = IntakeHelper::request_player_data(
            String::from(p[0]).replace(" ", "%20"), String::from(p[1]), String::from(p[2]), &api_key)
            .await.unwrap();
        Ok(player)
    }
    
    pub async fn get_summoner(puuid: &String, server: &String, api_key: &String) -> Result<Summoner, ()>{
        Ok(IntakeHelper::request_summoner(puuid, server, api_key).await.unwrap())
    }

    pub async fn get_raw_data(&self) -> Vec<FilteredData> {
        self.raw_data.clone()
    }

    pub async fn sort_raw(&mut self) {
        self.raw_data.sort_by(|v1, v2| v1.game_start.cmp(&v2.game_start));
    }

}

//impl Player{
//    pub async fn new(run_data: StartData) -> Player {
//        //let intake = IntakeHelper::new(&run_data).await;
//        Player {
//            //raw_data: intake.get_game_data_vec().await.unwrap()=,
//            raw_data: Vec::<FilteredData>::new(),
//            start: run_data,
//        }
//    }
//
//    pub async fn load_raw(&mut self, data: Vec<FilteredData>) {
//        self.raw_data = data;
//    }
//
//    
//    // pub async fn fetch_new_games(&mut self) -> Vec<FilteredData> {
//    //     let new_start = match self.raw_data.last() {
//    //         Some(game) => game.game_start + 10,
//    //         None => { 
//    //             self.gen_raw().await; 
//    //             -1 }
//    //     };
//
//    //     if new_start < 0 {
//    //         return
//    //     }
//
//    //     let new_games = IntakeHelper::get_games_utc(new_start, &self.start.puuid, &self.start.region, &self.start.api_key).await.unwrap();
//    //     IntakeHelper::
//    // }
//
//    pub fn get_game_stats(&self, i: usize) -> &FilteredData {
//        return &self.raw_data.get(i).unwrap();
//    }
//
//    pub fn get_game_count(&self) -> usize {
//        return self.raw_data.len();
//    }
//
//    pub fn get_pids(&self, i: usize) -> &[(String, String); 5] {
//        return &self.raw_data.get(i).unwrap().pids;
//    }
//
//    pub async fn process_day_games(&self) -> Vec<GameStatistics> {
//        let mut games: Vec<GameStatistics> = Vec::new();
//
//        println!("{}", "Processing game data...".green());
//        for game in &self.raw_data {
//            println!("{}", "Processing game...".green());
//            games.push(self.process(game));
//        }
//
//        return games;
//    }
//    //need to find specific stats such as gd15, which position player of interest is in, and 
//
//}

//todo: 
/* 
 * 
 * 
*/
