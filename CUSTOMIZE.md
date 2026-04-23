# iwaservice カスタマイズガイド

## ファイル構成

```
src/
├── main.rs      ルーター・AppState・SiteLayout
├── boot.rs      起動アニメーション
├── header.rs    ナビゲーション
├── home.rs      トップページ
├── about.rs     自己紹介ページ
└── ...          ページを追加する場合はここに

assets/
└── boot.css     全スタイル
```

---

## よくあるカスタマイズ

### 名前・肩書きを変える
`home.rs` と `about.rs` を編集する。

```rust
// home.rs
div { class: "hero-name", "Iwa" }        // ← 名前
div { class: "hero-sub",
    span { class: "accent", "Engineer" } // ← 肩書き
    " & Maker"
}
```

---

### タグを増減する
`home.rs` の `tags` セクションを編集する。
色クラスは `hl-red` / `hl-green` / `hl-blue` / `hl-yellow` / `hl-cyan` / `hl-mauve` から選ぶ。

```rust
div { class: "tags",
    span { class: "tag rust",  "Rust"    }  // 追加・削除・変更自由
    span { class: "tag esp32", "ESP32"   }
    // span { class: "tag linux", "Linux" } // コメントアウトで非表示
}
```

---

### ブートログを変える
`boot.rs` の `LINES` 定数を編集する。

```rust
const LINES: &[Line] = &[
    Line { status: "ok",   label: "Mounting", detail: "/portfolio",   delay_ms: 100 },
    Line { status: "wait", label: "Loading",  detail: "好きな文字列", delay_ms: 300 },
    Line { status: "warn", label: "Warning",  detail: "警告っぽい文", delay_ms: 160 },
    // status は "ok" / "wait" / "warn" の3種類
    // delay_ms はその行が表示されるまでの待ち時間(ms)
];
```

---

### ページを追加する
① `src/newpage.rs` を作る

```rust
use dioxus::prelude::*;

#[component]
pub fn NewPage() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero-tag", "// newpage" }
            div { class: "hero-name", "New Page" }
        }
    }
}
```

② `main.rs` に登録する

```rust
mod newpage;
use newpage::NewPage;

// Route enum に追加
#[route("/newpage")]
NewPage {},
```

③ `header.rs` にリンクを追加する

```rust
Link { to: Route::NewPage {}, "newpage" }
```

---

### 色テーマを変える
`boot.css` の `:root` を編集する。
デフォルトは Catppuccin Mocha。

```css
:root {
  --green:   #a6e3a1;  /* OK ステータス・Rust タグなど */
  --blue:    #89b4fa;  /* リンク・アクセント           */
  --red:     #f38ba8;  /* Rust タグ                    */
  --yellow:  #f9e2af;  /* WARN ステータス               */
  --cyan:    #89dceb;  /* サブタイトルアクセント        */
  --mauve:   #cba6f7;  /* ロボット・VR タグ             */
  --white:   #cdd6f4;  /* メインテキスト                */
  --dim:     #6c7086;  /* サブテキスト・キー            */
  --bg:      #0a0a0f;  /* 背景                          */
  --surface: #11111b;  /* カード背景                    */
}
```

---

### ブートをスキップさせたくない / 毎回流したい
`main.rs` の `AppState` の初期値を変えるだけ。

```rust
// 常にブートを流す（デフォルト）
let mut state = use_signal(|| AppState::Boot);

// 常にスキップ（開発中に便利）
let mut state = use_signal(|| AppState::Site);
```

---

### フォントを変える
`boot.css` の `@import` と `font-family` を変える。

```css
/* Google Fonts の URL を差し替える */
@import url('https://fonts.googleapis.com/css2?family=YOUR_FONT&display=swap');

body {
  font-family: 'Your Font', monospace;
}
```

---

## カラークラス早見表

| クラス       | 色     | 用途例               |
|-------------|--------|---------------------|
| `hl-green`  | 緑     | Rust、OK、強調       |
| `hl-blue`   | 青     | リンク、Linux        |
| `hl-red`    | 赤     | エラー、Rust タグ    |
| `hl-yellow` | 黄     | 警告、ESP32          |
| `hl-cyan`   | 水色   | サブタイトル、VR     |
| `hl-mauve`  | 紫     | ロボット、趣味系     |
| `hl-dim`    | グレー | キー名、補足テキスト |
