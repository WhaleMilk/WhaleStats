mod app_re;

use app_re::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App_Re/>
        }
    })
}
