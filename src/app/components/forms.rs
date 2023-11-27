use chrono::Utc;
use leptos::*;

mod inputs;
use inputs::BudgetInput;
use inputs::NewItineraryNameInput;

#[component]
pub fn CreateItineraryForm() -> impl IntoView {
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
    view! {
        <form class="space-y-8 divide-y divide-gray-200">
            <div class="space-y-8 divide-y divide-gray-200 sm:space-y-5">
                <div>
                    <div class="mt-6 sm:mt-5 space-y-6 sm:space-y-5">
                        <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start">
                            <div class="isolate -space-y-px rounded-md shadow-sm">
                                <NewItineraryNameInput/>
                                <BudgetInput/>
                                <input type="date" class="w-full" value=&today min=&today/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </form>
    }
}
