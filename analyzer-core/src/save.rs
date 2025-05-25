use serde_json::Serializer;
use serde_derive::{Deserialize, Serialize};
use crate::player::analysis::GameStatistics;
use crate::intake::data_filter::FilteredData;
use crate::StartData;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Save {
    info: PlayerInfo,
    data: PlayerData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerInfo {
    summoner_name: String,
    tagline: String,
    puuid: String,
    last_calc_date: String,
    region: String,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    games: Vec<FilteredData>,
    graph_data: Vec<GameStatistics>,
    sessions: Vec<Session>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session { //averages of a given session or at least a grouping of games
    date: String,
    gd15: i32,
    csm: f32,
    dpm: f32,
    kp: f32,
}

pub fn create_save() -> Save {
    //gather all relevant info from start info, etc
    todo!()
}