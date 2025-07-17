use analyzer_core::{player::Player, StartData};
use analyzer_core::save::Save;
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::path::Path;
use chrono::{Days, Utc};
use serde::{Serialize, Deserialize};
use analyzer_core::intake::IntakeHelper;
use dotenv::dotenv;

//gets Riot Games API token from a .env file
fn get_api_key() -> String {
    dotenv().ok();
    std::env::var("API_TOKEN").expect("API_TOKEN not set in .env file")
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command] 
async fn load_player_data(player: &str) -> Result<String, ()> { //incoming string should be formatted like "USERNAME_TAG_SERVER" ("WhaleMilk_PHUD_NA")
    let api_key = get_api_key();
    //check if we have saved info for player; if we do, load that info instead, if we don't, make new player ident
    let index_file = read_indexed_profiles().unwrap();
    let location = format!("./profiles/{}.json", player);
    let path = Path::new(location.as_str());
    let mut player_profile = String::new();
    let date = Utc::now().naive_utc().format("%Y-%m-%d").to_string();
    
    let mut save = match index_file.iter().any(|e| e == player) {
        true => { //if the player file exists, load that data into the save variable
            println!("Found {} in index file", player);
            let mut player_file = OpenOptions::new().read(true).open(path).unwrap();
            player_file.read_to_string(&mut player_profile).unwrap();
            serde_json::from_str(&player_profile).unwrap()
        }
        false => { //if the player file does not exist, add it to index.json and create the file, then create start structures from scratch.
            println!("{} not found in index file contianing {:?}", player, index_file);
            update_index_file(player).unwrap();
            File::create(format!("./profiles/{}.json", player)).unwrap();
            let player_iden = Player::get_ident_from_str(player, &api_key).await.unwrap();
            let summoner = Player::get_summoner(&player_iden.puuid, &player_iden.server, &api_key).await.unwrap();
            let last_calc = match Utc::now().naive_utc().checked_sub_days(Days::new(2)) {
                Some(n) => n.format("%Y-%m-%d").to_string(),
                None => date.clone()
            };
            Save::new(player_iden, summoner, last_calc)
        }
    };
    
    let start = StartData {
        api_key: api_key,
        puuid: save.info.player.puuid.clone(),
        start_date: save.info.last_calc_date.clone(),
        end_date: date,
        summoner_id: save.info.summoner.id.clone(),
        region: save.info.player.server.clone(),
    };

    let mut player = Player::new(start).await;
    if !save.data.games.is_empty() { //if we are working with a populated save file, load that data into the player
            player.load_raw(save.data.games.clone()).await;
    } else { //otherwise, generate save data from scratch
        player.gen_raw().await;
        let mut out_data = player.process_all_player_games().await;
        out_data.sort_by(|v1, v2| v1.game_start.cmp(&v2.game_start));
        player.sort_raw().await;
        save.update_data(player.get_raw_data().await, out_data);
    }
    let mut player_file = OpenOptions::new().write(true).open(path).unwrap();
    player_file.write(serde_json::to_string(&save).unwrap().as_bytes()).unwrap();
    
    Ok(serde_json::to_string(&save.data.graph_data).unwrap())
}

#[tauri::command]
async fn reload_profile_data(timestamp: i64, player: &str) -> Result<bool, ()> { //special types of argument structures need to be pased in for enty commands
    let api_key = get_api_key();
    println!("Reloading profile data for {}", player);
    let location = format!("./profiles/{}.json", player);
    let path = Path::new(location.as_str());
    let mut player_profile = String::new();
    let mut player_file = OpenOptions::new().read(true).open(path).unwrap();
    player_file.read_to_string(&mut player_profile).unwrap();
    let mut save: Save = serde_json::from_str(&player_profile).unwrap();
    let date = Utc::now().naive_utc().format("%Y-%m-%d").to_string();


    let start = StartData {
        api_key: api_key.clone(),
        puuid: save.info.player.puuid.clone(),
        start_date: save.info.last_calc_date.clone(),
        end_date: date,
        summoner_id: save.info.summoner.id.clone(),
        region: save.info.player.server.clone(),
    };

    let mut player = Player::new(start).await;
    //let new_games =player.get_recent_games_from_utc(timestamp + 1).await.unwrap(); 
    //let intake = IntakeHelper::new(&start).await;
    let new_games = IntakeHelper::get_games_utc(timestamp, &save.info.player.puuid, &save.info.player.server, &api_key).await.unwrap();
    if new_games.is_empty() {
        println!("No new games found, {:?}", new_games);

        return Ok(false);
    }
    let new_games = IntakeHelper::get_game_data_by_list(new_games, &api_key, &save.info.player.puuid).await.unwrap();
    player.load_raw(new_games).await;
    let mut out_data = player.process_all_player_games().await;
    out_data.sort_by(|v1, v2| v1.game_start.cmp(&v2.game_start));
    player.sort_raw().await;
    save.update_data(player.get_raw_data().await, out_data);
    let mut player_file = OpenOptions::new().write(true).truncate(true).open(path).unwrap();
    player_file.write(serde_json::to_string(&save).unwrap().as_bytes()).unwrap();
    Ok(true)
}

#[tauri::command]
async fn query_games_test(puuid: &str) -> Result<String, ()> {

    let start = StartData {
        api_key: String::from("RGAPI-94ec128d-e0c9-47a5-a6e4-ce86d04755d7"),
        puuid: String::from(puuid),
        start_date: String::from("2025-5-20"),
        end_date: String::from("2025-5-21"),
        summoner_id: String::from("BkwWsZp4D0KbrXr7YTmoLFRDVD8Y8UoJz4HZ7sPQAPej"), //codysun summoner id
        region: String::from("NA"),
    };

    let mut buffer = File::create(format!("./profiles/{}.json", &start.puuid)).unwrap();
    let player = Player::new(start).await;
    let mut out_data = player.process_all_player_games().await;
    out_data.sort_by(|v1, v2| v1.game_start.cmp(&v2.game_start));

    let serialized = serde_json::to_string(&out_data).unwrap();
    match buffer.write(serialized.as_bytes()) {
        Ok(_) => (),
        Err(_) => (),
    }
    
    Ok(serialized)
}

fn read_indexed_profiles() -> Result<Vec<String>, ()> {
    #[derive(Serialize, Deserialize, Debug)]
    struct Temp{
        profiles: Vec<String>,
    }
    let mut buf = String::new();
    OpenOptions::new().read(true).open("./profiles/index.json").unwrap().read_to_string(&mut buf).unwrap();
    let indexes: Temp = match serde_json::from_str::<Temp>(&buf) {
        Ok(n) => n,
        Err(_) => Temp { profiles: Vec::new() }
    };
    println!("{:?}", indexes.profiles);
    Ok(indexes.profiles)
}

fn update_index_file(player: &str) -> Result<(), ()> {
    #[derive(Serialize, Deserialize)]
    struct Temp{
        profiles: Vec<String>,
    }
    let mut indexed = read_indexed_profiles().unwrap();
    indexed.push(String::from(player));
    let updated = Temp { profiles: indexed };
    let serialized = serde_json::to_string(&updated).unwrap();
    let mut file = OpenOptions::new().write(true).truncate(true).open("./profiles/index.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![query_games_test, greet, load_player_data, reload_profile_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
