use leptos::*;

#[component]
pub fn Divider() -> impl IntoView {
    view! { <div class="hidden lg:block lg:h-6 lg:w-px lg:bg-gray-900/10" aria-hidden="true"></div> }
}
