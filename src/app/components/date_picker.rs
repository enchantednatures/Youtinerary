use chrono::{NaiveDate, Utc};
use leptos::*;

#[component]
pub fn DatePicker(
    selected_date: RwSignal<NaiveDate>,
    selected_end_date: RwSignal<NaiveDate>,
) -> impl IntoView {
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();

    view! {
        <div class="flex flex-wrap flex-row gap-x-2">
            <input
                type="date"
                class="w-3/8 block rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                prop:value=&today
                min=&today
                on:input=move |ev| {
                    selected_date
                        .set(
                            NaiveDate::parse_from_str(&event_target_value(&ev), "%Y-%m-%d").unwrap(),
                        )
                }
            />

            <input
                type="date"
                class="w-3/8 block rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                prop:value=move || selected_date.get().format("%Y-%m-%d").to_string()
                min=move || selected_date.get().format("%Y-%m-%d").to_string()
                on:input=move |ev| {
                    selected_end_date
                        .set(
                            NaiveDate::parse_from_str(&event_target_value(&ev), "%Y-%m-%d").unwrap(),
                        )
                }
            />

        </div>
    }
}
