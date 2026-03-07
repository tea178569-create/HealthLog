use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Option<i32>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub height_cm: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    // Body Composition
    Weight,
    BodyFatPercentage,
    BodyFatMass,
    VisceralFatLevel,
    SkeletalMusclePercentage,
    SkeletalMuscleMass,
    Bmi,
    BasalMetabolicRate,
    BodyAge,

    // Vitals
    SystolicBp,
    DiastolicBp,
    Pulse,

    // Liver Function
    Alt,
    Ggtp,
    TotalProtein,
    Albumin,
    AgRatio,
    Cholesterol,

    // Red Blood Cells
    Rbc,
    Hemoglobin,
    Hematocrit,
    Mcv,
    Mch,
    Mchc,

    // WBC & Platelets
    Wbc,
    Platelets,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    Csv,
    Manual,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    pub id: Option<i32>,
    pub metric_type: MetricType,
    pub timestamp: NaiveDateTime,
    pub value: f64,
    pub source: Source,
    pub is_outlier: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub id: Option<i32>,
    pub timestamp: NaiveDateTime, // どの献血の時の場所かを記録するため日時が必要
    pub prefecture: String,
    pub venue_name: String,
    pub donation_type: String,
}
