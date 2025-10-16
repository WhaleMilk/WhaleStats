use leptos::prelude::*;

use crate::landing::*;
use crate::stats_page::*;

#[component]
pub fn Entry() -> impl IntoView {
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