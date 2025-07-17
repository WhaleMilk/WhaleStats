use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

use crate::interface::match_data::MatchData;
use crate::interface::timeline::Timeline;

pub mod filter;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GraphData{
    puuid: String,
    pub game_start: i64,
    pub game_end: i64,
    pub position: Position,
    pub champion: String,
    pub gd15: i32,
    pub csm: f32,
    pub dpm: f32,
    pub kp: f32,
    pub wl: bool
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RawData {
    pub pids: [(String, String); 5],
    pub me: Me, 
    pub g15: [(i32, i32); 5], // (blue, red) g@15
    pub csm: [(f32, f32); 5],
    pub dpm: [(f32, f32); 5],
    pub kp: [(f32, f32); 5],
    pub champs: [(String, String); 5],
    pub win_loss: (bool, bool),
    pub game_end: i64
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Me{
    pub side: Side,
    pub champ: String,
    pub pos: Position,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Side{
    #[default]
    BLUE,
    RED
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Position{
    #[default]
    TOP,
    JUNGLE,
    MIDDLE,
    BOTTOM,
    SUPPORT,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Game{
    graph_data: GraphData,
    raw_data: RawData,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Games{
    games: Vec<Game>
}

impl Games {
    pub fn new() -> Games {
        Games{ 
            games: Vec::<Game>::new(),
        }
    }
}