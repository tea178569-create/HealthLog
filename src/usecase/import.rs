use crate::infra::db::sqlite_repo::SqliteRepository;
use crate::infra::parser::blood::parse_blood_csv;
use crate::infra::parser::omron::parse_omron_csv;
use std::error::Error;
use std::fs;

// アプリケーション起動時に、ダミーデータをDBにインポートする初期化処理
pub fn initialize_dummy_data(repo: &mut SqliteRepository) -> Result<(), Box<dyn Error>> {
    // 既存のデータがすでにある場合は何もしない（初期化済みとみなす）
    let existing_measurements = repo.get_all_measurements()?;
    if !existing_measurements.is_empty() {
        println!(
            "DB is already initialized with {} measurements.",
            existing_measurements.len()
        );
        return Ok(());
    }

    println!("Initializing DB with dummy data...");

    // 1. オムロンCSVのダミーデータを読み込んで保存
    let omron_csv_path = "tests/fixtures/dummy_omron.csv";
    if let Ok(csv_data) = fs::read_to_string(omron_csv_path) {
        #[allow(clippy::collapsible_if)]
        if let Ok(measurements) = parse_omron_csv(&csv_data) {
            let count = measurements.len();
            repo.insert_measurements(measurements)?;
            println!("Imported {count} omron measurements.");
        }
    }

    // 2. 献血ログCSVのダミーデータを読み込んで保存
    let blood_csv_path = "tests/fixtures/dummy_blood.csv";
    if let Ok(csv_data) = fs::read_to_string(blood_csv_path) {
        #[allow(clippy::collapsible_if)]
        if let Ok((measurements, locations)) = parse_blood_csv(&csv_data) {
            let m_count = measurements.len();
            let l_count = locations.len();
            repo.insert_measurements(measurements)?;
            repo.insert_locations(locations)?;
            println!("Imported {m_count} blood measurements and {l_count} locations.");
        }
    }

    println!("DB initialization completed.");
    Ok(())
}
