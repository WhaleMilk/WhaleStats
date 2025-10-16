mod landing;
mod entry;
mod stats_page;

use entry::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Entry/>
        }
    })
}
