use pages::ItineraryTravelLegs;
use pages::ItineraryView;
use pages::ItineraryStays;
use pages::ItinerariesView;
use components::CreateItinerarySlideOut;
use components::ShowItinerarySignal;
use pages::TravelOutlet;
use std::time::Duration;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod auth;
mod command_pallet;
mod icons;
mod nav;

use nav::Nav;

use pages::{About, Home, LogOut, Login, Signup};

mod header;
use header::Header;
use icons::LogoIcon;


use state::GlobalState;
use state::GlobalStateSignal;

use command_pallet::ShowCommandPalletSignal;

use crate::app::command_pallet::CommandPallet;

pub fn is_logged_in() -> bool {
    let state = expect_context::<GlobalStateSignal>();
    state.with(|s| s.user.is_some())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let show_command_pallet = create_rw_signal(false);

    let show_itinerary = create_rw_signal(false);
    let state = create_rw_signal(GlobalState::default());

    provide_context(state);
    provide_context(ShowItinerarySignal::new(show_itinerary));
    provide_context(ShowCommandPalletSignal::new(
        show_command_pallet.write_only(),
    ));

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
                            <CommandPallet/>
                        </AnimatedShow>

                        <AnimatedShow
                            when=show_itinerary
                            // optional CSS class which will be applied if `when == true`
                            show_class="ease-out duration-900 opacity-100"
                            // optional CSS class which will be applied if `when == false` and before the
                            // `hide_delay` starts -> makes CSS unmount animations really easy
                            hide_class="ease-in duration-300 opacity-0"
                            // the given unmount delay which should match your unmount animation duration
                            hide_delay=Duration::from_millis(300)
                        >
                            <CreateItinerarySlideOut/>
                        </AnimatedShow>
                        <Routes>
                            <Route path="/" view=Home/>
                            <Route path="/about" view=About/>
                            <Route path="/login" view=Login/>
                            <Route path="/logout" view=LogOut/>
                            <Route path="/signup" view=Signup/>
                            // <Route path="/itineraries" view=ItinerariesView/>
                            <Route path="/itineraries" view=TravelOutlet>
                                <ItineraryInfoRoutes/>
                                <Route path="" view=ItinerariesView/>
                            </Route>
                        </Routes>
                        <div class="px-4 sm:px-6 lg:px-8"></div>
                    </main>
                </div>
            </div>
        </Router>
    }
}

#[component(transparent)]
fn ItineraryInfoRoutes() -> impl IntoView {
    view! {
        <Route path=":id" view=ItineraryView>
            <Route path="" view=move || view! {}/>
            <Route path="stays" view=ItineraryStays/>
            <Route path="legs" view=ItineraryTravelLegs/>
        // <Route path="address" view=Address/>
        // <Route path="messages" view=Messages/>
        </Route>
    }
}
