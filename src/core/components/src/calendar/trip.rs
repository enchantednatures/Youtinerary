use chrono::{DateTime, TimeZone};
use leptos::*;

pub struct TripCalendarViewModel<Tz: TimeZone> {
    pub start_date: DateTime<Tz>,
    pub end_date: DateTime<Tz>,
    pub title: String,
}

#[component]
pub fn Trip<F: Fn() -> TripCalendarViewModel<Tz> + 'static, Tz: TimeZone>(
    trip: F,
) -> impl IntoView {
    view! {
        <div>
            <h1>{trip().title}</h1>
            <p>{trip().start_date.date_naive().format("").to_string()}</p>
            <p>{trip().end_date.date_naive().format("").to_string()}</p>
        </div>
    }
}
