use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::pages::logout;
use crate::app::state::{GlobalStateSignal, LoggedInUser};
use anyhow::Result;

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
pub trait LoginService {
    const LOGIN_URL: &'static str;
    async fn login(&self) -> Result<String>;
    async fn logout(&self) -> Result<()>;
}
impl LoginService for reqwest::Client {
    const LOGIN_URL: &'static str = "http://127.0.0.1:6969/";
    async fn login(&self) -> Result<String> {
        let res = self.get(Self::LOGIN_URL).send().await?.text().await?;
        Ok(res)
    }
    async fn logout(&self) -> Result<()> {
        Ok(())
    }
}

#[component]
pub fn Login() -> impl IntoView {
    provide_meta_context();

    let async_data = create_local_resource(
        || (),
        |_| async move {
            let state = expect_context::<GlobalStateSignal>();
            let login_service: reqwest::Client = state.get().http_client;
            login_service.login().await.unwrap()
        },
    );

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
            {move || async_data.get()}
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
