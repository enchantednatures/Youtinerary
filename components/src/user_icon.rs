use leptos::*;

use state::GlobalStateSignal;

#[component]
pub fn UserIcon() -> impl IntoView {
    let state = expect_context::<GlobalStateSignal>();

    view! {
        <button
            type="button"
            class="-m-1.5 flex items-center p-1.5"
            id="user-menu-button"
            aria-expanded="false"
            aria-haspopup="true"
        >
            <span class="sr-only">Open user menu</span>
            <img
                class="h-8 w-8 rounded-full bg-gray-50"
                src=move || {
                    state
                        .with(|s| match &s.user {
                            Some(user) => user.avatar_url.clone(),
                            None => "".to_string(),
                        })
                }

                alt=""
            />
            <span class="hidden lg:flex lg:items-center">
                <span class="ml-4 text-sm font-semibold leading-6 text-gray-900" aria-hidden="true">
                    {move || {
                        state
                            .with(|s| match &s.user {
                                Some(user) => user.name.clone(),
                                None => "".to_string(),
                            })
                    }}

                </span>
                <svg
                    class="ml-2 h-5 w-5 text-gray-400"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            </span>
        </button>
    }
}
