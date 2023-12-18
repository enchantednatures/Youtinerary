use leptos::html::Div;
use leptos::*;

#[derive(Debug, Clone, Copy)]
pub struct ShowNavigationSignal(RwSignal<bool>);

impl ShowNavigationSignal {
    pub fn new(signal: RwSignal<bool>) -> Self {
        Self(signal)
    }

    pub fn set(&self, value: bool) {
        self.0.set(value);
    }
    pub fn get(&self) -> bool { 
        self.0.get()
    }
}

#[component]
pub fn Navigation() -> impl IntoView {
    let target = create_node_ref::<Div>();

    let show_nav: ShowNavigationSignal = use_context().expect("Navigation signal not provided");

    view! {
        <div
            class="navigation"
            class:active=move || show_nav.get()
            // on:contextmenu=prevent_default
            on:contextmenu=move |evt| {
                evt.prevent_default();
                show_nav.set(!show_nav.get())
            }

            on:click=move |_| { show_nav.set(!show_nav.get()) }
        >
            <span style="--i:0;--x:-1;--y:0;">
                <ion-icon name="camera-outline"></ion-icon>
            </span>
            <span style="--i:1;--x:1;--y:0;">
                <ion-icon name="diamond-outline"></ion-icon>
            </span>
            <span style="--i:2;--x:0;--y:-1;">
                <ion-icon name="chatbubble-outline"></ion-icon>
            </span>
            <span style="--i:3;--x:0;--y:1;">
                <ion-icon name="alarm-outline"></ion-icon>
            </span>
            <span style="--i:4;--x:1;--y:1;">
                <ion-icon name="game-controller-outline"></ion-icon>
            </span>
            <span style="--i:5;--x:-1;--y:-1;">
                <ion-icon name="moon-outline"></ion-icon>
            </span>
            <span style="--i:6;--x:0;--y:0;">
                <ion-icon name="notifications-outline"></ion-icon>
            </span>
            <span style="--i:7;--x:-1;--y:1;">
                <ion-icon name="water-outline"></ion-icon>
            </span>
            <span style="--i:8;--x:1;--y:-1;">
                <ion-icon name="time-outline"></ion-icon>
            </span>
        </div>
    }
}
