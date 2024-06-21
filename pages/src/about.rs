use leptos::*;
use leptos_meta::*;

#[component]
pub fn About() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="About"/>
        <p class="text-center">"Youtinerary is an Itinerary for You"</p>
    }
}
