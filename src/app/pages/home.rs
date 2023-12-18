use leptos::*;
use leptos_meta::*;
use leptos_router::Outlet;

#[component]
pub fn Home() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Home"/>
        <p class="text-center">"Hello, world!"</p>

        <Outlet/>
    }
}
