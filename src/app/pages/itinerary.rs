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
            <div
                class="h-48 lg:h-auto lg:w-48 flex-none bg-cover rounded-t lg:rounded-t-none lg:rounded-l text-center overflow-hidden"
                style="background-image: url('/img/card-left.jpg')"
                title="Woman holding a mug"
            ></div>
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
    }
}
