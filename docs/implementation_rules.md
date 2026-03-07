# 実装ルール

## コミットメッセージのルール

コミットの内容に応じて、以下の絵文字とプレフィックスを使用します。

### 絵文字・プレフィックス一覧

- 🔖 `:bookmark:` **release** (Version Tag)
- ✨ `:sparkles:` **feat** (New Feature)
- 🐝 `:bee:` **fix** (Bugfix)
- ♻️ `:recycle:` **refactor** (Refactoring)
- 📚 `:books:` **docs** (Documentation)
- 🎨 `:art:` **style** (Code Style)
- 🐎 `:horse:` **perform** (Performance)
- 🔧 `:wrench:` **tool** (Tooling)
- 🚨 `:rotating_light:` **test** (Tests)
- 🗑️ `:wastebasket:` **remove** (Removal)
- 🚧 `:construction:` **WIP** (Work In Progress)

### フォーマット

```text
:emojiprefix: Subject

Commit body...
```

**例:**
```text
✨feat: csvインポート機能の追加

オムロンの体組成データCSVをパースしてデータベースに保存する処理を追加。
```

## ディレクトリ構造とアーキテクチャ

**クリーンアーキテクチャ**を採用し、以下のディレクトリ構造で実装します。

```text
src/
├── domain/        # 外部依存なし (MetricType, Measurement, WindowCalculator)
├── usecase/       # ビジネスシナリオ (ImportCsv, GetFilteredTimeline, UpdatePreference)
├── infra/         # インフラストラクチャ実装
│   ├── db/        # データベース関連 (Diesel Schema, Migrations, SqliteRepo)
│   ├── parser/    # CSVパーサー (OmronParser, BloodLogParser)
│   └── ui/        # UI・ビュー (Dioxus Components, Views, EvalJS)
└── main.rs        # 依存性の注入 (Dependency Injection) とアプリケーションの起動
```