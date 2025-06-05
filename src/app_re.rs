use std::fs::read;
use std::future::ready;

use charming::{
    component::{Axis, Legend, Title}, 
    element::{AxisType, ItemStyle, LineStyle, LineStyleType}, 
    series::{Graph, Line},
    theme::Theme,
    Chart, WasmRenderer
};
use leptos::{ev::SubmitEvent, html::Select, prelude::*};
use leptos::html::Input;
use leptos::logging::log;

use analyzer_core::player::analysis::GameStatistics;

use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

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

/* Entry point for the program: component loads one child component at a time depending on the status of the ReadSignal "current comp"
 * The match statement in this component is more or less the directory for every screen the WhaleStats will be able to display
 */
#[component] 
pub fn App_Re() -> impl IntoView {
    let (current_comp, set_current_comp) = signal("Landing".to_string());
    let (puuid, set_puuid) = signal("".to_string());

    provide_context(set_current_comp);

    view! {
        <div>
            {
                move || match current_comp.get().as_str() {
                    "Landing" => view! { <Landing set_puuid=set_puuid read_puuid=puuid/> }.into_any(),
                    "StatDisplay" => view! { <StatDisplay read_puuid=puuid/> }.into_any(),
                    _ => view! { <div> "Loading..." </div> }.into_any(),
                }
            }
        </div>
    }
}

/* Stat Display: The page which displayes all data collected by the analyzer-core backend. 
Takes in a ReadSignal as an argument as an identification for what player is being displayed. */
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
        let deserialized: Vec<GameStatistics> = serde_json::from_str(&data).unwrap();
        let mut last_start: i64 = 0;
        let (mut gd, mut csm, mut dpm, mut kp) = (Vec::<f64>::new(), Vec::<f64>::new(), Vec::<f64>::new(), Vec::<f64>::new());
        for game in deserialized {
            gd.push(game.gd15 as f64);
            csm.push(game.csm as f64);
            dpm.push(game.dpm as f64);
            kp.push(game.kp as f64);
            last_start = game.game_start;
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
            let renderer = WasmRenderer::new(400, 250).theme(Theme::Dark);
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
                <button on:click=move |_| { let _ = refresh.dispatch(()); }>"Refresh"</button>
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

#[component] 
pub fn Landing(set_puuid: WriteSignal<String>, read_puuid: ReadSignal<String>) -> impl IntoView {
    let set_current_comp = use_context::<WriteSignal<String>>().expect("Did not find setter");
    let username: NodeRef<Input> = NodeRef::new();
    let server: NodeRef<Select> = NodeRef::new();

    let swap = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input = username.get().expect("<input> should be mounted").value().replace("#", "_");
        let server = server.get().expect("<select> should be mounted").value();
        let out = format!("{}_{}", input, server);
        set_puuid.set(out);
        set_current_comp.set("StatDisplay".to_string());
    };

    view!{
        <div class="container">
            <h1>"Welcome to WhaleStats!"</h1>

            <p>"Please enter username and tag"</p>
            <form class="row" on:submit=swap>
                <select node_ref=server>
                    <option value="NA">"NA"</option>
                    <option value="EUW">"EUW"</option>
                    <option value="KR">"KR"</option>
                </select>
                <input type="text" value=read_puuid node_ref=username/>
                <input type="submit" value="Go"/>
            </form>
            <p>"Puuid is: " {read_puuid}</p>
        </div>
    }
}