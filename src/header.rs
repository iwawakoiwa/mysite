use dioxus::prelude::*;
use crate::Route;
use crate::AppState;

#[component]
pub fn Header() -> Element {
    let mut state: Signal<AppState> = use_context();

    let reboot = move |_| {
        state.set(AppState::Boot);
    };

    rsx! {
        nav {
            span { class: "nav-prompt", "iwa@iwaservice:~$" }
            Link { to: Route::Home {}, "~/" }
            Link { to: Route::Projects {}, "projects" }
            Link { to: Route::Blog {}, "blog" }
            Link { to: Route::About {}, "about" }
            button { class: "reboot-btn", onclick: reboot, "⟳ reboot" }
        }
    }
}