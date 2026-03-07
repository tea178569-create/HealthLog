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
- 🔀 `:twisted_rightwards_arrows:` **merge** (Merge Branch)

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

## コーディング規約

### コメント・ログ出力のテイスト
コード内のコメントやログ出力は、以下のルールで統一します。

- **文体**: 「だ/である」調を使用し、「です/ます」調は避ける。ただし、「である」という直接的な表現は極力避け、体言止めなど自然な言い回しにする。
- **句読点**:
  - 句点（。）は基本的に付けない。
  - 読点（、）は使用せず、文の区切りがどうしても必要な場合のみカンマと半角スペース（`, `）を使用する。

## Gitブランチ戦略

開発は `develop` ブランチをベースとし、作業内容に応じて以下のプレフィックスを持つブランチを作成して進めます。
作業が完了したら、Pull Request（PR）を作成し `develop` へマージします。
マージ後は不要になったフィーチャーブランチ（`feature/xxx`等）を削除します。

### ブランチの種類と役割

- **`main`**: リリース用（常にデプロイ・配布可能な安定版コードを配置）
- **`develop`**: 開発用（次のリリースに向けた最新のコードを集約するベースブランチ）
- **`feature/xxx`**: 新機能の追加用（例: `feature/blood-log-parser`）
- **`refactor/xxx`**: 既存コードの改善・リファクタリング用
- **`fix/xxx`**: バグ修正用（例: `fix/csv-parse-error`）
