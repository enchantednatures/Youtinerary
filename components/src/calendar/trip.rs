use chrono::Date;
use leptos::*;

#[derive(Default)]
pub struct TripCalendarViewModel<Tz: TimeZone> {
    pub start_date: Date<Tz>,
    pub end_date: Date<Tz>,
    pub title: String,
}

#[component]
pub fn Trip<F: Fn() -> TripCalendarViewModel<Tz> + 'static>(trip: F) -> impl IntoView {
    view! {
        <div>
            <h1>{||trip().title}</h1>
            <p>{trip().start_date}</p>
            <p>{trip().end_date}</p>
        </div>
    }
}
