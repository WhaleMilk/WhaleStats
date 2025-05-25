use analysis::GameStatistics;
use owo_colors::OwoColorize;
use serde::Deserialize;
use crate::intake::data_filter::FilteredData;
use crate::intake::IntakeHelper;
use crate::StartData;

pub mod analysis;

pub struct Player{ 
    raw_data: Vec<FilteredData>,
    start: StartData,
}

#[derive(Deserialize)]
pub struct PlayerIdent {
    puuid: String,
    game_name: String,
    tagline: String
}

impl Player{
    pub async fn new(run_data: StartData) -> Player {
        let intake = IntakeHelper::new(&run_data).await;
        Player {
            raw_data: intake.get_game_data_vec().await.unwrap(),
            start: run_data,
        }
    }

    pub fn get_game_stats(&self, i: usize) -> &FilteredData {
        return &self.raw_data.get(i).unwrap();
    }

    pub fn get_game_count(&self) -> usize {
        return self.raw_data.len();
    }

    pub fn get_pids(&self, i: usize) -> &[(String, String); 5] {
        return &self.raw_data.get(i).unwrap().pids;
    }

    pub async fn process_day_games(&self) -> Vec<GameStatistics> {
        let mut games: Vec<GameStatistics> = Vec::new();

        println!("{}", "Processing game data...".green());
        for game in &self.raw_data {
            println!("{}", "Processing game...".green());
            games.push(self.process(game));
        }

        return games;
    }
    //need to find specific stats such as gd15, which position player of interest is in, and 

    pub async fn build_start_data(game_name: String, tagline: String, server: String, api_key: String) -> StartData {
        let p = IntakeHelper::request_player_data(&game_name, &tagline, &server, &api_key).await.unwrap();
        todo!()
    }

    async fn search_save() {
        todo!("Check /src-tauri/profiles for json file w ith puuid name. If found, load save file, if not, ")
    }

}

//todo: 
/* 
 * 
 * 
*/

