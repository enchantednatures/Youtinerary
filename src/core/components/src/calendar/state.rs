use chrono::*;
use leptos::*;

#[derive(Clone)]
pub struct CalendarState {
    year: RwSignal<i32>,
    current_month: RwSignal<Months>,
    pub current: RwSignal<NaiveDate>,
}

impl CalendarState {
    pub fn new() -> Self {
        let current_date = chrono::Utc::now();
        Self {
            year: RwSignal::new(current_date.year()),
            current_month: RwSignal::new(Months::new(current_date.month())),
            current: RwSignal::new(current_date.date_naive()),
        }
    }
}
