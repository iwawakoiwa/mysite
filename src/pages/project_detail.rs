use dioxus::prelude::*;
use crate::app::Route;
use crate::components::projects_data::PROJECTS;
use crate::components::posts_data::POSTS;

#[component]
pub fn ProjectDetail(slug: String) -> Element {
    let project = PROJECTS.iter().find(|p| p.slug == slug);

    let Some(project) = project else {
        return rsx! {
            div { class: "project-detail",
                div { class: "hero-tag", "// 404" }
                div { class: "hero-name", "Not Found" }
            }
        };
    };

    let posts: Vec<_> = POSTS.iter().filter(|p| p.project_slug == slug).collect();

    rsx! {
        section { class: "project-detail",
            Link { to: Route::Projects {}, class: "post-back", "← /projects" }

            div { class: "project-detail-header",
                div { class: "hero-tag", "// project" }
                div { class: "hero-name", "{project.name}" }
                div { class: "project-detail-meta",
                    span {
                        class: match project.status {
                            "active" => "project-status status-active",
                            "done"   => "project-status status-done",
                            _        => "project-status status-wip",
                        },
                        match project.status {
                            "active" => "● active",
                            "done"   => "✓ done",
                            _        => "… wip",
                        }
                    }
                }
                div { class: "project-tags", style: "margin-top: 0.8rem;",
                    for (tag, color) in project.tags.iter() {
                        span { class: "skill-chip {color}", "{tag}" }
                    }
                }
                div { class: "hero-desc", style: "margin-top: 1.2rem;",
                    "{project.desc}"
                }
            }

            div { class: "card-title", style: "margin-top: 2.5rem;",
                span { class: "card-prompt", "$ " }
                "ls ~/devlog/{slug}"
            }

            if posts.is_empty() {
                div { class: "hero-desc", style: "margin-top: 1rem;",
                    "まだ記事がありません。"
                }
            } else {
                div { class: "post-list",
                    for post in posts {
                        Link {
                            to: Route::PostPage { slug: slug.clone(), post_slug: post.slug.to_string() },
                            class: "post-list-item",
                            span { class: "post-date", "{post.date}" }
                            span { class: "post-title", "{post.title}" }
                        }
                    }
                }
            }
        }
    }
}
