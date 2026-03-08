#![allow(dead_code)]

mod domain;
mod infra;
mod usecase;

use infra::db::sqlite_repo::SqliteRepository;
use infra::ui::App;
use usecase::import::initialize_dummy_data;

fn main() {
    println!("Starting MyHealthLog application...");

    // データベース接続とダミーデータの初期化
    let mut repo = SqliteRepository::new();
    if let Err(e) = initialize_dummy_data(&mut repo) {
        eprintln!("Failed to initialize DB: {}", e);
    }

    // Dioxus デスクトップアプリを起動
    dioxus::launch(App);
}
