use chrono::{NaiveDate, Utc};
use leptos::*;
use leptos_router::create_query_signal;
use uuid::Uuid;

use crate::calendar::months::CalendarState;

struct Event {
    id: Uuid,
    parent_event_id: Option<Uuid>,
    date: NaiveDate,
    title: String,
}

impl Event {
    fn is_parent(&self) -> bool {
        self.parent_event_id.is_none()
    }
}

struct MockData<T> {
    data: T,
}

// impl<T> MockData<T> {
//     fn new(data: T) -> Self {
//         Self { data }
//     }
// }

impl MockData<Vec<Event>> {
    fn new(current_date: NaiveDate) -> Self {
        let parent_event_id = Uuid::new_v4();
        Self {
            data: vec![
                Event {
                    id: parent_event_id,
                    parent_event_id: None,
                    date: current_date,
                    title: "Event 1".to_string(),
                },
                Event {
                    id: Uuid::new_v4(),
                    parent_event_id: Some(parent_event_id),
                    date: current_date,
                    title: "Event 2".to_string(),
                },
                Event {
                    id: Uuid::new_v4(),
                    parent_event_id: Some(parent_event_id),
                    date: current_date,
                    title: "Event 3".to_string(),
                },
            ],
        }
    }
}

#[component]
fn MonthViewDay(day: NaiveDate) -> impl IntoView {
    let calendar_state = expect_context::<CalendarState>();
    let (events, _) = create_signal(MockData::new(calendar_state.current_date.get()).data);
    let (event, set_event) = create_query_signal::<Uuid>("event_id");
    let clear = move |_| set_event.set(None);
    let show_event = move |event_id: Uuid| set_event.set(Some(event_id));

    view! {
        <div class="relative h-full">
            <div class="p-2 h-full " >
                <span class="text-sm font-semibold">{day.format("%d").to_string()}</span>
                <ul class="mt-1">
                    {move || events.with(|events| events.iter().filter(|e| e.is_parent()).map(|event| {
                        let event_id = event.id;
                        let event_title = event.title.clone();
                        view! {
                            <li
                                class="text-xs bg-blue-500 text-white p-1 mb-1 rounded cursor-pointer"
                                on:click=move |_| show_event(event_id)
                            >
                                {event_title}
                            </li>
                        }
                    }).collect::<Vec<_>>())}
                </ul>
            </div>
            {move || event.get().map(|event_id| {
                view! {
                    <div class="absolute top-0 left-0 w-full h-full bg-white bg-opacity-90 p-4">
                        <h3 class="text-lg font-bold mb-2">
                            {events.with(|events| events.iter().find(|e| e.id == event_id).map(|e| e.title.clone()).unwrap_or_default())}
                        </h3>
                        <p class="text-sm mb-4">
                            {events.with(|events| events.iter().find(|e| e.id == event_id).map(|e| e.date.format("%Y-%m-%d").to_string()).unwrap_or_default())}
                        </p>
                        <button
                            class="bg-red-500 text-white px-2 py-1 rounded"
                            on:click=clear
                        >
                            "Close"
                        </button>
                    </div>
                }
            })}
        </div>
    }
}
