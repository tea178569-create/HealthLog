use crate::domain::model::{Measurement, MetricType, Source};
use chrono::NaiveDateTime;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct OmronRow {
    #[serde(rename = "測定日")]
    date: String,
    #[serde(rename = "タイムゾーン")]
    _timezone: String,
    #[serde(rename = "体重(kg)")]
    weight: f64,
    #[serde(rename = "体脂肪(%)")]
    body_fat_percentage: f64,
    #[serde(rename = "体脂肪量(kg)")]
    body_fat_mass: f64,
    #[serde(rename = "内臓脂肪レベル")]
    visceral_fat_level: f64,
    #[serde(rename = "基礎代謝(kcal)")]
    basal_metabolic_rate: f64,
    #[serde(rename = "骨格筋(%)")]
    skeletal_muscle_percentage: f64,
    #[serde(rename = "骨格筋量(kg)")]
    skeletal_muscle_mass: f64,
    #[serde(rename = "BMI")]
    bmi: f64,
    #[serde(rename = "体年齢(才)")]
    body_age: f64,
    #[serde(rename = "機種")]
    _device: String,
}

pub fn parse_omron_csv(csv_data: &str) -> Result<Vec<Measurement>, Box<dyn Error>> {
    // BOM（Byte Order Mark）がある場合は除去する
    let data = csv_data.trim_start_matches('\u{feff}');

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data.as_bytes());

    let mut measurements = Vec::new();

    for result in rdr.deserialize() {
        let record: OmronRow = result?;
        let timestamp = NaiveDateTime::parse_from_str(&record.date, "%Y/%m/%d %H:%M")?;

        let metrics = [
            (MetricType::Weight, record.weight),
            (MetricType::BodyFatPercentage, record.body_fat_percentage),
            (MetricType::BodyFatMass, record.body_fat_mass),
            (MetricType::VisceralFatLevel, record.visceral_fat_level),
            (MetricType::BasalMetabolicRate, record.basal_metabolic_rate),
            (
                MetricType::SkeletalMusclePercentage,
                record.skeletal_muscle_percentage,
            ),
            (MetricType::SkeletalMuscleMass, record.skeletal_muscle_mass),
            (MetricType::Bmi, record.bmi),
            (MetricType::BodyAge, record.body_age),
        ];

        for (metric_type, value) in metrics {
            measurements.push(Measurement {
                id: None,
                metric_type,
                timestamp,
                value,
                source: Source::Csv,
                is_outlier: false,
            });
        }
    }

    Ok(measurements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_omron_csv() {
        let csv_data = "\u{feff}\"測定日\",\"タイムゾーン\",\"体重(kg)\",\"体脂肪(%)\",\"体脂肪量(kg)\",\"内臓脂肪レベル\",\"基礎代謝(kcal)\",\"骨格筋(%)\",\"骨格筋量(kg)\",\"BMI\",\"体年齢(才)\",\"機種\"\n\"2025/07/01 21:48\",\"Asia/Tokyo\",\"54.40\",\"26.8\",\"14.60\",\"4\",\"1206\",\"28.4\",\"15.40\",\"21.5\",\"31\",\"HBF-227T\"\n";

        let result = parse_omron_csv(csv_data).unwrap();

        assert_eq!(result.len(), 9);

        // Weight
        assert_eq!(result[0].metric_type, MetricType::Weight);
        assert_eq!(result[0].value, 54.40);
        assert_eq!(result[0].timestamp.to_string(), "2025-07-01 21:48:00");

        // Body Fat Percentage
        assert_eq!(result[1].metric_type, MetricType::BodyFatPercentage);
        assert_eq!(result[1].value, 26.8);

        // Body Age
        assert_eq!(result[8].metric_type, MetricType::BodyAge);
        assert_eq!(result[8].value, 31.0);
    }

    #[test]
    fn test_parse_dummy_omron_csv_file() {
        use std::fs;

        // CIでも実行可能なGit管理下のダミーファイルを読み込んでテストする
        let file_path = "tests/fixtures/dummy_omron.csv";
        let csv_data = fs::read_to_string(file_path).expect("Failed to read dummy file");

        let result = parse_omron_csv(&csv_data).unwrap();

        // ダミーファイル(5行のデータ)からデータが抽出できていることを確認
        // (1行につき9つの測定データが生成されるため, 45件になるはず)
        assert_eq!(result.len(), 45);
        println!(
            "パース成功: ダミーファイルから {} 件の測定データを抽出した！",
            result.len()
        );
    }
}
