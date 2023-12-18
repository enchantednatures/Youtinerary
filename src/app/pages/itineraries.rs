use itertools::Itertools;
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

use crate::app::state::GlobalStateSignal;
use crate::models::user::Itinerary;

#[component]
pub fn TravelOutlet() -> impl IntoView {
    view! { <Outlet/> }
}

#[derive(Debug, Clone)]
pub struct ItineraryData {
    key: usize,
    value: ReadSignal<Itinerary>,
}

impl ItineraryData {
    pub fn new(key: usize, value: ReadSignal<Itinerary>) -> Self {
        Self { key, value }
    }
}

#[component]
pub fn ItinerariesView() -> impl IntoView {
    provide_meta_context();
    let state = expect_context::<GlobalStateSignal>();
    let itinerary_cards = move || {
        state
            .get()
            .iter()
            .map(|itinerary| {
                let (itinary_signal, _) = create_signal(itinerary.clone());
                ItineraryData::new(itinerary.id, itinary_signal)
            })
            .collect_vec()
    };
    view! {
        <ul role="list" class="grid grid-cols-2 gap-2 sm:grid-cols-3 lg:grid-cols-4">
            <For

                each=itinerary_cards
                key=|state| state.key
                children=|child| {
                    let itinerary = move || child.value.get();
                    view! { <ItineraryCard itinerary=itinerary()/> }
                }
            />

        </ul>
    }
}

#[component]
pub fn ItineraryCard(itinerary: Itinerary) -> impl IntoView {
    view! {
        <A href=format!("{}", itinerary.id)>
            <div class="max-w-sm w-full lg:max-w-full lg:flex">
                <div
                    class="h-48 lg:h-auto lg:w-48 flex-none bg-cover rounded-t lg:rounded-t-none lg:rounded-l text-center overflow-hidden"
                    style="background-image: url('/img/card-left.jpg')"
                    title="Woman holding a mug"
                ></div>
                <div class="border-r border-b border-l border-gray-400 lg:border-l-0 lg:border-t lg:border-gray-400 bg-white rounded-b lg:rounded-b-none lg:rounded-r p-4 flex flex-col justify-between leading-normal">
                    <div class="mb-8">
                        <p class="text-sm text-gray-600 flex items-center">
                            {itinerary.description}
                        </p>
                        <div class="text-gray-900 font-bold text-xl mb-2">{itinerary.name}</div>
                        <p class="text-gray-700 text-base">{itinerary.start_date.to_string()}</p>
                    </div>
                    <div class="flex items-center">
                        <div class="text-sm">
                            <p class="text-gray-900 leading-none">Hunter</p>
                            <p class="text-gray-600">
                                {format!("{}", itinerary.start_date.format("%b %y"))}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </A>
    }
}
