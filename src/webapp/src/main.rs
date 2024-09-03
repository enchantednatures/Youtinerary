#![warn(dead_code)]
#![warn(non_snake_case)]

use leptos::*;
use webapp::app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    mount_to_body(|| view! { <App/> })
}

