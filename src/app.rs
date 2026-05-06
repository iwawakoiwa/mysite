// src/app.rs
use dioxus::prelude::*;
use crate::components::{BootScreen, Header};
use crate::pages::{Home, About, Projects, ProjectDetail, PostPage};

#[derive(Clone, PartialEq)]
pub enum AppState {
    Boot,
    Site,
}

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(SiteLayout)]
    #[route("/")]
    Home {},
    #[route("/projects")]
    Projects {},
    #[route("/projects/:slug")]
    ProjectDetail { slug: String },
    #[route("/projects/:slug/:post_slug")]
    PostPage { slug: String, post_slug: String },
    #[route("/about")]
    About {},
}

#[component]
pub fn App() -> Element {
    let mut state = use_signal(|| AppState::Boot);
    let mut site_visible = use_signal(|| false);

    use_context_provider(|| state);

    let enter_site = move |_| {
        state.set(AppState::Site);
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(50).await;
            site_visible.set(true);
        });
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/boot.css") }
        document::Link {
            rel: "icon",
            r#type: "image/x-icon",
            href: asset!("/assets/favicon.ico")
        }

        if state() == AppState::Boot {
            BootScreen { on_enter: enter_site }
        }
        div { class: if site_visible() { "site visible" } else { "site" },
            Router::<Route> {}
        }
    }
}

#[component]
fn SiteLayout() -> Element {
    rsx! {
        Header {}
        Outlet::<Route> {}
    }
}
