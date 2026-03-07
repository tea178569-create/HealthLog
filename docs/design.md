# MyHealthLog 設計仕様書

## 1. アーキテクチャ
**クリーンアーキテクチャ**を採用し、以下の4層で構成する。
- **Domain**: 外部依存なし。計測型、ユーザー定義、4時間ウィンドウ算出アルゴリズム。
- **UseCase**: CSVインポート、グラフ用データ集計などのビジネスシナリオ。
- **Infrastructure**: Diesel(SQLite), Dioxus(UIフレームワーク), Chart.js連携。
- **Interface Adapter**: Repository実装、CSVパーサー、ViewModel変換。

## 2. データモデル (Entity)
- **User**: 生年月日、性別、身長（献血ログから抽出）。
- **Measurement**: 測定日時、項目名(Enum)、値、ソース(Enum)、異常点フラグ。
- **Location**: 都道府県、会場名、献血種別。

## 3. データベース設計 (Diesel / SQLite)
- `users`: 基本プロファイル。
- `metrics`: 項目名、単位、基準値（最小・最大）。
- `measurements`: 実際の数値。`metric_id` で外部参照。

## 4. UI/UX 仕様
- **Frontend**: Dioxus + WASM/WebView。
- **Chart**: Chart.js + `chartjs-plugin-annotation` (基準値用) + `chartjs-plugin-zoom`。
- **State Management**: Dioxusの `use_state` / `use_ref` によるリアクティブな表示更新。

## 5. アルゴリズム仕様
- **4時間ウィンドウ**: 時刻のヒストグラムを作成し、スライディングウィンドウで最大密度区間を決定。
- **異常点除去**: 移動平均からの $2\sigma$ 逸脱を検知対象とする（暫定）。
