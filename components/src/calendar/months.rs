use leptos::*;

use crate::calendar::CalendarState;

#[component]
pub fn CalendarMonthDropDown() -> impl IntoView {
    let state = expect_context::<CalendarState>();
}
