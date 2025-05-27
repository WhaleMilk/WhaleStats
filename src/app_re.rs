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
use leptos::logging::log;

use analyzer_core::player::analysis::GameStatistics;

use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct QueryArgs<'a> {
    puuid: &'a str,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct Plots {
    pub gd: Vec<f64>,
    pub csm: Vec<f64>,
    pub dpm: Vec<f64>,
    pub kp: Vec<f64>,
}

impl Plots {
    pub fn new() -> Self {
        Plots { gd: Vec::new(), csm: Vec::new(), dpm: Vec::new(), kp: Vec::new() }
    }
}

enum GraphTypes {
    GD15,
    CSM,
    DPM,
    KP,
}

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

#[component] 
pub fn StatDisplay(read_puuid: ReadSignal<String>) -> impl IntoView {
    let user_puuid = read_puuid.get_untracked();

    let plots_sig = RwSignal::new(Plots::new()); 
    
    let gen_chart = move |dat: Vec<f64>, name: &str| -> Chart {
        Chart::new()
                    .title(Title::new().text(name))
                    .x_axis(Axis::new().data((0..dat.len()).map(|i| i.to_string()).collect()))//(0..local.len()).map(|i| i.to_string()).collect()))
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Line::new().data(dat))
    };

    let fetch_data = async move |puuid: String| -> Plots {
        let args = serde_wasm_bindgen::to_value(&QueryArgs {puuid: &puuid}).unwrap();
        let data = invoke("query_games_test", args).await.as_string().unwrap();
        let deserialized: Vec<GameStatistics> = serde_json::from_str(&data).unwrap();
        let (mut gd, mut csm, mut dpm, mut kp) = (Vec::<f64>::new(), Vec::<f64>::new(), Vec::<f64>::new(), Vec::<f64>::new());
        for game in deserialized {
            gd.push(game.gd15 as f64);
            csm.push(game.csm as f64);
            dpm.push(game.dpm as f64);
            kp.push(game.kp as f64);
        }

        Plots {gd: gd, csm: csm, dpm: dpm, kp: kp}
    };

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

    let load_data = LocalResource::new(move || { fetch_data(user_puuid.clone()) });

    Effect::new(move |_| {
        if let Some(data) = load_data.get() { 
            log!("Recieved data {:?}", data);
            plots_sig.set(data);

            log!("Rendering gd15");
            graph_render.dispatch(GraphTypes::GD15);
            log!("Rendering csm");
            graph_render.dispatch(GraphTypes::CSM);
            log!("Rendering dpm");
            graph_render.dispatch(GraphTypes::DPM);
            log!("Rendering kp");
            graph_render.dispatch(GraphTypes::KP);
        }
    });

    view! {
        <div class = "graph_container">
        {move || match load_data.get() {
            Some(_data) => {
                view! {
                    <div><div id="GD@15" class="chart"> /*{ graph_render.dispatch(GraphTypes::GD15); } */ </div> </div>
                    <div><div id="CS/M" class="chart"> /*{ graph_render.dispatch(GraphTypes::CSM); } */ </div> </div>
                    <div><div id="DP/M" class="chart"> /*{ graph_render.dispatch(GraphTypes::DPM); } */ </div> </div>
                    <div><div id="KP" class="chart"> /*{ graph_render.dispatch(GraphTypes::KP); } */ </div> </div>
                }
            }.into_any(),
            None => view! {<div><p>"Loading..."</p></div>}.into_any()
        }}
        </div>
            // <div class="graph_container"> 
            //     <Suspense fallback=move|| view! {<p>"Loading..."</p>}>
            //         {move || {
            //             let _ = Suspend::new(async move {
            //                 plots_sig.set(load_data.await);
            //             });
                        
            //             view! {
            //                 <div>
            //                     {move || {
            //                         let plots = plots_sig.get();
            //                         if !plots.gd.is_empty() {
            //                             view! {
            //                                 <>
            //                                     <div>
            //                                         <div id="GD@15" class="chart"> { graph_render.dispatch(GraphTypes::GD15)}</div>
            //                                     </div>
            //                                     <div>
            //                                         <div id="CS/M" class="chart"> { graph_render.dispatch(GraphTypes::CSM) }</div>
            //                                     </div>
            //                                     <div>
            //                                         <div id="DP/M" class="chart"> {graph_render.dispatch(GraphTypes::DPM)}</div>
            //                                     </div>
            //                                     <div>
            //                                         <div id="KP" class="chart"> { graph_render.dispatch(GraphTypes::KP) }.into_any()</div>
            //                                     </div>
            //                                 </>
            //                             }.into_any()
            //                         } else {
            //                             view! { <p>"Loading data..."</p> }.into_any()
            //                         }
            //                     }}
            //                 </div>
            //             }.into_any()
            //         }}
            //     </Suspense>
            // </div>
    }
}

#[component] 
pub fn Landing(set_puuid: WriteSignal<String>, read_puuid: ReadSignal<String>) -> impl IntoView {
    let set_current_comp = use_context::<WriteSignal<String>>().expect("Did not find setter");
    let input_element: NodeRef<Input> = NodeRef::new();

    let swap = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input = input_element.get().expect("<input> should be mounted").value();
        set_puuid.set(input);
        set_current_comp.set("StatDisplay".to_string());
    };

    view!{
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
            <form class="row" on:submit=swap>
                <input type="text" value=read_puuid node_ref=input_element/>
                <input type="submit" value="Go"/>
            </form>
            <p>"Puuid is: " {read_puuid}</p>
        </div>
    }
}