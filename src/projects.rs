
use dioxus::prelude::*;
use crate::projects_data::PROJECTS;
use crate::project_card::ProjectCard;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section {
            class: "projects-page",
            style: "max-width: 1100px; margin: 0 auto; padding: 4rem 2rem;",
            div { class: "hero-tag", "// projects" }
            div { class: "hero-name", "Projects" }
            div { class: "projects-grid",
                for project in PROJECTS.iter() {
                    ProjectCard {
                        name: project.name,
                        desc: project.desc,
                        tags: project.tags.to_vec(),
                        status: project.status,
                    }
                }
            }
        }
    }
}