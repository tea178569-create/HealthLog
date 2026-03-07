use crate::domain::model::{Location, Measurement, MetricType, Source};
use chrono::NaiveDateTime;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct BloodLogRow {
    #[serde(rename = "日時")]
    date: String,
    #[serde(rename = "都道府県")]
    prefecture: Option<String>,
    #[serde(rename = "会場名")]
    venue_name: Option<String>,
    #[serde(rename = "種別")]
    donation_type: Option<String>,
    #[serde(rename = "体重")]
    weight: Option<f64>,
    #[serde(rename = "体脂肪率")]
    body_fat_percentage: Option<f64>,
    #[serde(rename = "骨格筋率")]
    skeletal_muscle_percentage: Option<f64>,
    #[serde(rename = "最高血圧")]
    systolic_bp: Option<f64>,
    #[serde(rename = "最低血圧")]
    diastolic_bp: Option<f64>,
    #[serde(rename = "脈拍")]
    pulse: Option<f64>,
    #[serde(rename = "ALT")]
    alt: Option<f64>,
    #[serde(rename = "GGT")]
    ggtp: Option<f64>,
    #[serde(rename = "TP")]
    total_protein: Option<f64>,
    #[serde(rename = "ALB")]
    albumin: Option<f64>,
    #[serde(rename = "A/G")]
    ag_ratio: Option<f64>,
    #[serde(rename = "CHOL")]
    cholesterol: Option<f64>,
    #[serde(rename = "RBC")]
    rbc: Option<f64>,
    #[serde(rename = "Hb")]
    hemoglobin: Option<f64>,
    #[serde(rename = "Ht")]
    hematocrit: Option<f64>,
    #[serde(rename = "MCV")]
    mcv: Option<f64>,
    #[serde(rename = "MCH")]
    mch: Option<f64>,
    #[serde(rename = "MCHC")]
    mchc: Option<f64>,
    #[serde(rename = "WBC")]
    wbc: Option<f64>,
    #[serde(rename = "PLT")]
    platelets: Option<f64>,
}

pub fn parse_blood_csv(
    csv_data: &str,
) -> Result<(Vec<Measurement>, Vec<Location>), Box<dyn Error>> {
    let data = csv_data.trim_start_matches('\u{feff}');

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data.as_bytes());

    let mut measurements = Vec::new();
    let mut locations = Vec::new();

    for result in rdr.deserialize() {
        let record: BloodLogRow = result?;
        let timestamp = NaiveDateTime::parse_from_str(&record.date, "%Y-%m-%d %H:%M:%S")?;

        // Extract Location if available
        if let (Some(prefecture), Some(venue_name), Some(donation_type)) =
            (record.prefecture, record.venue_name, record.donation_type)
        {
            locations.push(Location {
                id: None,
                timestamp,
                prefecture,
                venue_name,
                donation_type,
            });
        }

        // Helper macro to push measurement if value is present
        macro_rules! push_metric {
            ($value_opt:expr, $metric_type:expr) => {
                if let Some(value) = $value_opt {
                    measurements.push(Measurement {
                        id: None,
                        metric_type: $metric_type,
                        timestamp,
                        value,
                        source: Source::Csv,
                        is_outlier: false,
                    });
                }
            };
        }

        push_metric!(record.weight, MetricType::Weight);
        push_metric!(record.body_fat_percentage, MetricType::BodyFatPercentage);
        push_metric!(
            record.skeletal_muscle_percentage,
            MetricType::SkeletalMusclePercentage
        );
        push_metric!(record.systolic_bp, MetricType::SystolicBp);
        push_metric!(record.diastolic_bp, MetricType::DiastolicBp);
        push_metric!(record.pulse, MetricType::Pulse);
        push_metric!(record.alt, MetricType::Alt);
        push_metric!(record.ggtp, MetricType::Ggtp);
        push_metric!(record.total_protein, MetricType::TotalProtein);
        push_metric!(record.albumin, MetricType::Albumin);
        push_metric!(record.ag_ratio, MetricType::AgRatio);
        push_metric!(record.cholesterol, MetricType::Cholesterol);
        push_metric!(record.rbc, MetricType::Rbc);
        push_metric!(record.hemoglobin, MetricType::Hemoglobin);
        push_metric!(record.hematocrit, MetricType::Hematocrit);
        push_metric!(record.mcv, MetricType::Mcv);
        push_metric!(record.mch, MetricType::Mch);
        push_metric!(record.mchc, MetricType::Mchc);
        push_metric!(record.wbc, MetricType::Wbc);
        push_metric!(record.platelets, MetricType::Platelets);
    }

    Ok((measurements, locations))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dummy_blood_csv_file() {
        use std::fs;

        // CIでも実行可能なGit管理下のダミーファイルを読み込んでテストする
        let file_path = "tests/fixtures/dummy_blood.csv";
        let csv_data = fs::read_to_string(file_path).expect("Failed to read dummy file");

        let (measurements, locations) = parse_blood_csv(&csv_data).unwrap();

        // 3行分のデータが存在することを確認
        assert_eq!(locations.len(), 3);

        // 1件目のロケーションチェック
        assert_eq!(locations[0].prefecture, "東京都");
        assert_eq!(locations[0].venue_name, "○○クリニック");
        assert_eq!(locations[0].donation_type, "健康診断");

        // いずれかの測定データが抽出できていることを確認
        assert!(measurements.len() > 0);

        // 最初の行の体重(53.0)が含まれているかチェック
        let has_weight = measurements
            .iter()
            .any(|m| m.metric_type == MetricType::Weight && m.value == 53.0);
        assert!(has_weight);

        println!(
            "パース成功: ダミーファイルから {} 件の測定データと {} 件の場所データを抽出した！",
            measurements.len(),
            locations.len()
        );
    }
}
