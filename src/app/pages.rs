use leptos::expect_context;
use leptos::*;
use leptos_meta::*;
use leptos_router::{use_navigate, NavigateOptions};
use web_sys::MouseEvent;

use super::state::{GlobalStateSignal, LoggedInUser};

mod home;
mod itineraries;
mod itinerary;

pub use home::Home;
pub use itineraries::ItinerariesView;
pub use itineraries::TravelOutlet;
pub use itinerary::ItineraryView;

mod about;
pub use about::About;

mod signup;
pub use signup::Signup;

pub fn logout() {
    let state = expect_context::<GlobalStateSignal>();

    state.update(|s| s.user = None);
}

mod login;
pub use login::LogOut;
pub use login::Login;
