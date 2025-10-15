use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::path::Path;
use serde::{Serialize, Deserialize};
use analyzer_core::player::Player;

pub fn read_indexed_profiles() -> Result<Vec<String>, ()> {
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

pub fn update_index_file(player: &str) -> Result<(), ()> {
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

pub fn write_player_save(player: Player, path: &Path) {
    let mut player_file = OpenOptions::new().write(true).truncate(true).open(path).unwrap();
    player_file.write(serde_json::to_string(&player).unwrap().as_bytes()).unwrap();
}