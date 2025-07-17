use crate::interface::match_data::MatchData;
use crate::interface::timeline::Timeline;
use crate::player::PlayerIdent;

pub mod match_data;
pub mod timeline;
pub mod ranked_data;
pub mod user_interface;
pub mod game_interface;

pub struct Interface {
    api_key: String,
    server: String,
}

impl Interface {
    pub async fn new(api_key: String) -> Interface {
        Interface {
            api_key: api_key,
            server: String::default(),
        }
    }

    pub async fn gen_player_ident_from_string(&mut self, raw_username: &str) -> PlayerIdent {
        let p: Vec<&str> = raw_username.split("_").collect();
        self.server = String::from(self.get_server(p[2]).await);
        let ident = self.request_player_data(
            String::from(p[0]).replace(" ", "%20"), String::from(p[1])).await.unwrap();
        return ident
    }

    pub async fn get_server(&self, server: &str) -> &str {
        match server {
            "NA" => "americas",
            "BR" => "americas",
            "LAS" => "americas",
            "LAN" => "americas",
            "KR" => "asia",
            "JP" => "asia",
            "EUW" => "europe",
            "EUNE" => "europe",
            "ME" => "europe",
            "TR" => "europe",
            "RU" => "europe",
            "OCE" => "sea",
            "VN" => "sea",
            "TW" => "sea",
            &_ => "americas"
        }
    }
}