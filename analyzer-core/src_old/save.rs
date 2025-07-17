use serde_derive::{Deserialize, Serialize};
use crate::player::analysis::GameStatistics;
use crate::intake::data_filter::FilteredData;
use crate::player::{PlayerIdent, Summoner};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Save {
    pub info: PlayerInfo,
    pub data: PlayerData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub player: PlayerIdent,
    pub summoner: Summoner,
    pub last_calc_date: String,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub games: Vec<FilteredData>,
    pub graph_data: Vec<GameStatistics>,
    pub max_games: i64
    //sessions: Vec<Session>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session { //averages of a given session or at least a grouping of games
    date: String,
    gd15: i32,
    csm: f32,
    dpm: f32,
    kp: f32,
}

impl PlayerData {
    fn default() -> PlayerData {
        PlayerData{ 
            games: Vec::default(),
            graph_data: Vec::default(),
            max_games: 20
        }
    }
}

impl Save{
    pub fn new(iden: PlayerIdent, summoner: Summoner, today: String) -> Save {
        Save {
            info: PlayerInfo { player: iden, summoner: summoner, last_calc_date: today },
            data: PlayerData::default()
        }
    }

    pub fn update_data(&mut self, new_filter: Vec<FilteredData>, new_graph: Vec<GameStatistics>) {
        self.data.games.extend(new_filter);
        if self.data.games.len() > self.data.max_games as usize {self.data.games.drain(.. (self.data.games.len() - (self.data.games.len() - self.data.max_games as usize) - 1));}
        self.data.graph_data.extend(new_graph);
        let drain_index = self.data.graph_data.len() - (self.data.max_games as usize);
        if self.data.graph_data.len() > self.data.max_games as usize {
            self.data.graph_data.drain(.. drain_index);
        }
    }
}

//pub fn update_data