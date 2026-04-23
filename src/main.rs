use dioxus::prelude::*;

mod boot;
mod home;
mod about;
mod header;
mod projects;
mod projects_data;
mod project_card;

use boot::BootScreen;
use home::Home;
use about::About;
use header::Header;
use projects::Projects;

#[derive(Clone, PartialEq)]
enum AppState {
    Boot,
    Site,
}

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[layout(SiteLayout)]
    #[route("/")]
    Home {},
    #[route("/projects")]
    Projects {},
    #[route("/blog")]
    Blog {},
    #[route("/about")]
    About {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    
    let mut state = use_signal(|| AppState::Boot);
    let mut site_visible = use_signal(|| false);

    // コンテキストに登録
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

        if state() == AppState::Boot {
            BootScreen { on_enter: enter_site }
        }

        div { class: if site_visible() { "site visible" } else { "site" }, Router::<Route> {} }
    }
}

#[component]
fn SiteLayout() -> Element {
    rsx! {
        Header {}
        Outlet::<Route> {}
    }
}

// #[component]
// fn Projects() -> Element {
//     rsx! {
//         section { class: "hero",
//             div { class: "hero-tag", "// projects" }
//             div { class: "hero-name", "Projects" }
//         }
//     }
// }

#[component]
fn Blog() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero-tag", "// blog" }
            div { class: "hero-name", "Blog" }
        }
    }
}