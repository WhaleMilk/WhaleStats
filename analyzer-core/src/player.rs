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
        let mut inter = Interface::new(&api_key).await;
        Player {
            ident: inter.gen_player_ident_from_string(raw_username).await,
            start_data: StartData { api_key: api_key, puuid: String::default(), start_date: 0, region: String::default() },
            games: Games::default(),
            interface: inter,
        }
    }

    pub async fn load_new_player(&mut self, start_data: StartData) {
        self.start_data = start_data.clone();
        let game_ids = self.interface.get_game_ids(&self.start_data.start_date.clone().to_string(), &self.start_data.puuid).await.unwrap();
        //check if we have fewer than 15 games, then check again with backed up timestamp

        let interface = Interface::new(&self.start_data.api_key).await;
        let raw_data = interface.get_match_data_collection(game_ids, &self.start_data.puuid).await.unwrap();
        
        todo!()
        //1) Create Games struct that loads games
        //2) pass in game ids to Games which then creates our struct natively
        //3)  
    }

    pub async fn load_indexed_player(&mut self, start_data: StartData, indexed_games_as_string: String) {
        todo!()
        //get game data as a save
        //deserialize indexed_games_as_string into save variable, load that relevant data into player
        //maybe forgo the save struct entirely and just read/write player?
    }
}