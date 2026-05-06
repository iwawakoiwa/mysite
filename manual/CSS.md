# CSS カスタマイズガイド

対象ファイル: `assets/boot.css`

---

## ファイル構成

| セクション | 内容 |
|---|---|
| 1. Design Tokens | 色・背景の変数定義 |
| 2. Reset | ブラウザデフォルトのリセット |
| 3. Boot Screen | 起動アニメーション画面 |
| 4. Site Shell | サイト全体のラッパー |
| 5. Navigation | ナビバー |
| 6. Page Layouts | 各ページのレイアウト |
| 7. Components | カード・タグなどの共通部品 |
| 8. Utilities | 色付けヘルパークラス |
| 9. Animations | `@keyframes` |
| 10. Responsive | 全メディアクエリ（ここだけ見れば OK） |

---

## よくあるカスタマイズ

### 色を変える

セクション 1 の `:root` だけ編集すれば全体に反映される。

```css
:root {
  --green:   #a6e3a1;  /* OK ステータス、アクセント */
  --blue:    #89b4fa;  /* リンク、見出し */
  --red:     #f38ba8;  /* Rust タグなど */
  --yellow:  #f9e2af;  /* WARN、Python タグなど */
  --cyan:    #89dceb;  /* サブタイトル accent */
  --mauve:   #cba6f7;  /* VR・KiCad タグなど */
  --white:   #cdd6f4;  /* 本文テキスト */
  --dim:     #6c7086;  /* 薄いテキスト・ラベル */

  --bg:      #0a0a0f;  /* ページ背景 */
  --surface: #11111b;  /* カード背景 */
}
```

ベースを明るいテーマにしたいなら `--bg` と `--surface` を白系に、`--white` と `--dim` を暗い色に反転する。

---

### フォントを変える

セクション 2 の `html, body` と、各コンポーネントの `font-family` を変える。

```css
/* 現在の設定 */
html, body {
  font-family: 'JetBrains Mono', monospace;  /* 本文 */
}

.hero-name, .hero-sub {
  font-family: 'Geist Mono', monospace;  /* 大見出し */
}
```

Google Fonts の URL はファイル冒頭の `@import` で読み込んでいる。フォントを変える場合はそこも合わせて変更する。

---

### ブレークポイントを変える

セクション 10 の `@media` を編集する。現在の設定：

| ブレークポイント | 対象 |
|---|---|
| `max-width: 768px` | タブレット〜。Home を 1カラム化 |
| `max-width: 640px` | スマートフォン。ナビ・About・各ページを調整 |

---

### 新しいページを追加したとき

セクション 6 にページのレイアウトを追加し、スマホ対応はセクション 10 の `max-width: 640px` ブロック内に追記する。

```css
/* セクション 6 に追加 */
.my-new-page {
  min-height: 100vh;
  padding: 4rem max(4rem, 10vw);
}

/* セクション 10 に追加 */
@media (max-width: 640px) {
  .my-new-page { padding: 2rem 1.2rem; }
}
```

---

### 新しいタグ色を追加したとき

セクション 7 の `.tag.xxx` パターンで追加する。

```css
.tag.python { color: var(--yellow); border-color: var(--yellow); }
```

あとは Rust 側で `span { class: "tag python", "Python" }` と書くだけ。

---

### スキャンラインを消したいとき

セクション 4 の `.site::before` を削除またはコメントアウトする。

```css
/* .site::before { ... } */
```

---

### ブート画面のログを変えたいとき

CSS ではなく Rust 側を編集する: `src/components/boot.rs` の `LINES` 定数。

```rust
const LINES: &[Line] = &[
    Line { status: "ok", label: "Started", detail: "my service", delay_ms: 50 },
    // ...
];
```

- `status`: `"ok"` / `"wait"` / `"warn"` で色が変わる
- `delay_ms`: その行が表示されるまでの待機時間（ms）

---

## ユーティリティクラス一覧

テキストに色を付けるだけのクラス。Rust の `rsx!` 内でそのまま使える。

```
.hl-green   → --green  （#a6e3a1）
.hl-blue    → --blue   （#89b4fa）
.hl-red     → --red    （#f38ba8）
.hl-yellow  → --yellow （#f9e2af）
.hl-cyan    → --cyan   （#89dceb）
.hl-mauve   → --mauve  （#cba6f7）
.hl-dim     → --dim    （#6c7086）
```

使用例:
```rust
span { class: "hl-green", "成功" }
```
