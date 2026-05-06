pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub project_slug: &'static str,
    pub content: &'static str,
}

pub const POSTS: &[Post] = &[
    Post {
        slug: "hajimari",
        title: "プロジェクト開始",
        date: "2025-01-10",
        project_slug: "mizumo-robot",
        content: include_str!("../../posts/mizumo-robot/2025-01-10-hajimari.md"),
    },
    Post {
        slug: "hajimari",
        title: "VVVFインバーターの実装",
        date: "2025-02-01",
        project_slug: "vvvf",
        content: include_str!("../../posts/vvvf/2025-02-01-hajimari.md"),
    },
    Post {
        slug: "hajimari",
        title: "サイト構築記録",
        date: "2025-03-01",
        project_slug: "iwaservice",
        content: include_str!("../../posts/iwaservice/2025-03-01-hajimari.md"),
    },
    Post {
        slug: "hajimari",
        title: "競技ロボット開発開始",
        date: "2025-04-01",
        project_slug: "robot",
        content: include_str!("../../posts/robot/2025-04-01-hajimari.md"),
    },
    Post {
        slug: "hajimari",
        title: "SlimeVR自作を始める",
        date: "2025-05-01",
        project_slug: "slimevr",
        content: include_str!("../../posts/slimevr/2025-05-01-hajimari.md"),
    },
];
