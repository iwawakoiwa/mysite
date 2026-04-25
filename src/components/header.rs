use dioxus::prelude::*;
use crate::app::Route;
use crate::app::AppState;

#[component]
pub fn Header() -> Element {
    let mut state: Signal<AppState> = use_context();
    let mut menu_open = use_signal(|| false);

    let reboot = move |_| {
        state.set(AppState::Boot);
    };

    let toggle = move |_| {
        menu_open.set(!menu_open());
    };

    rsx! {
        nav {
            span { class: "nav-prompt", "iwa@iwaservice:~$" }

            div { class: "nav-links",
                Link { to: Route::Home {}, "~/" }
                Link { to: Route::Projects {}, "projects" }
                Link { to: Route::About {}, "about" }
                button { class: "reboot-btn", onclick: reboot, "⟳ reboot" }
            }

            button { class: "burger-btn", onclick: toggle,
                if menu_open() {
                    "✕"
                } else {
                    "☰"
                }
            }
        }

        if menu_open() {
            div { class: "mobile-menu",
                Link {
                    to: Route::Home {},
                    onclick: move |_| menu_open.set(false),
                    "~/"
                }
                Link {
                    to: Route::Projects {},
                    onclick: move |_| menu_open.set(false),
                    "projects"
                }
                Link {
                    to: Route::About {},
                    onclick: move |_| menu_open.set(false),
                    "about"
                }
            }
        }
    }
}