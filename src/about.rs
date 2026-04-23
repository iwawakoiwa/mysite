use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section { class: "about",

            // ヘッダー
            div { class: "about-header",
                div { class: "hero-tag", "// about" }
                div { class: "hero-name", "Iwa" }
                div { class: "hero-sub",
                    span { class: "accent", "Engineer" }
                    " & Maker"
                }
            }

            div { class: "about-grid",

                // 左カラム：プロフィール
                div { class: "about-card",
                    div { class: "card-title",
                        span { class: "card-prompt", "$ " }
                        "whoami"
                    }
                    div { class: "card-body",
                        div { class: "about-row",
                            span { class: "about-key", "name" }
                            span { class: "about-val", "Iwa" }
                        }
                        div { class: "about-row",
                            span { class: "about-key", "location" }
                            span { class: "about-val", "JP 🇯🇵" }
                        }
                        div { class: "about-row",
                            span { class: "about-key", "school" }
                            span { class: "about-val hl-blue", "College of Technology" }
                        }
                        div { class: "about-row",
                            span { class: "about-key", "club" }
                            span { class: "about-val hl-green", "lobots" }
                        }
                        div { class: "about-row",
                            span { class: "about-key", "editor" }
                            span { class: "about-val hl-yellow", "Neovim" }
                        }
                        div { class: "about-row",
                            span { class: "about-key", "shell" }
                            span { class: "about-val hl-green", "fish" }
                        }
                        div { class: "about-row",
                            span { class: "about-key", "os" }
                            span { class: "about-val hl-blue", "Arch Linux + Hyprland" }
                        }
                    }
                }

                // 右カラム：スキル
                div { class: "about-card",
                    div { class: "card-title",
                        span { class: "card-prompt", "$ " }
                        "cat skills.toml"
                    }
                    div { class: "card-body",
                        SkillGroup {
                            label: "languages",
                            items: vec![
                                ("Rust", "hl-red"),
                                ("Python", "hl-yellow"),
                                ("C/C++", "hl-blue"),
                                ("JavaScript", "hl-yellow"),
                            ],
                        }
                        SkillGroup {
                            label: "embedded",
                            items: vec![
                                ("ESP32", "hl-green"),
                                ("Arduino", "hl-cyan"),
                                ("KiCad", "hl-mauve"),
                                ("A4988", "hl-dim"),
                            ],
                        }
                        SkillGroup {
                            label: "infra",
                            items: vec![
                                ("Docker", "hl-blue"),
                                ("Tailscale", "hl-cyan"),
                                ("GitHub Actions", "hl-dim"),
                                ("Axum", "hl-red"),
                            ],
                        }
                    }
                }

                // 下段：リンク
                div { class: "about-card about-card-wide",
                    div { class: "card-title",
                        span { class: "card-prompt", "$ " }
                        "ls ~/links"
                    }
                    div { class: "card-body links-row",
                        a {
                            class: "link-item",
                            href: "https://github.com",
                            target: "_blank",
                            span { class: "link-icon hl-dim", "→ " }
                            span { class: "hl-blue", "GitHub" }
                        }
                        a {
                            class: "link-item",
                            href: "https://iwaservice.uk",
                            target: "_blank",
                            span { class: "link-icon hl-dim", "→ " }
                            span { class: "hl-green", "iwaservice.uk" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillGroup(label: &'static str, items: Vec<(&'static str, &'static str)>) -> Element {
    rsx! {
        div { class: "skill-group",
            span { class: "skill-label hl-dim", "[{label}]" }
            div { class: "skill-items",
                for (name , color) in items {
                    span { class: "skill-chip {color}", "{name}" }
                }
            }
        }
    }
}