use std::time::Duration;

use auth_config::AuthSettings;
use components::CreateItinerarySlideOut;
use components::ShowItinerarySignal;
use icons::LogoIcon;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::Authorize;
use pages::ItinerariesView;
use pages::ItineraryStays;
use pages::ItineraryTravelLegs;
use pages::ItineraryView;
use pages::TravelOutlet;

mod command_pallet;
mod nav;

use nav::Nav;
use pages::About;
use pages::Home;
use pages::LogOut;
use pages::Signup;

mod header;
use command_pallet::ShowCommandPalletSignal;
use header::Header;
use oauth2::basic::BasicClient;
use state::GlobalState;

use crate::app::command_pallet::CommandPallet;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let show_command_pallet = create_rw_signal(false);

    let show_itinerary = create_rw_signal(false);
    let state = create_rw_signal(GlobalState::default());
    let auth_settings = AuthSettings {
    };
    let auth_client: BasicClient = auth_settings.try_into().expect("no auth client");

    provide_context(state);
    provide_context(reqwest::Client::new());
    provide_context(auth_client);
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

                        <Nav />
                    </div>
                </div>

                <div class="lg:pl-72">
                    <Header/>
                    <main class="py-10">
                        <AnimatedShow
                            when=show_command_pallet
                            show_class="ease-out duration-900 opacity-100"
                            hide_class="ease-in duration-300 opacity-0"
                            hide_delay=Duration::from_millis(300)
                        >
                            <CommandPallet/>
                        </AnimatedShow>

                        <AnimatedShow
                            when=show_itinerary
                            show_class="ease-out duration-900 opacity-100"
                            hide_class="ease-in duration-300 opacity-0"
                            hide_delay=Duration::from_millis(300)
                        >
                            <CreateItinerarySlideOut/>
                        </AnimatedShow>
                        <Routes>
                            <Route path="/" view=Home/>
                            <Route path="/about" view=About/>
                            <Route path="/authorized" view=Authorize/>
                            <Route path="/logout" view=LogOut/>
                            <Route path="/signup" view=Signup/>

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
