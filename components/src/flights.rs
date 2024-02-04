
use leptos::*;
use leptos_router::*;

use models::Flight;

#[component]
pub fn FlightSummaryCard(flight: Flight) -> impl IntoView {
    view! {
        <div>
            <p>{flight.airline}</p>
            <p>{flight.departure_airport}</p>
            <p>{flight.arrival_airport}</p>
        </div>
    }
}

#[component]
pub fn Flights() -> impl IntoView {
    view! {
        <div>
            <h1>"Flights"</h1>
            <A href="/">Flight</A>
        </div>
    }
}
