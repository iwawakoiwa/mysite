# boot.css

iwaservice.uk のスタイルシート。Catppuccin Mocha ベース。

## 構成

| # | セクション | 内容 |
|---|-----------|------|
| 1 | Design Tokens | CSS変数（カラーパレット・サーフェス） |
| 2 | Reset | マージン・ボックスモデル・ベーススタイル |
| 3 | Boot Screen | 起動アニメーション全体 |
| 4 | Site Shell | `.site` ラッパー・スキャンラインオーバーレイ |
| 5 | Nav | ナビゲーションバー・リブートボタン |
| 6 | Pages | Home / Hero / About のレイアウト |
| 7 | Components | タグ・プロジェクトカード・スキル・リンク |
| 8 | Utilities | `.hl-*` カラーヘルパー |
| 9 | Keyframes | `fadeIn` / `slideUp` / `blink` |

## カラーパレット

| 変数 | 色 | 用途 |
|-----|----|------|
| `--green`   | `#a6e3a1` | OK・Rust・アクセント |
| `--blue`    | `#89b4fa` | リンク・nav アクティブ |
| `--red`     | `#f38ba8` | Rust タグ・エラー |
| `--yellow`  | `#f9e2af` | WARN・ESP32 |
| `--cyan`    | `#89dceb` | サブタイトル・VR |
| `--mauve`   | `#cba6f7` | ロボット・趣味系 |
| `--white`   | `#cdd6f4` | メインテキスト |
| `--dim`     | `#6c7086` | サブテキスト・キー名 |
| `--bg`      | `#0a0a0f` | 背景 |
| `--surface` | `#11111b` | カード背景 |

## ユーティリティクラス

### カラー

```
.hl-green  .hl-blue  .hl-red  .hl-yellow  .hl-cyan  .hl-mauve  .hl-dim
```

### タグ（`.tag` + 修飾子）

```
.tag.rust  .tag.esp32  .tag.linux  .tag.robot  .tag.vr
```

### プロジェクトステータス

```
.status-active   → --green
.status-done     → --blue
.status-wip      → --yellow
```

## ページを追加するとき

新しいページには `.hero` + `.hero-tag` / `.hero-name` を使うとアニメーション付きで一貫したレイアウトになる。

```css
/* hero-tag・hero-name・hero-sub・hero-desc に slideUp アニメが適用済み */
.hero-tag  { animation-delay: 0.2s; }
.hero-name { animation-delay: 0.35s; }
.hero-sub  { animation-delay: 0.5s; }
.hero-desc { animation-delay: 0.65s; }
```
