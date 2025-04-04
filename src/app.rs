use charming::{
    component::{Axis, Legend, Title}, element::{AxisType, ItemStyle}, renderer::Echarts, series::{Graph, Line, Pie, PieRoseType}, Chart, WasmRenderer
};
use leptos::{reactive::spawn, task::spawn_local};
use leptos::{ev::SubmitEvent, prelude::*};
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
    //define at top an enum for each kind of graph
    //that enum can be used for the action to decide which graph to render
    enum GraphTypes {
        GD15,
        CSM,
        DPM,
    }
    let data = RwSignal::new(vec![150, 230, 224, 218, 135, 147, 260]);
    let data2 = RwSignal::new(vec![120, 130, 270, 144, 189, 261, 200]);
    let data3 = RwSignal::new(vec![50, 17, 42, 44, 48, 43, 79]);

    let graph_render = Action::new( move |input: &GraphTypes| {
        let (local, name) = match input {
            GraphTypes::GD15 => (data.get(), "GD@15"),
            GraphTypes::CSM => (data2.get(), "CS/M"),
            GraphTypes::DPM => (data3.get(), "DP/M"),
        };
        async move {
            let chart = Chart::new()
                .title(Title::new().text(name))
                .x_axis(
                    Axis::new()
                        .type_(AxisType::Category)
                        .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                )
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(Line::new().data(local));

            let renderer = WasmRenderer::new(400, 250);
            
            renderer.render(name,&chart).unwrap();
        }});

    view!{
        <main class="graph_container">
            <div>
                <div id="GD@15"> { graph_render.dispatch(GraphTypes::GD15); }</div>
            </div>
            <div>
                <div id="CS/M"> { graph_render.dispatch(GraphTypes::CSM); } </div>
            </div>
            <div>
                <div id="DP/M"> { graph_render.dispatch(GraphTypes::DPM);} </div> //DIV ID HAS TO MATCH ID OF CHART
            </div>
        </main>
    }

}

#[component]
pub fn NotApp() -> impl IntoView {

    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());
    let renderer = RwSignal::new(WasmRenderer::new(500, 500));
    let (chart, set_chart) = signal(Chart::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

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


    // view! {
    //     <main class="container">
    //         <h1>"Welcome to WhaleStats!"</h1>

    //         <div class="row">
    //             <a href="https://tauri.app" target="_blank">
    //                 <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
    //             </a>
    //             <a href="https://docs.rs/leptos/" target="_blank">
    //                 <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
    //             </a>
    //         </div>
    //         <p>"Please enter a UID"</p>

    //         <form class="row" on:submit=greet>
    //             <input
    //                 id="greet-input"
    //                 placeholder="Enter your UID..."
    //                 on:input=update_name
    //             />
    //             <button type="submit">"Go"</button>
    //         </form>
    //         <p></p>
    //     </main>
    // };

    //https://book.leptos.dev/islands.html

    view! {
        //<Router>
        
        <main class="container">
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

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter your UID..."
                    on:input=update_name
                />
                <button type="submit">"Go"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>
        </main>
        //</Router>
    }
    //<p>{ move || greet_msg.get() }</p>
}
