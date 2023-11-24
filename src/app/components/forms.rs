use leptos::*;

mod inputs;
use inputs::BudgetInput;
use inputs::NewItineraryNameInput;

#[component]
pub fn CreateItineraryForm() -> impl IntoView {
    view! {
        <form class="space-y-8 divide-y divide-gray-200">
            <div class="space-y-8 divide-y divide-gray-200 sm:space-y-5">
                <div>
                    <div class="mt-6 sm:mt-5 space-y-6 sm:space-y-5">
                        <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start">
                            <div class="isolate -space-y-px rounded-md shadow-sm">
                                <NewItineraryNameInput/>
                                <BudgetInput/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </form>
    }
}
