use itertools::Itertools;
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;
use models::Itinerary;
use state::GlobalStateSignal;

#[component]
pub fn TravelOutlet() -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <Outlet/>
        </div>
    }
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

#[derive(Debug, Clone, Copy)]
struct ShownItineraryOverlayContext(RwSignal<Option<usize>>);

impl ShownItineraryOverlayContext {
    pub fn new(signal: RwSignal<Option<usize>>) -> Self {
        Self(signal)
    }

    pub fn set(&self, value: usize) {
        self.0.set(Some(value));
    }
    pub fn clear(&self) {
        self.0.set(None);
    }
    pub fn get(&self) -> Option<usize> {
        self.0.get()
    }
}
#[component]
pub fn ItinerariesView() -> impl IntoView {
    provide_meta_context();
    let state = expect_context::<GlobalStateSignal>();
    provide_context(ShownItineraryOverlayContext(create_rw_signal(None)));
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
        <ul role="list" class="flex flex-row">
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
    let shown_overlay = expect_context::<ShownItineraryOverlayContext>();
    view! {
        <div
            class="basis-1/3 focus:basis-1/2"
            on:click=move |_| shown_overlay.set(itinerary.id)
            id=format!("itinerary-card-{}", itinerary.id)
        >
            // <div
            // class="h-48 lg:h-auto lg:w-48 flex-none bg-cover rounded-t lg:rounded-t-none lg:rounded-l text-center overflow-hidden"
            // style="background-image: url('/img/card-left.jpg')"
            // title="Woman holding a mug"
            // ></div>
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
        <Show when=move || Some(itinerary.id) == shown_overlay.get() fallback=|| ()>
            <Portal mount=document()
                .get_element_by_id(&format!("itinerary-card-{}", itinerary.id))
                .unwrap()>
                <span class="isolate inline-flex rounded-md shadow-sm">
                    <A href=format!("{}", itinerary.id)>
                        <button
                            type="button"
                            class="relative inline-flex items-center rounded-l-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                        >
                            Open
                        </button>
                    </A>

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
                    <A href="">
                        <button
                            type="button"
                            on:click=move |_| shown_overlay.set(itinerary.id)
                            class="relative -ml-px inline-flex items-center rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                        >
                            Close
                        </button>

                    </A>
                </span>
            </Portal>
        </Show>
    }
}
