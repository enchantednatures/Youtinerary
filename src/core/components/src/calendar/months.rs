use chrono::{Datelike, NaiveDate, Utc};
use leptos::*;

#[derive(Clone, Debug)]
pub struct CalendarState {
    current_date: RwSignal<NaiveDate>,
    view_mode: RwSignal<ViewMode>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ViewMode {
    Month,
    Week,
    Day,
}

#[component]
pub fn Calendar() -> impl IntoView {
    let state = CalendarState {
        current_date: create_rw_signal(Utc::now().naive_local().date()),
        view_mode: create_rw_signal(ViewMode::Month),
    };

    provide_context(state.clone());

    view! {
        <div class="calendar-container">
            <CalendarHeader />
            <CalendarBody />
        </div>
    }
}

#[component]
fn CalendarHeader() -> impl IntoView {
    let state = expect_context::<CalendarState>();

    view! {
        <div class="flex justify-between items-center mb-4">
            <CalendarNavigation />
            <CalendarViewModeSelector />
        </div>
    }
}

#[component]
fn CalendarNavigation() -> impl IntoView {
    let state = expect_context::<CalendarState>();
    let current_date = state.current_date;

    let prev = move |_| {
        current_date.update(|date| *date = date.pred());
    };

    let next = move |_| {
        current_date.update(|date| *date = date.succ());
    };

    view! {
        <div class="flex items-center">
            <button on:click=prev class="px-2 py-1 bg-gray-200 rounded-l">"<"</button>
            <span class="px-4">{move || current_date.get().format("%B %Y").to_string()}</span>
            <button on:click=next class="px-2 py-1 bg-gray-200 rounded-r">">"</button>
        </div>
    }
}

#[component]
fn CalendarViewModeSelector() -> impl IntoView {
    let state = expect_context::<CalendarState>();
    let view_mode = state.view_mode;

    let set_view_mode = move |mode: ViewMode| {
        view_mode.set(mode);
    };

    view! {
        <div class="flex">
            <button
                class="px-3 py-1 bg-blue-500 text-white rounded-l"
                class:bg-blue-700=move || view_mode.get() == ViewMode::Month
                on:click=move |_| set_view_mode(ViewMode::Month)
            >
                "Month"
            </button>
            <button
                class="px-3 py-1 bg-blue-500 text-white"
                class:bg-blue-700=move || view_mode.get() == ViewMode::Week
                on:click=move |_| set_view_mode(ViewMode::Week)
            >
                "Week"
            </button>
            <button
                class="px-3 py-1 bg-blue-500 text-white rounded-r"
                class:bg-blue-700=move || view_mode.get() == ViewMode::Day
                on:click=move |_| set_view_mode(ViewMode::Day)
            >
                "Day"
            </button>
        </div>
    }
}

#[component]
fn CalendarBody() -> impl IntoView {
    let state = expect_context::<CalendarState>();
    let view_mode = state.view_mode;

    view! {
        <div>
            {move || match view_mode.get() {
                ViewMode::Month => view! { <MonthView /> },
                ViewMode::Week => view! { <WeekView /> },
                ViewMode::Day => view! { <DayView /> },
            }}
        </div>
    }
}

#[component]
fn MonthView() -> impl IntoView {
    let state = expect_context::<CalendarState>();
    let current_date = state.current_date;

    view! {
        <div class="grid grid-cols-7 gap-1">
            <For
                each=move || {
                    let date = current_date.get();
                    let start_of_month = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();
                    let days_in_month = date.with_day(1).unwrap().checked_add_months(chrono::Months::new(1)).unwrap().signed_duration_since(date.with_day(1).unwrap()).num_days();
                    let start_weekday = start_of_month.weekday().num_days_from_sunday();

                    (0..42).map(|i| {
                        let day: i64 = i - start_weekday as i64 + 1;
                        if day > 0 && day <= days_in_month {
                            Some(NaiveDate::from_ymd_opt(date.year(), date.month(), day as u32).unwrap())
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>()
                }
                key=|day| day.map(|d| d.format("%Y-%m-%d").to_string())
                children=move |day| {
                    view! {
                        <div class="p-2 border text-center">
                            {day.map(|d| d.day().to_string()).unwrap_or_default()}
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
fn WeekView() -> impl IntoView {
    view! {
        <div>"Week View (To be implemented)"</div>
    }
}

#[component]
fn DayView() -> impl IntoView {
    view! {
        <div>"Day View (To be implemented)"</div>
    }
}
