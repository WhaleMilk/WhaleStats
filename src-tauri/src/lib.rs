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
        println!("Loading player data...");
        //load the list of raw usernames stored in index.json into a vec
        let index_file = read_indexed_profiles().unwrap();
        let l = format!("./profiles/{}.json", raw_user); 
        let path = Path::new(l.as_str()); //construct the path to the user name's json file
        let mut player_profile = String::new();

        //check if the raw_user that was passed in exists as a file already or not. Different execution path depending
        let mut save = match index_file.iter().any(|e| e == raw_user) {
            true => {
                let mut player_file = OpenOptions::new().read(true).open(path).unwrap();
                player_file.read_to_string(&mut player_profile).unwrap();
                serde_json::from_str(&player_profile).unwrap()
            }
            false => {
                println!("No player {} found, creating...", raw_user);
                update_index_file(raw_user).unwrap();
                File::create(format!("./profiles/{}.json", raw_user)).unwrap();
                Player::new(raw_user, get_api_key()).await.get_player().await
            }
        };
        
        if save.is_empty_games().await {
            //load starting data for new player
            save.load_new_player().await; 
            let mut player_file = OpenOptions::new().write(true).open(path).unwrap();
            player_file.write(serde_json::to_string(&save).unwrap().as_bytes()).unwrap();
        } else {
            //load in API key to the save, since everything was already loaded in before 
            save.set_api(get_api_key()).await;
        }
        Ok(serde_json::to_string(&save.games).unwrap())
    }

    #[tauri::command]
    async fn reload_profile_data(raw_user: &str) -> Result<bool, ()> {
        let location = format!("./profiles/{}.json", raw_user);
        let path = Path::new(location.as_str()); //construct the path for file
        let mut player_profile_str = String::new();
        let mut player_file = OpenOptions::new().read(true).open(path).unwrap();
        let _ = player_file.read_to_string(&mut player_profile_str); //read the file info into a struct

        let mut player = Player::load_indexed_player(get_api_key(), player_profile_str).await;
        player.set_api(get_api_key()).await;
        if player.load_new_games().await{
            let mut player_file = OpenOptions::new().write(true).truncate(true).open(path).unwrap();
            player_file.write(serde_json::to_string(&player).unwrap().as_bytes()).unwrap();
            return Ok(true) //return a true if we found new games
        }
        Ok(false) //return false if no new games were found
    }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_player_data, reload_profile_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
