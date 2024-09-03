use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use leptos::*;

#[component]
pub fn DateTimePicker(selected_date: RwSignal<DateTime<Utc>>) -> impl IntoView {
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();

    view! {
        <input
            type="date"
            class="w-3/8 block rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 appearance-none bg-white cursor-pointer"
            prop:value=&today
            min=&today
            style="background-image: url('data:image/svg+xml;utf8,<svg fill=%22currentColor%22 xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 16 16%22><path d=%22M3.5 0a.5.5 0 0 1 .5.5V1h8V.5a.5.5 0 0 1 1 0V1h1a2 2 0 0 1 2 2v11a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h1V.5a.5.5 0 0 1 .5-.5zM1 4v10a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V4H1z%22/></svg>'); background-repeat: no-repeat; background-position: right 0.5rem center; background-size: 1.5em;"
            on:input=move |ev| {
                selected_date
                    .set(
                        event_target_value(&ev)
                            .parse::<DateTime<Utc>>()
                            .expect("Failed to parse departure date"),
                    )
            }
        />
    }
}

#[component]
pub fn DateRangePicker(
    selected_start_date: RwSignal<NaiveDate>,
    selected_end_date: RwSignal<NaiveDate>,
) -> impl IntoView {
    let today = Utc::now().date_naive();
    let (is_open, set_is_open) = create_signal(false);
    let (temp_start_date, set_temp_start_date) = create_signal(today);
    let (temp_end_date, set_temp_end_date) = create_signal(today);

    let toggle_popout = move |_| set_is_open.update(|value| *value = !*value);

    let apply_dates = move |_| {
        selected_start_date.set(temp_start_date.get());
        selected_end_date.set(temp_end_date.get());
        set_is_open.set(false);
    };

    view! {
        <div class="relative">
            <button
                class="w-full px-4 py-2 text-left bg-white border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
                on:click=toggle_popout
            >
                {move || format!("{} - {}", 
                    selected_start_date.get().format("%Y-%m-%d"), 
                    selected_end_date.get().format("%Y-%m-%d")
                )}
            </button>

            {move || if is_open.get() {
                view! {
                    <div class="absolute z-10 w-full mt-1 bg-white border border-gray-300 rounded-md shadow-lg">
                        <div class="p-4">
                            <div class="flex gap-4 mb-4">
                                <input
                                    type="date"
                                    class="w-1/2 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
                                    prop:value=move || temp_start_date.get().format("%Y-%m-%d").to_string()
                                    min=today.format("%Y-%m-%d").to_string()
                                    on:input=move |ev| {
                                        if let Ok(date) = NaiveDate::parse_from_str(&event_target_value(&ev), "%Y-%m-%d") {
                                            set_temp_start_date.set(date);
                                        }
                                    }
                                />
                                <input
                                    type="date"
                                    class="w-1/2 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
                                    prop:value=move || temp_end_date.get().format("%Y-%m-%d").to_string()
                                    min=move || temp_start_date.get().format("%Y-%m-%d").to_string()
                                    on:input=move |ev| {
                                        if let Ok(date) = NaiveDate::parse_from_str(&event_target_value(&ev), "%Y-%m-%d") {
                                            set_temp_end_date.set(date);
                                        }
                                    }
                                />
                            </div>
                            <button
                                class="w-full px-4 py-2 text-white bg-indigo-600 rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                                on:click=apply_dates
                            >
                                "Apply"
                            </button>
                        </div>
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
        </div>
    }
}
