use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::pages::logout;
use crate::app::state::{GlobalStateSignal, LoggedInUser};

use web_sys::MouseEvent;

pub fn login() {
    let state = expect_context::<GlobalStateSignal>();

    state.update(|s| {
    s.user = Some(LoggedInUser {
        id: 1,
        email: "tcook@mail.com".to_string(),
        name: "Tom Cook".to_string(),
        username: "tcook".to_string(),
        avatar_url: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80".to_string(),
    })
});
}

#[component]
pub fn Login() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="Login"/>
        <button
            type="button"
            class="rounded bg-indigo-600 px-2 py-1 text-xs font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            on:click=move |_ev: MouseEvent| {
                login();
                let navigate = use_navigate();
                navigate("/", NavigateOptions::default());
            }
        >

            Login
        </button>
    }
}

#[component]
pub fn LogOut() -> impl IntoView {
    view! {
        <button
            type="button"
            class="rounded bg-indigo-600 px-2 py-1 text-xs font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            on:click=move |_ev: MouseEvent| {
                logout();
                let navigate = use_navigate();
                navigate("/signup", NavigateOptions::default());
            }
        >

            Log Out
        </button>
    }
}
