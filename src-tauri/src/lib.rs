pub mod save;

use analyzer_core::{player::Player, StartData};
use analyzer_core::save::Save;
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::path::Path;
use chrono::{Days, Utc};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;

use crate::save::*;

/* GENERAL HELPER FUNCTIONS */

//gets Riot Games API token from a .env file
fn get_api_key() -> String {
    dotenv().ok();
    std::env::var("API_TOKEN").expect("API_TOKEN not set in .env file")
}

/* COMMAND FUNCTIONS */
    #[tauri::command]
    async fn load_player_data(raw_user: &str) -> Result<String, ()> { //incoming string requires "USERNAME_TAG_SERVER" ("WhaleMilk_PHUD_NA") formatting
        let index_file = read_indexed_profiles().unwrap();
        let l = format!("./profiles/{}.json", raw_user);
        let path = Path::new(l.as_str());
        let mut player_profile = String::new();

        let mut save = match index_file.iter().any(|e| e == raw_user) {
            true => {
                let mut player_file = OpenOptions::new().read(true).open(path).unwrap();
                player_file.read_to_string(&mut player_profile).unwrap();
                serde_json::from_str(&player_profile).unwrap()
            }
            false => {
                update_index_file(raw_user).unwrap();
                File::create(format!("./profiles/{}.json", raw_user)).unwrap();
                Player::new(raw_user, get_api_key()).await.get_player().await
            }
        };
        
        if save.is_empty_games().await {
            //load starting data for new player
            save.load_new_player().await; 
        } else {
            //load in existing data into player object, then load new games. 
            save.set_api(get_api_key()).await;
            save.load_new_games().await; 
        }
        let mut player_file = OpenOptions::new().write(true).open(path).unwrap();
        player_file.write(serde_json::to_string(&save).unwrap().as_bytes()).unwrap();
        Ok(serde_json::to_string(&save.games).unwrap())
    }

    #[tauri::command]
    async fn reload_profile_data(timestamp: i64, player: &str) -> Result<bool, ()> {
        let location = format!("./profiles/{}.json", player);
        let path = Path::new(location.as_str());
        let mut player_profile_str = String::new();
        let mut player_file = OpenOptions::new().read(true).open(path).unwrap();
        let _ = player_file.read_to_string(&mut player_profile_str);

        let mut player = Player::load_indexed_player(get_api_key(), player_profile_str).await;
        player.set_api(get_api_key()).await;
        if player.load_new_games().await{
            let mut player_file = OpenOptions::new().write(true).truncate(true).open(path).unwrap();
            player_file.write(serde_json::to_string(&player).unwrap().as_bytes()).unwrap();
            return Ok(true)
        }
        Ok(false)
    }

// #[tauri::command]
// async fn reload_profile_data(timestamp: i64, player: &str) -> Result<bool, ()> { //special types of argument structures need to be pased in for entry commands
//     let api_key = get_api_key();
//     println!("Reloading profile data for {}", player);
//     let location = format!("./profiles/{}.json", player);
//     let path = Path::new(location.as_str());
//     let mut player_profile = String::new();
//     let mut player_file = OpenOptions::new().read(true).open(path).unwrap();
//     player_file.read_to_string(&mut player_profile).unwrap();
//     let mut save: Save = serde_json::from_str(&player_profile).unwrap();
//     let date = Utc::now().naive_utc().format("%Y-%m-%d").to_string();

//     let start = StartData {
//         api_key: api_key.clone(),
//         puuid: save.info.player.puuid.clone(),
//         start_date: save.info.last_calc_date.clone(),
//         end_date: date,
//         region: save.info.player.server.clone(),
//     };

//     let mut player = Player::new(start).await;
//     //let new_games =player.get_recent_games_from_utc(timestamp + 1).await.unwrap(); 
//     //let intake = IntakeHelper::new(&start).await;
//     let new_games = IntakeHelper::get_games_utc(timestamp, &save.info.player.puuid, &save.info.player.server, &api_key).await.unwrap();
//     if new_games.is_empty() {
//         println!("No new games found, {:?}", new_games);

//         return Ok(false);
//     }
//     let new_games = IntakeHelper::get_game_data_by_list(new_games, &api_key, &save.info.player.puuid).await.unwrap();
//     player.load_raw(new_games).await;
//     let mut out_data = player.process_all_player_games().await;
//     out_data.sort_by(|v1, v2| v1.game_start.cmp(&v2.game_start));
//     player.sort_raw().await;
//     save.update_data(player.get_raw_data().await, out_data);
//     let mut player_file = OpenOptions::new().write(true).truncate(true).open(path).unwrap();
//     player_file.write(serde_json::to_string(&save).unwrap().as_bytes()).unwrap();
//     Ok(true)
// }

// #[tauri::command]
// async fn query_games_test(puuid: &str) -> Result<String, ()> {

//     let start = StartData {
//         api_key: get_api_key(),
//         puuid: String::from(puuid),
//         start_date: String::from("2025-5-20"),
//         end_date: String::from("2025-5-21"), //codysun summoner id
//         region: String::from("NA"),
//     };

//     let mut buffer = File::create(format!("./profiles/{}.json", &start.puuid)).unwrap();
//     let player = Player::new(start).await;
//     let mut out_data = player.process_all_player_games().await;
//     out_data.sort_by(|v1, v2| v1.game_start.cmp(&v2.game_start));

//     let serialized = serde_json::to_string(&out_data).unwrap();
//     match buffer.write(serialized.as_bytes()) {
//         Ok(_) => (),
//         Err(_) => (),
//     }
    
//     Ok(serialized)
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_player_data, reload_profile_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
