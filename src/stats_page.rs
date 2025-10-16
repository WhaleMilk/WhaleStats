use std::future::ready;
use charming::{
    component::{Axis, Legend, Title}, 
    element::{AxisType, ItemStyle, LineStyle, LineStyleType}, 
    series::{Graph, Line},
    theme::Theme,
    Chart, WasmRenderer
};
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use leptos::prelude::*;

use analyzer_core::data_processor::Games;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct QueryArgs<'a> {
    player: &'a str,
}

#[derive(Serialize, Deserialize)]
struct RefreshArgs<'a> {
    timestamp: i64, 
    player: &'a str,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Plots {
    pub gd: Vec<f64>,
    pub csm: Vec<f64>,
    pub dpm: Vec<f64>,
    pub kp: Vec<f64>,
    pub last_game: i64
}

impl Plots {
    pub fn new() -> Self {
        Plots { gd: Vec::new(), csm: Vec::new(), dpm: Vec::new(), kp: Vec::new(), last_game: 0 }
    }
}

enum GraphTypes {
    GD15,
    CSM,
    DPM,
    KP,
}

#[component] 
pub fn StatDisplay(read_puuid: ReadSignal<String>) -> impl IntoView {
    let user_puuid = read_puuid.get_untracked();

    let plots_sig = RwSignal::new(Plots::new()); 

    // lambda that generates the Chart for later rendering
    let gen_chart = move |dat: Vec<f64>, name: &str| -> Chart {
        Chart::new()
                    .title(Title::new().text(name))
                    .x_axis(Axis::new().data((0..dat.len()).map(|i| i.to_string()).collect()))//(0..local.len()).map(|i| i.to_string()).collect()))
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Line::new().data(dat))
    };

    //async lambda that fetches the data from the backend by running the appropriate command
    let fetch_data = async move |puuid: String| -> Plots {
        let args = serde_wasm_bindgen::to_value(&QueryArgs {player: &puuid}).unwrap();
        let data = invoke("load_player_data", args).await.as_string().unwrap();
        let deserialized: Games = serde_json::from_str(&data).unwrap();
        let mut last_start: i64 = 0;
        let (mut gd, mut csm, mut dpm, mut kp) = (Vec::<f64>::new(), Vec::<f64>::new(), Vec::<f64>::new(), Vec::<f64>::new());
        for game in deserialized.games {
            gd.push(game.graph_data.gd15 as f64);
            csm.push(game.graph_data.csm as f64);
            dpm.push(game.graph_data.dpm as f64);
            kp.push(game.graph_data.kp as f64);
            last_start = game.graph_data.game_end + 1;
        }

        Plots {gd: gd, csm: csm, dpm: dpm, kp: kp, last_game: last_start}
    };

    //action which renders each graph
    let graph_render = Action::new( move |input: &GraphTypes| {
        let (local, name) = match input {
            GraphTypes::GD15 => (plots_sig.get_untracked().gd, "GD@15"),
            GraphTypes::CSM => (plots_sig.get_untracked().csm, "CS/M"),
            GraphTypes::DPM => (plots_sig.get_untracked().dpm, "DP/M"),
            GraphTypes::KP => (plots_sig.get_untracked().kp, "KP")
        };

        async move {
            let chart = gen_chart(local, name);
            let renderer = WasmRenderer::new(500, 300).theme(Theme::Dark);
            renderer.render(name, &chart).unwrap();
        }
    });

    let load_data = LocalResource::new(move || { fetch_data(user_puuid.clone()) }); //loads data. Pretty much blocks on the function call until completed

    //Effect which runs whenever signals update
    Effect::new(move |_| {
        if let Some(data) = load_data.get() { //effect subscribes to load_data
            log!("Recieved data {:?}", data);
            plots_sig.set(data);

            graph_render.dispatch(GraphTypes::GD15);
            graph_render.dispatch(GraphTypes::CSM);
            graph_render.dispatch(GraphTypes::DPM);
            graph_render.dispatch(GraphTypes::KP);
        }
    });

    let refresh = Action::new(move |_| {
        let user_puuid = read_puuid.get();
        let last_time = plots_sig.get_untracked().last_game;
        let plots = plots_sig.clone();
        let render = graph_render.clone();
        
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&RefreshArgs {timestamp: last_time, player: &user_puuid}).unwrap();
            let result = invoke("reload_profile_data", args).await;
            
            if let Some(true) = result.as_bool() {
                // If there's new data, fetch and update plots
                let new_data = fetch_data(user_puuid).await;
                plots.set(new_data);
                
                // Re-render all graphs with new data
                render.dispatch(GraphTypes::GD15);
                render.dispatch(GraphTypes::CSM);
                render.dispatch(GraphTypes::DPM);
                render.dispatch(GraphTypes::KP);
            }
        });

        ready(())
    });

    view! {
    {move || match load_data.get() {
        Some(_data) => {
            view! {
                <div class = "refresh_button">
                    <button on:click=move |_| { let _ = refresh.dispatch(()); }>"Refresh"</button>
                </div>
                <div class = "graph_container">
                    <div><div id="GD@15" class="chart"></div> </div>
                    <div><div id="CS/M" class="chart"></div> </div>
                    <div><div id="DP/M" class="chart"></div> </div>
                    <div><div id="KP" class="chart"></div> </div>
                </div>
            }
        }.into_any(),
        None => view! {<div><p>"Loading..."</p></div>}.into_any()
    }}
    }
}