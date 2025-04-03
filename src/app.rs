use charming::{
    component::{Legend, Axis, Title},
    element::{ItemStyle, AxisType},
    series::{Pie, PieRoseType, Line},
    Chart, WasmRenderer
};
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use leptos_use::utils::Pausable;
use leptos_use::use_interval_fn;

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
    // let (name, set_name) = signal(String::new());
    // let (greet_msg, set_greet_msg) = signal(String::new());
    // let renderer = RwSignal::new(WasmRenderer::new(500, 500));
    // let (chart, set_chart) = signal(Chart::new());

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

    // let graph = move|ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local( async move {
    //         let chart = Chart::new()
    //         .legend(Legend::new().top("bottom"))
    //         .series(
    //             Pie::new()
    //                 .name("GRAPH TEST")
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

    //         //set_chart.set(chart);

    //         let renderer = WasmRenderer::new(500, 500);
    //         renderer.render("chart", &chart).unwrap();
    //     });
    // };

    // let pie = Action::new(move |_input: &()| {
    //     async move {
    //         let chart = Chart::new()
    //         .legend(Legend::new().top("bottom"))
    //         .series(
    //             Pie::new()
    //                 .name("GRAPH TEST")
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
    
    //         let renderer = WasmRenderer::new(500, 500);
    //         renderer.render("chart", &chart).unwrap();
    //     }
    // });

    let data = RwSignal::new(vec![150, 230, 224, 218, 135, 147, 260]);
    let action = Action::new(move |_input: &()| {
        let local = data.get();
        async move {
            let chart = Chart::new()
                .title(Title::new().text("Demo: Leptos + Charming"))
                .x_axis(
                    Axis::new()
                        .type_(AxisType::Category)
                        .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                )
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(Line::new().data(local));

            let renderer = WasmRenderer::new(600, 600);
            renderer.render("chart",&chart).unwrap();
    }});

    // let Pausable { pause, resume, is_active: _ } = use_interval_fn(
    //     move || {
    //         data.update(|d| d.rotate_right(1));
    //         action.dispatch(());
    //     },
    //     1000
    // );
    

    //action.dispatch(());
    
//            <button on:click=move |_| pause()>"Pause"</button>
//             <button on:click=move |_| resume()>"Resume"</button>

    view!{
        <div>
            <div id="chart"> { action.dispatch(()); }</div>

        </div>
    }

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

    //         <form class="row" on:submit=graph>
    //             <input
    //                 id="greet-input"
    //                 placeholder="Enter your UID..."
    //                 on:input=update_name
    //             />
    //             <button type="submit">"Go"</button>
    //         </form>
    //         <p>{ move || pie.dispatch(()); }</p>
    //     </main>
    // }
    //<p>{ move || greet_msg.get() }</p>
}
