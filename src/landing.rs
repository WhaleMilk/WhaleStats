use leptos::{ev::SubmitEvent, html::{Select, Input}, prelude::*};

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