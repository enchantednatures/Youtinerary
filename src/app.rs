use std::time::Duration;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod command_pallet;
mod components;
mod icons;
mod nav;

use nav::Nav;

mod pages;
use pages::{About, Home, LogOut, Login, Signup};

mod header;
use header::Header;
use icons::LogoIcon;

use components::CreateItineraryForm;

mod state;

use state::GlobalState;
use state::GlobalStateSignal;
use components::ShowItnerarySignal;
use components::CreateItinerarySlideOut;

pub type ShowCommandPalletSignal = RwSignal<bool>;

pub fn is_logged_in() -> bool {
    let state = expect_context::<GlobalStateSignal>();
    state.with(|s| s.user.is_some())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let show_command_pallet: ShowCommandPalletSignal = create_rw_signal(false);
    let show_itinerary: ShowItnerarySignal = create_rw_signal(false);
    let state: GlobalStateSignal = create_rw_signal(GlobalState::default());
    provide_context(state);
    provide_context(show_command_pallet);
    provide_context(show_itinerary);

    // let _ = use_event_listener(use_window(), keydown, |evt| {
    //     console_log(format!("window keydown: '{}'", evt.key()).as_str());
    // });

    // let eh = leptos::ev::EventHandler::new()
    //     .on_keydown(|evt| {
    //         console_log(format!("window keydown: '{}'", evt.key()).as_str());
    //     });

    view! {
        <Title formatter=|text| format!("{text} — Youtinerary")/>
        <Router>
            <div>
                <div class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col">
                    <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-stone-600 px-6 pb-4">
                        <div class="flex h-16 shrink-0 items-center">
                            // <img class="h-8 w-auto" src="/logo.png" alt="Your Company"/>

                            <LogoIcon/>
                        </div>

                        <Nav/>
                    </div>
                </div>

                <div class="lg:pl-72">
                    <Header/>
                    <main class="py-10">
                        <AnimatedShow
                            when=show_command_pallet
                            // optional CSS class which will be applied if `when == true`
                            show_class="ease-out duration-900 opacity-100"
                            // optional CSS class which will be applied if `when == false` and before the
                            // `hide_delay` starts -> makes CSS unmount animations really easy
                            hide_class="ease-in duration-300 opacity-0"
                            // the given unmount delay which should match your unmount animation duration
                            hide_delay=Duration::from_millis(300)
                        >
                            <command_pallet::CommandPallet></command_pallet::CommandPallet>
                        </AnimatedShow>
                        <Routes>
                            <Route path="/" view=Home>
                                <Route path="create" view=CreateItinerarySlideOut/>
                                <Route path="new_itinerary" view=CreateItineraryForm/>
                            </Route>
                            <Route path="/about" view=About/>
                            <Route path="/login" view=Login/>
                            <Route path="/logout" view=LogOut/>
                            <Route path="/signup" view=Signup/>
                        </Routes>
                        <div class="px-4 sm:px-6 lg:px-8"></div>
                    </main>
                </div>
            </div>
        </Router>
    }
}
