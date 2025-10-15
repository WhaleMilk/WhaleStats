pub mod player;
pub mod interface;
pub mod data_processor;
pub mod save;

use serde_derive::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartData {
    #[serde(skip_deserializing)]
    pub api_key: String,
    #[serde(rename = "PUUID")]
    pub puuid: String,
    pub start_date: i64,
    pub region: String,
}