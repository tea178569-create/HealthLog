use crate::domain::model::Measurement;
use chrono::Timelike;

/// 与えられた測定データのリストから、最も測定密度の高い「連続する4時間」の開始時刻を算出する
///
/// # アルゴリズム
/// 1. 各Measurementのtimestampから「時(hour)」のみを抽出する（0..=23）
/// 2. 各時間帯のデータ数をカウントしてヒストグラムを作成する
/// 3. スライディングウィンドウ（幅4時間）を用いて、全時間帯を走査する
///    - ウィンドウは日またがり（22時〜1時など）も考慮して円環(modulo)として扱う
/// 4. 最もデータ数が多かったウィンドウの「開始時刻(hour)」を返す
///    - 最大値が複数ある場合は、より早い時刻（ただし日またがりを除く）を優先するなどの単純なルールを適用（現状は最初に見つかったものを返す）
pub fn calculate_best_4h_window(measurements: &[Measurement]) -> u32 {
    if measurements.is_empty() {
        return 0; // データがない場合はデフォルトで0時を返す
    }

    // 1 & 2: 時刻ごとのヒストグラムを作成 (0..=23)
    let mut histogram = [0; 24];
    for m in measurements {
        let hour = m.timestamp.hour() as usize;
        histogram[hour] += 1;
    }

    // 3 & 4: スライディングウィンドウで最大密度の区間を探索
    let mut max_count = 0;
    let mut best_start_hour = 0;

    for start_hour in 0..24 {
        let mut current_count = 0;
        // 4時間のウィンドウを計算（日またがりは modulo 24 で対応）
        for offset in 0..4 {
            let h = (start_hour + offset) % 24;
            current_count += histogram[h];
        }

        // 最大値を更新
        if current_count > max_count {
            max_count = current_count;
            best_start_hour = start_hour;
        }
    }

    best_start_hour as u32
}

/// 算出されたベストな4時間ウィンドウに含まれるMeasurementのみを抽出する
pub fn filter_by_best_window(
    measurements: Vec<Measurement>,
    best_start_hour: u32,
) -> Vec<Measurement> {
    measurements
        .into_iter()
        .filter(|m| {
            let h = m.timestamp.hour();
            // 日またがりのウィンドウかどうかで条件分岐
            let end_hour = (best_start_hour + 3) % 24;
            if best_start_hour <= end_hour {
                // 通常のウィンドウ (例: 6時〜9時)
                h >= best_start_hour && h <= end_hour
            } else {
                // 日またがりのウィンドウ (例: 22時〜1時)
                h >= best_start_hour || h <= end_hour
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::{MetricType, Source};
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    // テスト用のヘルパー関数: 指定した時刻のMeasurementを作成する
    fn create_dummy_measurement(hour: u32, minute: u32) -> Measurement {
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
        Measurement {
            id: None,
            metric_type: MetricType::Weight,
            timestamp: NaiveDateTime::new(date, time),
            value: 0.0,
            source: Source::Manual,
            is_outlier: false,
        }
    }

    #[test]
    fn test_calculate_best_4h_window_basic() {
        let measurements = vec![
            create_dummy_measurement(5, 0),
            create_dummy_measurement(6, 30),
            create_dummy_measurement(7, 45),
            create_dummy_measurement(8, 15),
            create_dummy_measurement(12, 0), // 範囲外
            create_dummy_measurement(18, 0), // 範囲外
        ];

        // 5, 6, 7, 8時に集中しているので、ベストウィンドウの開始は5時になるはず
        let best_start = calculate_best_4h_window(&measurements);
        assert_eq!(best_start, 5);
    }

    #[test]
    fn test_calculate_best_4h_window_midnight_crossing() {
        let measurements = vec![
            create_dummy_measurement(12, 0), // 範囲外
            create_dummy_measurement(22, 30),
            create_dummy_measurement(23, 45),
            create_dummy_measurement(0, 15),
            create_dummy_measurement(1, 10),
            create_dummy_measurement(10, 0), // 範囲外
        ];

        // 22, 23, 0, 1時に集中しているので、ベストウィンドウの開始は22時になるはず（日またがり）
        let best_start = calculate_best_4h_window(&measurements);
        assert_eq!(best_start, 22);
    }

    #[test]
    fn test_filter_by_best_window() {
        let measurements = vec![
            create_dummy_measurement(5, 0),   // inside (5)
            create_dummy_measurement(6, 30),  // inside (6)
            create_dummy_measurement(12, 0),  // outside (12)
            create_dummy_measurement(23, 45), // outside (23)
            create_dummy_measurement(8, 15),  // inside (8)
            create_dummy_measurement(9, 0), // outside (9 - ends at 8:59:59 conceptually, but our logic checks `h <= end_hour` so 8 is max)
        ];

        // 5時開始のウィンドウ (5, 6, 7, 8) でフィルタリング
        let filtered = filter_by_best_window(measurements, 5);

        assert_eq!(filtered.len(), 3);
        assert_eq!(filtered[0].timestamp.hour(), 5);
        assert_eq!(filtered[1].timestamp.hour(), 6);
        assert_eq!(filtered[2].timestamp.hour(), 8);
    }

    #[test]
    fn test_filter_by_best_window_midnight_crossing() {
        let measurements = vec![
            create_dummy_measurement(21, 0),  // outside
            create_dummy_measurement(22, 30), // inside
            create_dummy_measurement(23, 45), // inside
            create_dummy_measurement(0, 15),  // inside
            create_dummy_measurement(1, 10),  // inside
            create_dummy_measurement(2, 0),   // outside
        ];

        // 22時開始のウィンドウ (22, 23, 0, 1) でフィルタリング
        let filtered = filter_by_best_window(measurements, 22);

        assert_eq!(filtered.len(), 4);
        assert_eq!(filtered[0].timestamp.hour(), 22);
        assert_eq!(filtered[1].timestamp.hour(), 23);
        assert_eq!(filtered[2].timestamp.hour(), 0);
        assert_eq!(filtered[3].timestamp.hour(), 1);
    }
}
