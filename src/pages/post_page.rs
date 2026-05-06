use dioxus::prelude::*;
use crate::app::Route;
use crate::components::posts_data::POSTS;

fn md_to_html(src: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(src, pulldown_cmark::Options::all());
    let mut out = String::new();
    pulldown_cmark::html::push_html(&mut out, parser);
    out
}

#[component]
pub fn PostPage(slug: String, post_slug: String) -> Element {
    let post = POSTS.iter().find(|p| p.project_slug == slug && p.slug == post_slug);

    let Some(post) = post else {
        return rsx! {
            div { class: "project-detail",
                div { class: "hero-tag", "// 404" }
                div { class: "hero-name", "Not Found" }
            }
        };
    };

    let html = md_to_html(post.content);

    rsx! {
        section { class: "post-page",
            Link {
                to: Route::ProjectDetail { slug: slug.clone() },
                class: "post-back",
                "← /projects/{slug}"
            }

            div { class: "post-header",
                div { class: "hero-tag", "// devlog" }
                div { class: "hero-name", "{post.title}" }
                div { class: "post-meta-date", "{post.date}" }
            }

            div {
                class: "post-content",
                dangerous_inner_html: "{html}"
            }
        }
    }
}
