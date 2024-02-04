use components::UserSignupForm;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Signup() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="Signup"/>
        <UserSignupForm/>
    }
}
