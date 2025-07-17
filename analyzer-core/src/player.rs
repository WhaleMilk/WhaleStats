use serde::{Deserialize, Serialize};
use crate::StartData;
use crate::data_processor::Games;
use crate::interface::Interface;

pub struct Player {
    pub ident: PlayerIdent,
    pub start_data: StartData,
    pub games: Games,
    pub interface: Interface,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
pub struct PlayerIdent{
    pub summoner: Summoner,
    pub game_name: String,
    pub tagline: String,
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub puuid: String,
    pub profile_icon_id: i64,
    pub revision_date: i64,
    pub summoner_level: i64,
}

impl Player {
    pub async fn new(raw_username: &str, api_key: String) -> Player {
        let mut inter = Interface::new(api_key.clone()).await;
        Player {
            ident: inter.gen_player_ident_from_string(raw_username).await,
            start_data: StartData { api_key: api_key, puuid: String::default(), start_date: String::default(), region: String::default() },
            games: Games::default(),
            interface: inter,
        }
    }

    pub fn load_new_player(start_data: StartData) {
        todo!()
    }

    pub fn load_indexed_player(start_data: StartData, indexed_games_as_string: String) {
        todo!()
    }
}