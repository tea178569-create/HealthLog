#![allow(dead_code)]

mod domain;
mod infra;

use infra::ui::App;

fn main() {
    println!("Starting MyHealthLog application...");

    // Dioxus デスクトップアプリを起動
    dioxus::launch(App);
}
