use dioxus::prelude::*;
use crate::app::Route;
use crate::components::projects_data::PROJECTS;

#[component]
pub fn RecentProjects() -> Element {
    rsx! {
        div { class: "home-right",
            div { class: "recent-title",
                span { class: "card-prompt", "$ " }
                "ls ~/projects --recent"
            }
            div { class: "recent-cards",
                for project in PROJECTS.iter().take(3) {
                    ProjectCard {
                        name: project.name,
                        slug: project.slug,
                        desc: project.desc,
                        tags: project.tags.to_vec(),
                        status: project.status,
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProjectCard(
    name: &'static str,
    slug: &'static str,
    desc: &'static str,
    tags: Vec<(&'static str, &'static str)>,
    status: &'static str,
) -> Element {
    rsx! {
        Link {
            to: Route::ProjectDetail { slug: slug.to_string() },
            class: "project-card-link",
            div { class: "project-card",
                div { class: "project-card-header",
                    span { class: "project-name", "{name}" }
                    span {
                        class: match status {
                            "active" => "project-status status-active",
                            "done" => "project-status status-done",
                            _ => "project-status status-wip",
                        },
                        match status {
                            "active" => "● active",
                            "done" => "✓ done",
                            _ => "… wip",
                        }
                    }
                }
                div { class: "project-desc", "{desc}" }
                div { class: "project-tags",
                    for (tag, color) in tags {
                        span { class: "skill-chip {color}", "{tag}" }
                    }
                }
            }
        }
    }
}
