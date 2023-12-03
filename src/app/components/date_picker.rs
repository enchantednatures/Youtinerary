use chrono::{Datelike, Month, NaiveDate, Utc};
use leptos::*;


#[component]
pub fn DatePicker(selected_date: RwSignal<NaiveDate>, selected_end_date: RwSignal<NaiveDate>) -> impl IntoView {
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();

    view! {
        <input
            type="date"
            class="w-full"
            prop:value=&today
            min=&today
            on:input=move |ev| selected_date.set(
                NaiveDate::parse_from_str(&event_target_value(&ev), "%Y-%m-%d").unwrap(),
            )
        />

        <input
            type="date"
            class="w-full"
            prop:value=move || selected_date.get().format("%Y-%m-%d").to_string()
            min=move || selected_date.get().format("%Y-%m-%d").to_string()
            on:input=move |ev| selected_end_date.set(
                NaiveDate::parse_from_str(&event_target_value(&ev), "%Y-%m-%d").unwrap(),
            )
        />
    }
}
