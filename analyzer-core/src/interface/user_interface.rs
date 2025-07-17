use crate::interface::Interface;
use crate::player::{PlayerIdent, Summoner};

use serde::Deserialize;
use std::error::Error;

impl Interface {
    pub async fn request_player_data(&self, game_name: String, tagline: String) -> Result<PlayerIdent, Box<dyn Error>> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Account {
            pub puuid: String,
            pub game_name: String,
            pub tag_line: String
        } 
        let account = reqwest::get
            (format!("https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}?api_key={}", self.server, game_name, tagline, self.api_key))
            .await.unwrap().json::<Account>().await.unwrap(); 

        let summ = reqwest::get
            (format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}?api_key={}", Self::get_legacy_server(self.server.as_str()).await, account.puuid, self.api_key)) //figure out the server for the url. Maybe local match?
            .await
            .unwrap()
            .json::<Summoner>()
            .await
            .unwrap();

        Ok(PlayerIdent { summoner: summ, game_name: account.game_name, tagline: account.tag_line, server: self.server.clone()})
    }

    pub async fn get_legacy_server(server: &str) -> &str {
        match server {
            "NA" => "na1",
            "BR" => "br1",
            "EUW" => "euw1",
            "EUNE" => "eun1",
            "OCE" => "oc1",
            &_ => "na1",
        }
    }
}