use leptos::expect_context;
use leptos::*;

use super::state::GlobalStateSignal;

mod home;
mod itineraries;
mod itinerary;

pub use home::Home;
pub use itineraries::ItinerariesView;
pub use itineraries::TravelOutlet;
pub use itinerary::ItineraryView;
pub use itinerary::*;


mod signup;
pub use signup::Signup;

pub fn logout() {
    let state = expect_context::<GlobalStateSignal>();

    state.update(|s| s.user = None);
}

mod login;
pub use login::LogOut;
pub use login::Login;
