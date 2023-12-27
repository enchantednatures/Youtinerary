use chrono::Utc;
use leptos::*;
use leptos::{html::Input, leptos_dom::logging::console_log};
use leptos_router::*;

use crate::app::components::DatePicker;
use crate::app::state::{CreateFlightRequest, GlobalStateSignal};

#[derive(Debug, Clone, Copy)]
pub struct ShowCreateFlightSlideOutSignal(RwSignal<bool>);

impl ShowCreateFlightSlideOutSignal {
    pub fn new(signal: RwSignal<bool>) -> Self {
        Self(signal)
    }

    pub fn set(&self, value: bool) {
        self.0.set(value);
    }

}

#[derive(Params, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateFlightSlideOutParams {
    pub itinerary_id: usize,
}

#[component]
pub fn CreateFlightSlideOut() -> impl IntoView {
    let show: ShowCreateFlightSlideOutSignal = expect_context::<ShowCreateFlightSlideOutSignal>();
    let global_state = expect_context::<GlobalStateSignal>();

    let params = use_params::<CreateFlightSlideOutParams>();

    let itinerary_id = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.itinerary_id)
                .unwrap_or_default()
        })
    };

    let name_signal = create_rw_signal("".to_string());
    let description_signal = create_rw_signal("".to_string());

    let  departure_date= create_rw_signal(Utc::now().naive_utc().date());
    let arrival_date = create_rw_signal(Utc::now().naive_utc().date());

    let itnerary_signal = create_rw_signal(CreateFlightRequest {
        departure_airport: "".to_string(),
        arrival_airport: "".to_string(),
        airline: "".to_string(),
        confirmation_code: "".to_string(),
        departure_time: Utc::now(),
        arrival_time: Utc::now(),
    });

    let slide_target = create_node_ref::<Input>();

    create_effect(move |_| {
        if let Some(input) = slide_target.get() {
            let _ = input.focus();
        }
    });

    let (button_is_disabled, _) =
        create_signal(move || name_signal.get().is_empty() || description_signal.get().is_empty());

    view! {
        <div
            class="relative z-50"
            aria-labelledby="slide-over-title"
            role="dialog"
            aria-modal="true"
        >
            <div class="fixed inset-0 overflow-hidden">
                <div class="absolute inset-0 overflow-hidden">
                    <div class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10 sm:pl-16">
                        <div class="pointer-events-auto w-screen max-w-2xl">
                            <form
                                class="flex h-full flex-col overflow-y-scroll bg-white shadow-xl"
                                on:submit=|ev| ev.prevent_default()
                            >
                                <div class="flex-1">
                                    <div class="bg-gray-50 px-4 py-6 sm:px-6">
                                        <div class="flex items-start justify-between space-x-3">
                                            <div class="space-y-1">
                                                <h2
                                                    class="text-base font-semibold leading-6 text-gray-900"
                                                    id="slide-over-title"
                                                >
                                                    Add new flight to Itinerary
                                                </h2>
                                                <p class="text-sm text-gray-500">
                                                    Get started by filling in the information below to create your new flight.
                                                </p>
                                            </div>
                                            <div class="flex h-7 items-center">
                                                <button
                                                    type="button"
                                                    class="relative text-gray-400 hover:text-gray-500"
                                                    on:click=move |_| show.set(false)
                                                >
                                                    <span class="absolute -inset-2.5"></span>
                                                    <span class="sr-only">Close panel</span>
                                                    <svg
                                                        class="h-6 w-6"
                                                        fill="none"
                                                        viewBox="0 0 24 24"
                                                        stroke-width="1.5"
                                                        stroke="currentColor"
                                                        aria-hidden="true"
                                                    >
                                                        <path
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            d="M6 18L18 6M6 6l12 12"
                                                        ></path>
                                                    </svg>
                                                </button>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="space-y-6 py-6 sm:space-y-0 sm:divide-y sm:divide-gray-200 sm:py-0">
                                        <div class="space-y-2 px-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:space-y-0 sm:px-6 sm:py-5">
                                            <div>
                                                <label
                                                    for="itinerary-name"
                                                    class="block text-sm font-medium leading-6 text-gray-900 sm:mt-1.5"
                                                >
                                                    Itinerary name
                                                </label>
                                            </div>
                                            <div class="sm:col-span-2">
                                                <input
                                                    type="text"
                                                    node_ref=slide_target
                                                    name="itinerary-name"
                                                    id="itinerary-name"
                                                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                                    autofocus=true
                                                    on:input=move |ev| {
                                                        name_signal.set(event_target_value(&ev));
                                                        console_log(&name_signal.get());
                                                    }
                                                />

                                            </div>
                                        </div>

                                        <div class="space-y-2 px-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:space-y-0 sm:px-6 sm:py-5">
                                            <div>
                                                <label
                                                    for="itinerary-description"
                                                    class="block text-sm font-medium leading-6 text-gray-900 sm:mt-1.5"
                                                >
                                                    Description
                                                </label>
                                            </div>
                                            <div class="sm:col-span-2">
                                                <textarea
                                                    id="itinerary-description"
                                                    name="itinerary-description"
                                                    rows="3"
                                                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                                    on:input=move |ev| {
                                                        description_signal.set(event_target_value(&ev));
                                                        console_log(&description_signal.get());
                                                    }
                                                >
                                                </textarea>
                                            </div>
                                        </div>

                                        <div class="space-y-2 px-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:space-y-0 sm:px-6 sm:py-5">
                                            <div>
                                                <label
                                                    for="itinerary-dates"
                                                    class="block text-sm font-medium leading-6 text-gray-900 sm:mt-1.5"
                                                >
                                                    Date Range
                                                </label>
                                            </div>
                                            <div class="sm:col-span-2">
                                                <DatePicker
                                                    selected_date=departure_date
                                                    selected_end_date=arrival_date
                                                />
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="flex-shrink-0 border-t border-gray-200 px-4 py-5 sm:px-6">
                                    <div class="flex justify-end space-x-3">
                                        <button
                                            type="button"
                                            class="rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                                            on:click=move |_| show.set(false)
                                        >
                                            Cancel
                                        </button>
                                        // todo: reenable
                                        <button
                                            type="submit"
                                            prop:disabled=button_is_disabled()
                                            class="inline-flex justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:bg-slate-50 disabled:text-slate-500"
                                            on:click=move |_| {
                                                global_state
                                                    .get()
                                                    .add_flight(itinerary_id(), itnerary_signal.get());
                                                show.set(false);
                                            }
                                        >

                                            Create
                                        </button>
                                    </div>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
