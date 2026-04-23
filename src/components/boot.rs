use dioxus::prelude::*;

struct Line {
    status: &'static str, // "ok" | "wait" | "warn"
    label: &'static str,
    detail: &'static str,
    delay_ms: u32,
}

const LINES: &[Line] = &[
    Line { status: "ok",   label: "Mounting", detail: "/portfolio",                    delay_ms: 50 },
    Line { status: "ok",   label: "Starting", detail: "iwaservice.service",            delay_ms: 60 },
    Line { status: "ok",   label: "Started",  detail: "Rust/Axum backend",             delay_ms: 45  },
    Line { status: "ok",   label: "Started",  detail: "Dioxus frontend renderer",      delay_ms: 55 },
    Line { status: "wait", label: "Loading",  detail: "blog entries...",               delay_ms: 160 },
    Line { status: "ok",   label: "Loaded",   detail: "12 posts found",                delay_ms: 40  },
    Line { status: "ok",   label: "Started",  detail: "project registry",              delay_ms: 50 },
    Line { status: "warn", label: "Warning",  detail: "too many projects in progress", delay_ms: 80 },
    Line { status: "ok",   label: "Started",  detail: "SSH daemon (Tailscale)",        delay_ms: 45  },
    Line { status: "ok",   label: "Reached",  detail: "multi-user.target",             delay_ms: 50 },
];

/// ブート画面コンポーネント。
/// 起動ログを流し終えたら `on_enter` を呼び出します。
#[component]
pub fn BootScreen(on_enter: EventHandler<()>) -> Element {
    let mut visible_count = use_signal(|| 0usize);
    let mut show_login = use_signal(|| false);
    let mut fading = use_signal(|| false);

    // ブート完了 → フェードアウト → on_enter
    let do_enter = move || {
        if fading() {
            return;
        }
        spawn(async move {
            fading.set(true);
            gloo_timers::future::TimeoutFuture::new(1100).await;
            on_enter.call(());
        });
    };

    // 起動シーケンス（マウント時に一度だけ実行）
    use_effect(move || {
        spawn(async move {
            for (i, line) in LINES.iter().enumerate() {
                // 疑似ランダムなゆらぎ（i を使って変化をつける）
                let jitter = ((i * 37) % 80) as u32;
                gloo_timers::future::TimeoutFuture::new(line.delay_ms + jitter).await;
                *visible_count.write() = i + 1;
            }
            gloo_timers::future::TimeoutFuture::new(400).await;
            show_login.set(true);
            gloo_timers::future::TimeoutFuture::new(1000).await;
            do_enter();
        });
    });

    rsx! {
        div { class: if fading() { "boot fade-out" } else { "boot" },

            // Arch ロゴ
            // pre { class: "arch-logo",
            //     "   /\\        _\n  /  \\      | |__  __ _\n / /\\ \\     | '_ \\/ _` |\n/ ____ \\  _ | | | \\__,_|\n/_/    \\_\\(_)|_| |_|__,|"
            // }

            // ブートログ
            div { class: "boot-lines",
                for (i , line) in LINES.iter().enumerate() {
                    if i < visible_count() {
                        div { class: "boot-line visible",
                            span {
                                class: match line.status {
                                    "ok" => "status ok",
                                    "wait" => "status wait",
                                    _ => "status warn",
                                },
                                match line.status {
                                    "ok" => "[  OK  ]",
                                    "wait" => "[ WAIT ]",
                                    _ => "[ WARN ]",
                                }
                            }
                            span { class: "line-label",
                                "{line.label} "
                                span { class: "line-detail", "{line.detail}" }
                            }
                        }
                    }
                }
            }

            // ログインプロンプト
            if show_login() {
                div { class: "login-prompt",
                    span { class: "host", "iwaservice" }
                    span { class: "sep", " login: " }
                    span { class: "cursor-blink" }
                }
            }

            // スキップボタン
            button { class: "skip-btn", onclick: move |_| do_enter(), "skip [Enter]" }
        }
    }
}
