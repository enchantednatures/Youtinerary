use std::collections::HashMap;
use std::fmt::Debug;

use chrono::{Datelike, Months, NaiveDate, NaiveTime, Timelike};
use leptos::*;
use leptos_router::A;

#[derive(Clone)]
struct CalendarState {
    year: RwSignal<i32>,
    current_month: RwSignal<Months>,
    current: RwSignal<NaiveDate>,
}

impl CalendarState {
    fn new() -> Self {
        let current_date = chrono::Utc::now();
        Self {
            year: RwSignal::new(current_date.year()),
            current_month: RwSignal::new(Months::new(current_date.month())),
            current: RwSignal::new(current_date.date_naive()),
        }
    }
}

#[component]
pub fn Calendar() -> impl IntoView {
    let current_date = chrono::Utc::now();
    let today = current_date.date_naive();
    provide_context(CalendarState::new());
    let mut weeks = Vec::new();

    let mut current_start = current_date.date_naive().with_day(1).unwrap();
    weeks.push(view! { <CalendarWeek start=current_start/> });

    while current_start.month() == current_date.month() {
        current_start = current_start.succ_opt().unwrap();
        if current_start.weekday() == chrono::Weekday::Sun {
            weeks.push(view! { <CalendarWeek start=current_start/> });
        }
    }

    view! {
        <div class="lg:flex lg:h-full lg:flex-col">
            <CalendarHeader/>
            <div class="shadow ring-1 ring-black ring-opacity-5 lg:flex lg:flex-auto lg:flex-col">
                <div class="grid grid-cols-7 gap-px border-b border-gray-300 bg-gray-200 text-center text-xs font-semibold leading-6 text-gray-700 lg:flex-none">
                    <div class="flex justify-center bg-white py-2">
                        <span>S</span>
                        <span class="sr-only sm:not-sr-only">un</span>
                    </div>
                    <div class="flex justify-center bg-white py-2">
                        <span>M</span>
                        <span class="sr-only sm:not-sr-only">on</span>
                    </div>
                    <div class="flex justify-center bg-white py-2">
                        <span>T</span>
                        <span class="sr-only sm:not-sr-only">ue</span>
                    </div>
                    <div class="flex justify-center bg-white py-2">
                        <span>W</span>
                        <span class="sr-only sm:not-sr-only">ed</span>
                    </div>
                    <div class="flex justify-center bg-white py-2">
                        <span>T</span>
                        <span class="sr-only sm:not-sr-only">hu</span>
                    </div>
                    <div class="flex justify-center bg-white py-2">
                        <span>F</span>
                        <span class="sr-only sm:not-sr-only">ri</span>
                    </div>
                    <div class="flex justify-center bg-white py-2">
                        <span>S</span>
                        <span class="sr-only sm:not-sr-only">at</span>
                    </div>

                </div>

                // <CalendarWeek start=today/>
                {weeks}

            </div>
        </div>
    }
}

#[component]
pub fn CalendarHeader() -> impl IntoView {
    let state = expect_context::<CalendarState>();
    view! {
        <header class="flex items-center justify-between border-b border-gray-200 px-6 py-4 lg:flex-none">
            <h1 class="text-base font-semibold leading-6 text-gray-900">
                <time datetime=state
                    .current
                    .get()
                    .to_string()>

                    {format!("{}", state.current.get().format("%B %Y"))}
                </time>
            </h1>
            <div class="flex items-center">
                <div class="relative flex items-center rounded-md bg-white shadow-sm md:items-stretch">
                    <button
                        type="button"
                        class="flex h-9 w-12 items-center justify-center rounded-l-md border-y border-l border-gray-300 pr-1 text-gray-400 hover:text-gray-500 focus:relative md:w-9 md:pr-0 md:hover:bg-gray-50"
                    >
                        <span class="sr-only">Previous month</span>
                        <svg
                            class="h-5 w-5"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M12.79 5.23a.75.75 0 01-.02 1.06L8.832 10l3.938 3.71a.75.75 0 11-1.04 1.08l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 011.06.02z"
                                clip-rule="evenodd"
                            ></path>
                        </svg>
                    </button>
                    <button
                        type="button"
                        class="hidden border-y border-gray-300 px-3.5 text-sm font-semibold text-gray-900 hover:bg-gray-50 focus:relative md:block"
                    >
                        Today
                    </button>
                    <span class="relative -mx-px h-5 w-px bg-gray-300 md:hidden"></span>
                    <button
                        type="button"
                        class="flex h-9 w-12 items-center justify-center rounded-r-md border-y border-r border-gray-300 pl-1 text-gray-400 hover:text-gray-500 focus:relative md:w-9 md:pl-0 md:hover:bg-gray-50"
                    >
                        <span class="sr-only">Next month</span>
                        <svg
                            class="h-5 w-5"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                                clip-rule="evenodd"
                            ></path>
                        </svg>
                    </button>
                </div>
                <div class="hidden md:ml-4 md:flex md:items-center">
                    <div class="relative">
                        <button
                            type="button"
                            class="flex items-center gap-x-1.5 rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                            id="menu-button"
                            aria-expanded="false"
                            aria-haspopup="true"
                        >
                            Month view
                            <svg
                                class="-mr-1 h-5 w-5 text-gray-400"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                                    clip-rule="evenodd"
                                ></path>
                            </svg>
                        </button>
                    </div>
                    <div class="ml-6 h-6 w-px bg-gray-300"></div>
                    <button
                        type="button"
                        class="ml-6 rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                    >
                        Add event
                    </button>
                </div>
                <div class="relative ml-6 md:hidden">
                    <button
                        type="button"
                        class="-mx-2 flex items-center rounded-full border border-transparent p-2 text-gray-400 hover:text-gray-500"
                        id="menu-0-button"
                        aria-expanded="false"
                        aria-haspopup="true"
                    >
                        <span class="sr-only">Open menu</span>
                        <svg
                            class="h-5 w-5"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path d="M3 10a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0zM8.5 10a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0zM15.5 8.5a1.5 1.5 0 100 3 1.5 1.5 0 000-3z"></path>
                        </svg>
                    </button>

                    <div
                        class="absolute right-0 z-10 mt-3 w-36 origin-top-right divide-y divide-gray-100 overflow-hidden rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                        role="menu"
                        aria-orientation="vertical"
                        aria-labelledby="menu-0-button"
                        tabindex="-1"
                    >
                        <div class="py-1" role="none">
                            <a
                                href="#"
                                class="text-gray-700 block px-4 py-2 text-sm"
                                role="menuitem"
                                tabindex="-1"
                                id="menu-0-item-0"
                            >
                                Create event
                            </a>
                        </div>
                        <div class="py-1" role="none">
                            <a
                                href="#"
                                class="text-gray-700 block px-4 py-2 text-sm"
                                role="menuitem"
                                tabindex="-1"
                                id="menu-0-item-1"
                            >
                                Go to today
                            </a>
                        </div>
                        <div class="py-1" role="none">
                            <a
                                href="#"
                                class="text-gray-700 block px-4 py-2 text-sm"
                                role="menuitem"
                                tabindex="-1"
                                id="menu-0-item-2"
                            >
                                Day view
                            </a>
                            <a
                                href="#"
                                class="text-gray-700 block px-4 py-2 text-sm"
                                role="menuitem"
                                tabindex="-1"
                                id="menu-0-item-3"
                            >
                                Week view
                            </a>
                            <a
                                href="#"
                                class="text-gray-700 block px-4 py-2 text-sm"
                                role="menuitem"
                                tabindex="-1"
                                id="menu-0-item-4"
                            >
                                Month view
                            </a>
                            <a
                                href="#"
                                class="text-gray-700 block px-4 py-2 text-sm"
                                role="menuitem"
                                tabindex="-1"
                                id="menu-0-item-5"
                            >
                                Year view
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
pub fn CalendarWeek(start: NaiveDate) -> impl IntoView {
    let mut days = Vec::new();
    let mut current_date = start;
    while current_date.weekday() != chrono::Weekday::Sun {
        current_date = current_date.pred_opt().expect("no previous day?");
    }
    for _ in 0..7 {
        days.push(view! { <CalendarDay day=current_date/> });
        current_date = current_date.succ_opt().expect("no next day?");
    }

    view! {
        <div class="flex bg-gray-200 text-xs leading-6 text-gray-700 lg:flex-auto">
            <div class="hidden w-full lg:grid lg:grid-cols-7 lg:grid-rows-6 lg:gap-px">{days}</div>
        </div>
    }
}

#[component]
pub fn CalendarDay(day: NaiveDate) -> impl IntoView {
    view! {
        <div class="relative bg-white px-3 py-2">
            <time datetime=day.to_string()>{day.day()}</time>
        </div>
    }
}

#[component]
pub fn CalendarEvent(id: usize, name: String, time: NaiveTime) -> impl IntoView {
    let t = time.format("%H:%M").to_string();
    view! {
        <div class="absolute inset-0 bg-green-500 bg-opacity-75">
            <A href=format!("itinerary/{}", id)>
                <p class="text-white font-semibold">{t}</p>
                <p class="text-white font-semibold">{name}</p>
            </A>

        </div>
    }
}

#[component]
fn CalendarSelectViewMenu() -> impl IntoView {
    view! {}
}
