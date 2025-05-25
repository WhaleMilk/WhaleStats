use std::clone;

use charming::{
    component::{Axis, Legend, Title}, 
    element::{AxisType, ItemStyle, LineStyle, LineStyleType}, 
    series::{Graph, Line, Pie, PieRoseType},
    theme::Theme,
    Chart, WasmRenderer
};

use leptos::{ev::{SubmitEvent, Event}, prelude::*};
use leptos::{reactive::spawn, task::spawn_local};
use leptos::html::Input;

use analyzer_core::player::analysis::GameStatistics;

use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone)]
struct Graphs {
    pub gd15: Vec<f64>,
    pub csm: Vec<f64>,
    pub dpm: Vec<f64>,
    pub kp: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct QueryArgs<'a> {
    puuid: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_comp, set_current_comp) = signal("Landing".to_string());
    let (puuid, set_puuid) = signal("nothing".to_string());
    //signals for summoner name

    provide_context(set_current_comp);
    //provide_context(set_puuid);
    //provide_context(puuid);
    //provide_context(puuid);

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

async fn fetch_data(puuid: String) -> Graphs {
    let args = serde_wasm_bindgen::to_value(&QueryArgs {puuid: &puuid}).unwrap();
    let data = invoke("query_games_test", args).await.as_string().unwrap();
    let deserialized: Vec<GameStatistics> = serde_json::from_str(&data).unwrap();
    let mut gd: Vec<f64> = Vec::new(); let mut csm: Vec<f64> = Vec::new(); let mut dpm: Vec<f64> = Vec::new(); let mut kp: Vec<f64> = Vec::new();
    //set_data.set(deserialized);
    for game in deserialized {
        gd.push(game.gd15 as f64);
        csm.push(game.csm as f64);
        dpm.push(game.dpm as f64);
        kp.push(game.kp as f64);
    }
    Graphs {
        gd15: gd,
        csm: csm,
        dpm: dpm,
        kp: kp,
    }
}

#[component]
pub fn StatDisplay(read_puuid: ReadSignal<String>) -> impl IntoView {
    //define at top an enum for each kind of graph that enum can be used for the action to decide which graph to render

    //first thing we need to do is somehow request our data from file
    //file needs to be passed in at the start of the rout to this component
    //state?
    //we then read that file from path, serde into the save struct, and then read out of that

    //Call the command for reading save, grabbing data, etc from backend

    // let (read_gd15, set_gd15) = signal(Vec::<f32>::new());
    // let (read_csm, set_csm) = signal(Vec::new());
    // let (read_dpm, set_dpm) = signal(Vec::new());
    // let (read_kp, set_kp) = signal(Vec::new());

    let user_puuid =  read_puuid.get_untracked();

    let load_data = LocalResource::new(  move || { fetch_data(user_puuid.clone()) });

    let gd15_sig = RwSignal::new(Vec::<f64>::new());
    let csm_sig = RwSignal::new(Vec::<f64>::new());
    let dpm_sig = RwSignal::new(Vec::<f64>::new());
    let kp_sig = RwSignal::new(Vec::<f64>::new());



    // let gen_data = async move || {
    //     let user_puuid = read_puuid.get();
    //     let args = serde_wasm_bindgen::to_value(&QueryArgs {puuid: &user_puuid}).unwrap();
    //     let data = invoke("query_games_test", args).await.as_string().unwrap();
    //     let deserialized: Vec<GameStatistics> = serde_json::from_str(&data).unwrap();
    //     let mut temp_gd: Vec<f32> = Vec::new(); let mut temp_csm: Vec<f32> = Vec::new(); let mut temp_dpm: Vec<f32> = Vec::new(); let mut temp_kp: Vec<f32> = Vec::new();
    //     //set_data.set(deserialized);
    //     for game in deserialized {
    //         temp_gd.push(game.gd15 as f32);
    //         temp_csm.push(game.csm);
    //         temp_dpm.push(game.dpm);
    //         temp_kp.push(game.kp);
    //     }
    //     gd15.set(temp_gd);
    //     csm.set(temp_csm);
    //     dpm.set(temp_dpm);
    //     kp.set(temp_kp);
    // };

    //executor::block_on(fetch_data(gd15, csm, dpm, kp, user_puuid.clone()));

    //fetch_data(gd15, csm, dpm, kp, user_puuid.clone()).await;

    

    enum GraphTypes {
        GD15,
        CSM,
        DPM,
        KP,
    }
    let data = RwSignal::new(vec![150.1, 230.2, 224.3, 218.2, 135.4, 147.2, 260.2]);
    let data2 = RwSignal::new(vec![120.1, 130.0, 270.3, 144.5, 189.5, 261.5, 200.5]);
    let data3 = RwSignal::new(vec![50.3, 17.9, 42.2, 44.2, 48.2, 43.3, 79.4]);

    let graph_render = Action::new( move |input: &GraphTypes| {
        unsafe {
            let unsafe_extract_data = move |in_vec: Vec<f64>| -> Vec<f64> {
                let mut out = Vec::<f64>::new();
                for i in 0..in_vec.len(){
                    out.push(*in_vec.get_unchecked(i));
                }
                out
            };
            async move {
            let local = gd15_sig.get();
            let slice = &local[0..];
            let name = "GD@15";
            //let mut test = unsafe_extract_data(local);
            let target = LineStyle::new();
            //let test: [f32; len] = [0.0; len];
            
            let chart = Chart::new()
                .title(Title::new().text(name))
                .x_axis(Axis::new().data(vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]))//(0..local.len()).map(|i| i.to_string()).collect()))
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(Line::new().data( local/* .into_iter().map(|i| i.as_ref().unwrap()).collect()) */));
                //.series(Line::new().data(vec![75, 75, 75, 75, 75, 75, 75]))
                //.series(Line::new().line_style(target.type_(LineStyleType::Dotted)).data(vec![60, 70, 80, 90, 100, 110, 120]));

            let renderer = WasmRenderer::new(400, 250).theme(Theme::Dark);
            
            renderer.render(name,&chart).unwrap();
        }
        // let gd = move || gd15.get();
        // let cs = move || csm.get();
        // let dpm = move || dpm.get();
        // let kp = move || kp.get();
        // let (local, name) = match input {
        //         GraphTypes::GD15 => (gd15_sig.get(), "GD@15"),
        //         GraphTypes::CSM => (csm_sig.get(), "CS/M"),
        //         GraphTypes::DPM => (dpm_sig.get(), "DP/M"),
        //         GraphTypes::KP => (kp_sig.get(), "KP")
        //     };
            
        
        }});

    view!{
        <div class="graph_container">

        <Suspense fallback=move|| view! {<p>"Loading..."</p>}>
        
        // <p>
        {move || {
            load_data.get().map(|a| {
                gd15_sig.try_set(a.gd15);});
            load_data.get().map(|a| {
                csm_sig.try_set(a.csm);});
            load_data.get().map(|a| {
                dpm_sig.try_set(a.dpm);});
            load_data.get().map(|a| {
                kp_sig.try_set(a.kp);});
        }  
        }
            <div>
                <div id="GD@15" class="chart"> { graph_render.dispatch(GraphTypes::GD15); }</div>
            </div>
            <div>
                <div id="CS/M" class="chart"> { graph_render.dispatch(GraphTypes::CSM); } </div>
            </div>
            <div>
                <div id="DP/M" class="chart"> { graph_render.dispatch(GraphTypes::DPM);} </div> //DIV ID HAS TO MATCH ID OF CHART
            </div>
            <div>
            <div id="KP" class="chart"> { graph_render.dispatch(GraphTypes::KP); } </div>
        </div>
        <p> { move || read_puuid.get() }</p>
        // {move || format!("{:?}", gd15.get())}
        // </p>
        </Suspense>
        </div>
    }

}

#[component]
pub fn Landing(set_puuid: WriteSignal<String>, read_puuid: ReadSignal<String>) -> impl IntoView {

    //example code for commands

    // let (name, set_name) = signal(String::new());
    // let (greet_msg, set_greet_msg) = signal(String::new());

    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };

    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }

    //         let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };
    //let puuid = use_context::<ReadSignal<String>>().expect("Did not find read end");
    //let set_puuid = use_context::<WriteSignal<String>>().expect("Did not find setter for puuid");

    let set_current_comp = use_context::<WriteSignal<String>>().expect("Did not find setter for current comp");

    let input_element: NodeRef<Input> = NodeRef::new();

    let update_puuid = move |ev: Event| {
        let v = event_target_value(&ev);
        set_puuid.set(v);
        //page_swap.set("StatDisplay".to_string());
    };

    let swap = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input = input_element.get().expect("<input> should be mounted").value();
        set_puuid.set(input);
        set_current_comp.set("StatDisplay".to_string());
    };

    view! {
        <div class="container">
            <h1>"Welcome to WhaleStats!"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Please enter a UID"</p>

            // <form class="row" on:submit=greet>
            //     <input
            //         id="greet-input"
            //         placeholder="Enter your UID..."
            //         on:input=update_name
            //     />
            //     <button type="submit">"Go"</button>
            // </form>
            // <p>{ move || greet_msg.get() }</p>

            // <form class="row">
            //     <input 
            //         id="puuid_input"
            //         placeholder = "Enter your UID..."
            //         on:input =update_puuid
            //     />
            //     <button on:click=move |_| page_swap.set("StatDisplay".to_string())>"Go"</button>
            // </form>
            
            <form class="row" on:submit=swap>
                <input type="text" value=read_puuid node_ref=input_element/>
                <input type="submit" value="Go"/>
            </form>
            <p>"Puuid is: " {read_puuid}</p>

            // <button on:click=move |_| page_swap.set("StatDisplay".to_string())>
            //     "Swap to Display"
            // </button>
            
        </div>
    }
    //<p>{ move || greet_msg.get() }</p>
}
