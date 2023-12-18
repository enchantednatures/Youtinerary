use leptos::*;
use leptos_meta::*;
use leptos_router::Outlet;

use crate::app::components::Navigation;

#[component]
pub fn Home() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Home"/>
        <p class="text-center">"Hello, world!"</p>
        <Navigation/>

        <Outlet/>
    }
}
