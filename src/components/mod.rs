// src/components/mod.rs
mod boot;
mod header;
mod project_card;
pub mod projects_data; 

pub use boot::BootScreen;
pub use header::Header;
pub use project_card::{ProjectCard, RecentProjects};