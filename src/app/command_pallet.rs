use leptos::html::Li;
use leptos::leptos_dom::logging::console_log;

use leptos::{html::Input, *};
use leptos_router::{use_navigate, NavigateOptions};
use leptos_use::{use_element_hover_with_options, UseElementHoverOptions};
use web_sys::MouseEvent;

use super::components::{ShowCreateFlightSlideOutSignal, ShowItinerarySignal};

#[derive(Debug, Clone, Copy)]
pub struct ShowCommandPalletSignal(WriteSignal<bool>);

impl ShowCommandPalletSignal {
    pub fn new(signal: WriteSignal<bool>) -> Self {
        Self(signal)
    }

    pub fn set(&self, value: bool) {
        self.0.set(value);
    }
}

#[component]
pub fn Command(id: usize, value: String, on_click: fn(MouseEvent) -> ()) -> impl IntoView {
    // this will have to change when we add keyboard support
    let target = create_node_ref::<Li>();
    let is_hovered = use_element_hover_with_options(
        target,
        UseElementHoverOptions::default()
            .delay_enter(0)
            .delay_leave(30),
    );

    view! {
        <li
            node_ref=target
            class="cursor-default select-none px-4 py-2"
            class:text-white=is_hovered
            class:bg-indigo-600=is_hovered
            id=format!("option-{}", id)
            role="option"
            tabindex="-1"
            on:click=on_click
        >
            {value}
        </li>
    }
}

fn create_new_flight_action(_me: MouseEvent) {
    let show_command_pallet: ShowCommandPalletSignal =
        use_context().expect("Show Create Flight slide out signal not provided");
    let show = use_context::<ShowCreateFlightSlideOutSignal>();
    let show = match show {
        Some(show) => show,
        None => {
            let sig = ShowCreateFlightSlideOutSignal::new(create_rw_signal(false));
            provide_context(sig);
            sig
        }
    };

    show.set(true);
    show_command_pallet.set(false);
}

fn create_new_itinerary_action(_me: MouseEvent) {
    let show_command_pallet: ShowCommandPalletSignal =
        use_context().expect("Command pallet signal not provided");
    console_log("clicked new itinerary");
    let show_iternary = expect_context::<ShowItinerarySignal>();
    show_iternary.set(true);
    show_command_pallet.set(false);
}

#[component]
pub fn CommandPallet() -> impl IntoView {
    // let (commands, set_commands) = create_signal(vec![]);
    let (user_input, set_user_input) = create_signal(String::new());
    let show_command_pallet = expect_context::<ShowCommandPalletSignal>();
    // use_context().expect("Command pallet signal not provided");
    let input_ref = create_node_ref::<Input>();

    create_effect(move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    let new_itinerary_action: fn(MouseEvent) -> _ = create_new_itinerary_action;
    let new_flight_action: fn(MouseEvent) -> _ = create_new_flight_action;

    let about_action: fn(MouseEvent) -> _ = |_| {
        let show_command_pallet: ShowCommandPalletSignal =
            use_context().expect("Command pallet signal not provided");
        console_log("clicked about");

        let navigation = use_navigate();
        navigation("/about", NavigateOptions::default());

        show_command_pallet.set(false);
    };

    let commands = vec![
        ("New Itinerary".to_string(), new_itinerary_action),
        ("New Flight".to_string(), new_flight_action),
        ("About".to_string(), about_action),
    ];

    let (cs, _) = create_signal(commands);

    let command_signal = create_memo(move |_| {
        cs().into_iter()
            .enumerate()
            .filter(|(_, (name, _))| {
                name.to_lowercase()
                    .contains(&user_input.with(|s| s.to_lowercase()))
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class="relative z-10" role="dialog" aria-modal="true">
            <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"></div>
            <div
                class="fixed inset-0 z-10 w-screen overflow-y-auto p-4 sm:p-6 md:p-20"
                on:focusout=move |_| {
                    show_command_pallet.set(false);
                }
            >

                <div class="mx-auto max-w-xl transform divide-y divide-gray-100 overflow-hidden rounded-xl bg-white shadow-2xl ring-1 ring-black ring-opacity-5 transition-all">
                    <div class="relative">
                        <svg
                            class="pointer-events-none absolute left-4 top-3.5 h-5 w-5 text-gray-400"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z"
                                clip-rule="evenodd"
                            ></path>
                        </svg>
                        <input
                            node_ref=input_ref
                            type="text"
                            class="h-12 w-full border-0 bg-transparent pl-11 pr-4 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm"
                            placeholder="Search..."
                            role="combobox"
                            aria-expanded="false"
                            aria-controls="options"
                            autofocus
                            prop:value=user_input
                            on:input=move |ev| {
                                set_user_input(event_target_value(&ev));
                                console_log(&user_input.get());
                            }
                        />

                    </div>

                    <ul
                        class="max-h-72 scroll-py-2 overflow-y-auto py-2 text-sm text-gray-800"
                        id="options"
                        role="listbox"
                    >
                        <For
                            each=command_signal
                            key=|(index, _)| *index
                            children=move |(id, (name, on_click))| {
                                view! { <Command id=id value=name.clone() on_click=on_click/> }
                            }
                        />

                    </ul>

                // <p class="p-4 text-sm text-gray-500">No people found.</p>
                </div>
            </div>
        </div>
    }
}
