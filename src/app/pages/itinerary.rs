use leptos::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::*;

use crate::app::state::GlobalStateSignal;

#[derive(Params, PartialEq, Eq, PartialOrd, Ord)]
struct ItineraryParams {
    id: usize,
}

#[component]
pub fn ItineraryView() -> impl IntoView {
    provide_meta_context();
    let global_state = expect_context::<GlobalStateSignal>();
    let params = use_params::<ItineraryParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());
    let itinerary = global_state.get().get_itinerary(id()).unwrap().clone();
    let formatter = |text| format!("{text} — Youtinerary");

    view! {
        <Title formatter/>
        <div class="max-w-sm w-full lg:max-w-full lg:flex">
        //     <div
        //         class="h-48 lg:h-auto lg:w-48 flex-none bg-cover rounded-t lg:rounded-t-none lg:rounded-l text-center overflow-hidden"
        //         title="Itinerary"
        //     >
        // </div>
            <div class="border-r border-b border-l border-gray-400 lg:border-l-0 lg:border-t lg:border-gray-400 bg-white rounded-b lg:rounded-b-none lg:rounded-r p-4 flex flex-col justify-between leading-normal">
                <div class="mb-8">
                    <p class="text-sm text-gray-600 flex items-center">{itinerary.description}</p>
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

        <span class="isolate inline-flex rounded-md shadow-sm">
            <A href="stays">

                <button
                    type="button"
                    class="relative inline-flex items-center rounded-l-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                >
                    Stays
                </button>
            </A>

            <A href="legs">
                <button
                    type="button"
                    class="relative -ml-px inline-flex items-center bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                >
                    Travels Legs
                </button>

            </A>
            <A href="/itineraries">
                <button
                    type="button"
                    class="relative -ml-px inline-flex items-center rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                >
                    Close
                </button>

            </A>
        </span>
        <br/>
        <Outlet/>
    }
}

#[component]
pub fn ItineraryStays() -> impl IntoView {
    view! {
        <span class="isolate inline-flex rounded-md shadow-sm">
            <button
                type="button"
                class="relative inline-flex items-center rounded-l-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
            >
                Hotels
            </button>
            <button
                type="button"
                class="relative -ml-px inline-flex items-center bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
            >
                Hostels
            </button>
            <button
                type="button"
                class="relative -ml-px inline-flex items-center rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
            >
                AirBnB
            </button>
        </span>
        <p>Stays</p>
    }
}

#[component]
pub fn ItineraryTravelLegs() -> impl IntoView {
    view! {
        <span class="isolate inline-flex rounded-md shadow-sm">
            <button
                type="button"
                class="relative inline-flex items-center rounded-l-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
            >
                Flights
            </button>
            <button
                type="button"
                class="relative -ml-px inline-flex items-center bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
            >
                Trains
            </button>
            <button
                type="button"
                class="relative -ml-px inline-flex items-center rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
            >
                Bus
            </button>
            <button
                type="button"
                class="relative -ml-px inline-flex items-center rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
            >
                Taxis
            </button>
        </span>
        <p>Travel Legs</p>
    }
}
