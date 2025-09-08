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

impl Game{
    pub async fn new(graph: GraphData, raw: RawData) -> Game {
        Game { graph_data: graph, raw_data: raw }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Games{
    games: Vec<Game>
}

impl Games {
    pub async fn new(raw_data: Vec<RawData>) -> Games {
        //for each item in raw_data, generate a relevant graph data vec, then insert into game vec
        let mut games = Vec::<Game>::new();
        for data in raw_data {
            let game_graph_data = Games::pull_graph_data(&data).await;
            games.push(Game::new(game_graph_data, data).await);
        }
        Games{ 
            games: games
        }
    }

    pub async fn pull_graph_data(data: &RawData) -> GraphData {
        
        // GraphData {
        //     puuid: 
        // }
        todo!();
    }

    async fn calc_gd(&self, game: &RawData, pos: &Position, side: &Side) -> i32 {
        todo!()
    }
    
    async fn find_csm(&self, game: &RawData, pos: &Position, side: &Side) -> f32 {
        todo!()
    }

    async fn find_dpm(&self, game: &RawData, pos: &Position, side: &Side) -> f32 {
        todo!()
    }
    
    async fn find_kp(&self, game: &RawData, pos: &Position, side: &Side) -> f32 {
        todo!()
    }
    
    async fn find_wl(&self, game: &RawData, pos: &Position, side: &Side) -> bool {
        todo!()
    }

    fn get_index_from_pos(pos: &Position) -> usize {
        return match pos {
            Position::TOP => 0,
            Position::JUNGLE => 1,
            Position::MIDDLE => 2,
            Position::BOTTOM => 3,
            Position::SUPPORT => 4
        };
    }
}