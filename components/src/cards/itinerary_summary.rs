use chrono::DateTime;
use chrono::Utc;
use leptos::*;
use leptos_router::A;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItinerarySummary {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[component]
fn ItinerarySummaryCard(itinerary: ItinerarySummary, link: String) -> impl IntoView {
    view! {
        <A id="itinerary-summary-{itinerary.id}" href=link>
            <p>{itinerary.name}</p>
            <p>{itinerary.description}</p>
            <p>{itinerary.start_date.to_string()}</p>
            <p>{itinerary.end_date.to_string()}</p>
        </A>
    }
}
