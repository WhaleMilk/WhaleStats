use charming::{
    component::{Axis, Legend, Title}, 
    element::{AxisType, ItemStyle, LineStyle, LineStyleType}, 
    series::{Graph, Line, Pie, PieRoseType},
    theme::Theme,
    Chart, WasmRenderer
};
use leptos::{reactive::spawn, task::spawn_local};
use leptos::{ev::{SubmitEvent, MouseEvent}, prelude::*};
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes, A};
use leptos_router::hooks::use_params_map;
use leptos_router::path;

// use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes, A};
// use leptos_router::hooks::use_params_map;
// use leptos_router::path;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}



#[component]
pub fn App() -> impl IntoView {
    let (current_comp, set_current_comp) = signal("Landing".to_string());

    provide_context(set_current_comp);

    view! {
        <div>
            {
                move || match current_comp.get().as_str() {
                    "Landing" => view! { <Landing/> }.into_any(),
                    "StatDisplay" => view! { <StatDisplay/> }.into_any(),
                    _ => view! { <div> "Loading..." </div> }.into_any(),
                }
            }
        </div>
    }
}

#[component]
pub fn StatDisplay() -> impl IntoView {
    //define at top an enum for each kind of graph
    //that enum can be used for the action to decide which graph to render

    //first thing we need to do is somehow request our data from file
    //file needs to be passed in at the start of the rout to this component
    //state?
    //we then read that file from path, serde into the save struct, and then read out of that

    enum GraphTypes {
        GD15,
        CSM,
        DPM,
        KP,
    }
    let data = RwSignal::new(vec![150, 230, 224, 218, 135, 147, 260]);
    let data2 = RwSignal::new(vec![120, 130, 270, 144, 189, 261, 200]);
    let data3 = RwSignal::new(vec![50, 17, 42, 44, 48, 43, 79]);

    let graph_render = Action::new( move |input: &GraphTypes| {
        let (local, name) = match input {
            GraphTypes::GD15 => (data.get_untracked(), "GD@15"),
            GraphTypes::CSM => (data2.get_untracked(), "CS/M"),
            GraphTypes::DPM => (data3.get_untracked(), "DP/M"),
            GraphTypes::KP => (data3.get_untracked(), "KP")
        };
        async move {
            let target = LineStyle::new();
            
            let chart = Chart::new()
                .title(Title::new().text(name))
                .x_axis(
                    Axis::new()
                        .type_(AxisType::Category)
                        .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                )
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(Line::new().data(local))
                .series(Line::new().data(vec![75, 75, 75, 75, 75, 75, 75]))
                .series(Line::new().line_style(target.type_(LineStyleType::Dotted)).data(vec![60, 70, 80, 90, 100, 110, 120]));

            let renderer = WasmRenderer::new(400, 250).theme(Theme::Dark);
            
            renderer.render(name,&chart).unwrap();
        }});

    view!{
        <div class="graph_container">
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
        </div>
    }

}

#[component]
pub fn Landing() -> impl IntoView {

    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());
    let renderer = RwSignal::new(WasmRenderer::new(500, 500));
    let (chart, set_chart) = signal(Chart::new());

    let page_swap = use_context::<WriteSignal<String>>().expect("Did not find setter");

    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    //let navigator = leptos_router::hooks::use_navigate();

    view! {
        //<Router>
        
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
            // <A href="/stat">
            //     <button> "Swap to Display"</button>
            // </A>
            <button on:click=move |_| page_swap.set("StatDisplay".to_string())>
                "Swap to Display"
            </button>
            
        </div>
        //</Router>
    }
    //<p>{ move || greet_msg.get() }</p>
}
