use itertools::Itertools;
use leptos::*;
use leptos_meta::provide_meta_context;

use crate::app::state::Itineraries;
use crate::models::user::Itinerary;

#[derive(Debug, Clone)]
pub struct ItineraryData {
    key: i32,
    value: ReadSignal<Itinerary>,
}

impl ItineraryData {
    pub fn new(key: i32, value: ReadSignal<Itinerary>) -> Self {
        Self { key, value }
    }
}

#[component]
pub fn ItineraryCard(data: ItineraryData) -> impl IntoView {
    let itinerary = data.value.get();
    view! {
        <div class="flex items-center justify-center h-screen">
            <div class="w-1/2 p-6 bg-white rounded-lg shadow-lg">
                <h2 class="text-2xl font-semibold mb-2">Card Title</h2>
                <div class="mb-4">
                    <p>
                        <strong>ID:</strong>
                        123
                    </p>
                    <p>
                        <strong>Name:</strong>
                        Sample Card
                    </p>
                    <p>
                        <strong>Description:</strong>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                    </p>
                    <p>
                        <strong>User ID:</strong>
                        456
                    </p>
                    <p>
                        <strong>Created At:</strong>
                        2023-01-16 12:34:56 UTC
                    </p>
                    <p>
                        <strong>Start Date:</strong>
                        2023-01-16
                    </p>
                    <p>
                        <strong>End Date:</strong>
                        2023-01-20
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ItinerariesView() -> impl IntoView {
    provide_meta_context();
    let itineraries = Itineraries::default();

    let itinerary_cards = itineraries
        .get()
        .iter()
        .map(|itinerary| {
            let (itinerary_signal, _) = create_signal(itinerary.clone());
            let data = ItineraryData::new(itinerary.id, itinerary_signal);
            view! {
                <li class="relative flex items-center space-x-4 py-4">
                    <ItineraryCard data=data/>
                </li>
            }
        })
        .collect_vec();

    view! {
        <ul role="list" class="divide-y divide-white/5">
            {itinerary_cards}
        </ul>
    }
}
