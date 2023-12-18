use leptos::*;
use leptos_meta::*;

use crate::app::components::UserSignupForm;

#[component]
pub fn Signup() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="Signup"/>
        <UserSignupForm/>
    }
}
