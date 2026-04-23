pub struct Project {
    pub name: &'static str,
    pub desc: &'static str,
    pub tags: &'static [(&'static str, &'static str)],
    pub status: &'static str,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "水面ロボット",
        desc: "競技用水面走行ロボット。ESP32 + TB6612FNG + LiFePO4。",
        tags: &[("ESP32", "hl-green"), ("KiCad", "hl-mauve"), ("Rust", "hl-red")],
        status: "active",
    },
    Project {
        name: "VVVF再現",
        desc: "ステッパーモーターでドレミファインバーターを物理再現。",
        tags: &[("ESP32", "hl-green"), ("NEMA17", "hl-cyan"), ("A4988", "hl-dim")],
        status: "active",
    },
    Project {
        name: "iwaservice.uk",
        desc: "このサイト。Dioxus + Axum + Docker で構築。",
        tags: &[("Rust", "hl-red"), ("Dioxus", "hl-blue"), ("Docker", "hl-cyan")],
        status: "active",
    },
    Project {
        name: "Robot",
        desc: "ROS 2 Jazzy + Docker + GitHub Actions で構築した競技ロボット。",
        tags: &[("ROS2", "hl-blue"), ("Docker", "hl-cyan"), ("C++", "hl-yellow")],
        status: "active",
    },
    Project {
        name: "SlimeVR自作",
        desc: "ESP32 + MPU9250 で作るVRフルボディトラッキングシステム。",
        tags: &[("ESP32", "hl-green"), ("VRChat", "hl-mauve"), ("Rust", "hl-red")],
        status: "wip",
    },
];
