use leptos::*;
use leptos_router::*;
use state::GlobalStateSignal;

use crate::app::icons::AboutIcon;
use crate::app::icons::HomeIcon;
use crate::app::icons::SettingsIcon;

#[component]
fn NavElement(name: String, link: String, children: Children) -> impl IntoView {
    view! {
        <A
            href=link
            class="text-stone-200 hover:text-white hover:bg-stone-700 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
        >
            {children()}
            {name}
        </A>
    }
}

#[component]
pub fn Nav() -> impl IntoView {
    let state = expect_context::<GlobalStateSignal>();

    view! {
        <nav class="flex flex-1 flex-col">
            <ul role="list" class="flex flex-1 flex-col gap-y-7">
                <li>
                    <ul role="list" class="-mx-2 space-y-1">
                        <li>
                            <NavElement name="Home".to_string() link="/".into()>
                                <HomeIcon/>
                            </NavElement>
                        </li>
                        <li>
                            <NavElement name="About".to_string() link="/about".into()>
                                <AboutIcon/>
                            </NavElement>
                        </li>
                        <Show when=move || state.with(|s| s.user.is_none())>
                            <li>
                                <NavElement name="Sign Up".to_string() link="/signup".into()>
                                    <AboutIcon/>
                                </NavElement>
                            </li>
                            <li>
                                <NavElement name="Login".to_string() link="/login".into()>
                                    <AboutIcon/>
                                </NavElement>
                            </li>
                        </Show>
                        <Show when=move || state.with(|s| s.user.is_some())>
                            <li>
                                <NavElement
                                    name="Itineraries".to_string()
                                    link="/itineraries".into()
                                >
                                    <AboutIcon/>
                                </NavElement>
                            </li>

                            <li>
                                <NavElement name="Log Out".to_string() link="/logout".into()>
                                    <AboutIcon/>
                                </NavElement>
                            </li>

                        </Show>
                    </ul>
                </li>

                <Show when=move || state.with(|s| s.user.is_some())>
                    <li>
                        <div class="text-xs font-semibold leading-6 text-stone-200">
                            "Your itineraries"
                        </div>
                        <ul role="list" class="-mx-2 mt-2 space-y-1">
                            <li>
                                <a
                                    href="#"
                                    class="text-stone-200 hover:text-white hover:bg-stone-700 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                                >
                                    <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border border-stone-400 bg-stone-500 text-[0.625rem] font-medium text-white">
                                        H
                                    </span>
                                    <span class="truncate">Fav</span>
                                </a>
                            </li>
                        </ul>
                    </li>
                    <li class="mt-auto">
                        <a
                            href="#"
                            class="group -mx-2 flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-stone-200 hover:bg-stone-700 hover:text-white"
                        >
                            <SettingsIcon/>
                            Settings
                        </a>
                    </li>
                </Show>
            </ul>
        </nav>
    }
}
