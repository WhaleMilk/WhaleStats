use analysis::GameStatistics;
use owo_colors::OwoColorize;
use crate::intake::data_filter::FilteredData;
use crate::intake::IntakeHelper;
use crate::StartData;

mod analysis;

pub struct Player{ 
    raw_data: Vec<FilteredData>,
    start: StartData,
}


// pub struct DataPerPos{ //weighted averages of stuff
//     games_played: i16,
//     gd15: i16,
//     csm: f32,
//     dpm: i16,
//     kp: f32,
//     win_loss: f32
// }


impl Player{
    pub fn new(run_data: StartData) -> Player {
        let intake = IntakeHelper::new(&run_data);
        Player {
            raw_data: intake.get_game_data_vec().unwrap(),
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

    pub fn process_day_games(&self) -> Vec<GameStatistics> {
        let mut games: Vec<GameStatistics> = Vec::new();

        println!("{}", "Processing game data...".green());
        for game in &self.raw_data {
            games.push(self.process(game));
        }

        return games;
    }
    //need to find specific stats such as gd15, which position player of interest is in, and 

}

//todo: 
/* 
 * 
 * 
*/

