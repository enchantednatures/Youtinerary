use leptos::RwSignal;

#[derive(Clone, Debug, Default)]
pub struct LoggedInUser {
    pub email: String,
    pub name: String,
    pub username: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, Default)]
pub struct GlobalState {
    pub user: Option<LoggedInUser>,
}

pub type GlobalStateSignal = RwSignal<GlobalState>;
