#![allow(non_snake_case)]

mod calendar;
mod cards;
mod date_picker;
mod divider;
mod flights;
mod forms;
mod slide_out;
mod user_icon;
mod user_signup;

mod create_itinerary {
    //! the components relating to the creating of an itinerary
}
mod add_train_leg {}
mod add_stay {}
mod view_itinerary_flights {}
mod view_itinerary_stays {}
mod view_itinerary_budget {}
mod add_itinerary_budget {}

pub use calendar::*;
pub use date_picker::*;
pub use divider::*;
pub use flights::*;
pub use forms::*;
pub use slide_out::*;
pub use user_icon::*;
pub use user_signup::*;
