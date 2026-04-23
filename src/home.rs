use crate::project_card::RecentProjects;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        section { class: "home",

            // 左：ヒーロー
            div { class: "home-left",
                div { class: "hero-tag", "// hello, world" }
                div { class: "hero-name", "Iwa" }
                div { class: "hero-sub",
                    span { class: "accent", "Engineer" }
                    " & Maker"
                }
                div { class: "hero-desc",
                    span { class: "hl-green", "Rust" }
                    " / embedded systems / robotics."
                    br {}
                    span { class: "hl-yellow", "ESP32" }
                    " で動くものを作るのが好き。"
                    br {}
                    span { class: "hl-blue", "Arch Linux" }
                    " + Hyprland で生活しています。"
                }
                div { class: "tags",
                    span { class: "tag rust", "Rust" }
                    span { class: "tag esp32", "ESP32" }
                    span { class: "tag linux", "Arch Linux" }
                    span { class: "tag robot", "Robotics" }
                    span { class: "tag vr", "VRChat" }
                }
            }

            // 右：最近のプロジェクト
            RecentProjects {}
        
        }
    }
}
