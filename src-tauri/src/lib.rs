//use analyzer_core::{player::Player, StartData};
use charming::{
    component::Legend,
    element::ItemStyle,
    series::{Pie, PieRoseType},
    Chart, WasmRenderer
};
use::tauri::Runtime;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// async fn query_games_from_id(id: &str) -> Result<String, ()> {
//     let start = StartData {
//         api_key: String::from("RGAPI-73295ec7-1230-4f6a-b109-9a9bab663f26"),
//         //puuid: String::from("etqNgHQY0OaE_LSnmSZZiYPBYNkOZQbO31cNpDsbmzTx36--Xjx7C99RgxIqBWggeaqq1o6FBNTz5Q"), //my puuid
//         puuid: String::from("O2PSg8RKd8pPk8AMFdNkOi0s85jdl6ssJuBiOKWL9ZbLDqlJM7TvhsA5yAmrkVL6yP4cFeexai5kaw"), //snowystepz
//         start_date: String::from("2025-3-7"),//1741662952247
//         end_date: String::from("2025-3-8"),  //1741227571561
//         summoner_id: String::from("sLJygQ8zhAOuhaqcsGNDqBui-sQoYxwky6XcrgFgK3pfmJk")
//     };
//     let player = Player::new(start).await;
//     let out_data = player.process_day_games();
//     Ok(format!("{:?}", out_data[0]))
// }

// #[tauri::command]
// fn set_api_key(key: &str) -> String {
//     format!("Set {} as development API key", key)
// }

#[tauri::command]
async fn draw_graph<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    let chart = Chart::new()
    .legend(Legend::new().top("bottom"))
    .series(
        Pie::new()
            .name("GRAPH TEST")
            .rose_type(PieRoseType::Radius)
            .radius(vec!["50", "250"])
            .center(vec!["50%", "50%"])
            .item_style(ItemStyle::new().border_radius(8))
            .data(vec![
                (40.0, "rose 1"),
                (38.0, "rose 2"),
                (32.0, "rose 3"),
                (30.0, "rose 4"),
                (28.0, "rose 5"),
                (26.0, "rose 6"),
                (22.0, "rose 7"),
                (18.0, "rose 8"),
            ]),
    );


    Ok(())
}


// fn bad_draw_graph(name: &str) -> WasmRenderer {
//     let chart = Chart::new()
//         .legend(Legend::new().top("bottom"))
//         .series(
//             Pie::new()
//                 .name(String::from(name))
//                 .rose_type(PieRoseType::Radius)
//                 .radius(vec!["50", "250"])
//                 .center(vec!["50%", "50%"])
//                 .item_style(ItemStyle::new().border_radius(8))
//                 .data(vec![
//                     (40.0, "rose 1"),
//                     (38.0, "rose 2"),
//                     (32.0, "rose 3"),
//                     (30.0, "rose 4"),
//                     (28.0, "rose 5"),
//                     (26.0, "rose 6"),
//                     (22.0, "rose 7"),
//                     (18.0, "rose 8"),
//                 ]),
//         );

//     let mut renderer = WasmRenderer::new(500, 500);
//     return renderer;
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, draw_graph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
