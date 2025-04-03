

pub mod player;
pub mod intake;
mod save;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartData {
    pub api_key: String,
    #[serde(rename = "PUUID")]
    pub puuid: String,
    pub start_date: String,
    pub end_date: String,
    #[serde(rename = "summonerID")]
    pub summoner_id: String,
}
