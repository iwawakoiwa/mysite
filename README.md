# iwaservice.uk

個人サイト。Dioxus + Axum + Tailwind CSS で構築。

## 技術スタック

- **Dioxus 0.7** — Rust製UIフレームワーク（WASM）
- **Axum** — サーバーサイド
- **gloo-timers** — WASMタイマー
- **dioxus-router** — SPAルーティング
- **Tailwind CSS + boot.css** — スタイル（Catppuccin Mocha）

## ファイル構成

```
src/
├── main.rs                 # エントリポイント（launch のみ）
├── app.rs                  # App・Route・SiteLayout・AppState
├── components/
│   ├── mod.rs
│   ├── boot.rs             # BootScreen（起動アニメーション）
│   ├── header.rs           # Header（ナビ + リブートボタン）
│   ├── project_card.rs     # ProjectCard / RecentProjects
│   └── projects_data.rs    # PROJECTS 定数（データ一元管理）
└── pages/
    ├── mod.rs
    ├── home.rs             # Home（2カラム）
    ├── about.rs            # About
    └── projects.rs         # Projects（全件）

assets/
└── boot.css                # 全スタイル（Catppuccin Mocha）
```

## 開発

```sh
dx serve --platform web
```

## 方針

- コンポーネントは `components/`、ページは `pages/` に分離する
- データは `projects_data.rs` で一元管理
- スタイルは基本 `boot.css`、インラインは最小限
- Dioxus 0.7 の API に合わせる（`use_signal` / `use_effect` / `spawn`）
