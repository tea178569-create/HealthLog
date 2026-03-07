use crate::domain::model::{Location, Measurement, MetricType, Source};
use crate::infra::db::schema::{locations, measurements, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = users)]
pub struct DbUser {
    pub id: Option<i32>,
    pub birth_date: Option<String>,
    pub gender: Option<String>,
    pub height_cm: Option<f32>,
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = measurements)]
pub struct DbMeasurement {
    pub id: Option<i32>,
    pub metric_type: String,
    pub timestamp: String,
    pub value: f32,
    pub source: String,
    pub is_outlier: bool,
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = locations)]
pub struct DbLocation {
    pub id: Option<i32>,
    pub timestamp: String,
    pub prefecture: String,
    pub venue_name: String,
    pub donation_type: String,
}

// データベースモデルからドメインモデル（Measurement）への変換処理
impl From<DbMeasurement> for Measurement {
    fn from(db: DbMeasurement) -> Self {
        Measurement {
            id: db.id,
            metric_type: parse_metric_type(&db.metric_type),
            timestamp: NaiveDateTime::parse_from_str(&db.timestamp, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default(),
            value: db.value as f64,
            source: if db.source == "Csv" {
                Source::Csv
            } else {
                Source::Manual
            },
            is_outlier: db.is_outlier,
        }
    }
}

// ドメインモデルからデータベースモデル（DbMeasurement）への変換処理
impl From<Measurement> for DbMeasurement {
    fn from(m: Measurement) -> Self {
        DbMeasurement {
            id: m.id,
            metric_type: metric_type_to_string(&m.metric_type),
            timestamp: m.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            value: m.value as f32,
            source: match m.source {
                Source::Csv => "Csv".to_string(),
                Source::Manual => "Manual".to_string(),
            },
            is_outlier: m.is_outlier,
        }
    }
}

// データベースモデルからドメインモデル（Location）への変換処理
impl From<DbLocation> for Location {
    fn from(db: DbLocation) -> Self {
        Location {
            id: db.id,
            timestamp: NaiveDateTime::parse_from_str(&db.timestamp, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default(),
            prefecture: db.prefecture,
            venue_name: db.venue_name,
            donation_type: db.donation_type,
        }
    }
}

// ドメインモデルからデータベースモデル（DbLocation）への変換処理
impl From<Location> for DbLocation {
    fn from(loc: Location) -> Self {
        DbLocation {
            id: loc.id,
            timestamp: loc.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            prefecture: loc.prefecture,
            venue_name: loc.venue_name,
            donation_type: loc.donation_type,
        }
    }
}

// ドメインモデルのMetricType（Enum）をデータベース保存用の文字列に変換する
fn metric_type_to_string(m: &MetricType) -> String {
    match m {
        MetricType::Weight => "Weight",
        MetricType::BodyFatPercentage => "BodyFatPercentage",
        MetricType::BodyFatMass => "BodyFatMass",
        MetricType::VisceralFatLevel => "VisceralFatLevel",
        MetricType::SkeletalMusclePercentage => "SkeletalMusclePercentage",
        MetricType::SkeletalMuscleMass => "SkeletalMuscleMass",
        MetricType::Bmi => "Bmi",
        MetricType::BasalMetabolicRate => "BasalMetabolicRate",
        MetricType::BodyAge => "BodyAge",
        MetricType::SystolicBp => "SystolicBp",
        MetricType::DiastolicBp => "DiastolicBp",
        MetricType::Pulse => "Pulse",
        MetricType::Alt => "Alt",
        MetricType::Ggtp => "Ggtp",
        MetricType::TotalProtein => "TotalProtein",
        MetricType::Albumin => "Albumin",
        MetricType::AgRatio => "AgRatio",
        MetricType::Cholesterol => "Cholesterol",
        MetricType::Rbc => "Rbc",
        MetricType::Hemoglobin => "Hemoglobin",
        MetricType::Hematocrit => "Hematocrit",
        MetricType::Mcv => "Mcv",
        MetricType::Mch => "Mch",
        MetricType::Mchc => "Mchc",
        MetricType::Wbc => "Wbc",
        MetricType::Platelets => "Platelets",
    }
    .to_string()
}

// データベース文字列からドメインモデルのMetricType（Enum）に変換する
fn parse_metric_type(s: &str) -> MetricType {
    match s {
        "Weight" => MetricType::Weight,
        "BodyFatPercentage" => MetricType::BodyFatPercentage,
        "BodyFatMass" => MetricType::BodyFatMass,
        "VisceralFatLevel" => MetricType::VisceralFatLevel,
        "SkeletalMusclePercentage" => MetricType::SkeletalMusclePercentage,
        "SkeletalMuscleMass" => MetricType::SkeletalMuscleMass,
        "Bmi" => MetricType::Bmi,
        "BasalMetabolicRate" => MetricType::BasalMetabolicRate,
        "BodyAge" => MetricType::BodyAge,
        "SystolicBp" => MetricType::SystolicBp,
        "DiastolicBp" => MetricType::DiastolicBp,
        "Pulse" => MetricType::Pulse,
        "Alt" => MetricType::Alt,
        "Ggtp" => MetricType::Ggtp,
        "TotalProtein" => MetricType::TotalProtein,
        "Albumin" => MetricType::Albumin,
        "AgRatio" => MetricType::AgRatio,
        "Cholesterol" => MetricType::Cholesterol,
        "Rbc" => MetricType::Rbc,
        "Hemoglobin" => MetricType::Hemoglobin,
        "Hematocrit" => MetricType::Hematocrit,
        "Mcv" => MetricType::Mcv,
        "Mch" => MetricType::Mch,
        "Mchc" => MetricType::Mchc,
        "Wbc" => MetricType::Wbc,
        "Platelets" => MetricType::Platelets,
        _ => MetricType::Weight, // Default fallback
    }
}
