use serde_derive::{Deserialize, Serialize};

use crate::{data_processor::{GraphData, RawData, Games}, player::PlayerIdent};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Save{
    pub player_info: PlayerIdent,
    pub data: PlayerData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerData {
    pub raw_data: Vec<RawData>,
    pub graph_data: Vec<GraphData>,
    pub max_games: i64
}

impl PlayerData {
    fn default() -> PlayerData {
        PlayerData { raw_data: Vec::default(), graph_data: Vec::default(), max_games: 20 }
    }
}

//other structs for doing the averaging/grouping stuff

impl Save {
    pub fn new(iden: PlayerIdent) -> Save {
        Save{
            player_info: iden,
            data: PlayerData::default()
        }
    }

    pub fn update_data(&mut self, games: Games) {
        todo!()
    }
}
