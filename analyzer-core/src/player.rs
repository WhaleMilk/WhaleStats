use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::{player, StartData};
use crate::data_processor::Games;
use crate::interface::Interface;

#[derive(Deserialize, Serialize)]
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
        let mut inter = Interface::new(&api_key).await;
        let start_of_day = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap()
                .and_local_timezone(Utc).unwrap().timestamp_millis();
        let ident = inter.gen_player_ident_from_string(raw_username).await;
        Player {
            ident: ident.clone(),
            start_data: StartData { api_key: api_key, puuid: ident.summoner.puuid.clone(), start_date: start_of_day, region: ident.server.clone() },
            games: Games::default(),
            interface: inter,
        }
    }

    pub async fn load_new_player(&mut self) {
        let game_ids = self.interface.get_game_ids(&self.start_data.start_date.clone().to_string(), &self.start_data.puuid).await.unwrap();
        //TODO: check if we have fewer than 15 games, then check again with backed up timestamp

        self.interface = Interface::new(&self.start_data.api_key).await;
        
        self.games = Games::new(self.interface.get_match_data_collection(game_ids, &self.start_data.puuid).await.unwrap()).await;
    }

    pub async fn load_indexed_player(start_data: StartData, player_as_string: String) -> Player {
        let save: Player = serde_json::from_str(&player_as_string).unwrap();
        let mut inter = Interface::new(&start_data.api_key).await;
        Player {
            ident: save.ident,
            start_data: start_data.clone(),
            games: save.games,
            interface: inter,
        }
    }

    pub async fn load_new_games(&mut self) {
        todo!()
        //probe the API for any new games, and then load those in if they exist. 
        //Drop recent games if its over max game size
    }

    pub async fn get_player(self) -> Player {
        self
    }
}