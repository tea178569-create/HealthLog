use crate::domain::model::{Measurement, MetricType};
use crate::domain::window::{calculate_best_4h_window, filter_by_best_window};
use crate::infra::db::sqlite_repo::SqliteRepository;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    BodyComposition,
    Vitals,
    Liver,
    Rbc,
    Wbc,
}

impl Tab {
    pub fn label(&self) -> &'static str {
        match self {
            Tab::BodyComposition => "体組成",
            Tab::Vitals => "バイタル",
            Tab::Liver => "肝機能",
            Tab::Rbc => "赤血球",
            Tab::Wbc => "白血球・血小板",
        }
    }
}

// データベースから取得したデータを表示用のプロパティとしてコンポーネントに渡す
#[component]
fn MeasurementTable(measurements: Vec<Measurement>, title: String) -> Element {
    rsx! {
        div {
            h3 { style: "font-size: 1.2rem; font-weight: bold; margin-top: 20px; margin-bottom: 10px;", "{title}" }
            table {
                style: "width: 100%; border-collapse: collapse; text-align: left;",
                thead {
                    tr {
                        style: "background-color: #f3f4f6; border-bottom: 2px solid #e5e7eb;",
                        th { style: "padding: 10px;", "測定日時" }
                        th { style: "padding: 10px;", "測定項目" }
                        th { style: "padding: 10px;", "値" }
                    }
                }
                tbody {
                    for m in measurements {
                        tr {
                            style: "border-bottom: 1px solid #e5e7eb;",
                            td { style: "padding: 10px;", "{m.timestamp}" }
                            td { style: "padding: 10px;", "{m.metric_type:?}" }
                            td { style: "padding: 10px; font-weight: bold;", "{m.value}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut active_tab = use_signal(|| Tab::BodyComposition);
    let mut all_measurements = use_signal(Vec::<Measurement>::new);
    let mut is_window_optimized = use_signal(|| false);

    // 初回レンダリング時にデータベースから全データを取得
    use_effect(move || {
        let mut repo = SqliteRepository::new();
        if let Ok(data) = repo.get_all_measurements() {
            all_measurements.set(data);
        }
    });

    // 表示するデータを決定（体組成タブ ＆ 最適化トグルがONの場合のみフィルタリングする）
    let displayed_measurements = if is_window_optimized() && active_tab() == Tab::BodyComposition {
        let best_hour = calculate_best_4h_window(&all_measurements());
        filter_by_best_window(all_measurements(), best_hour)
    } else {
        all_measurements()
    };

    rsx! {
        div {
            style: "display: flex; height: 100vh; font-family: sans-serif; background-color: #f9fafb;",

            // サイドバー (タブメニュー)
            div {
                style: "width: 250px; background-color: #ffffff; border-right: 1px solid #e5e7eb; padding: 20px; display: flex; flex-direction: column; gap: 10px;",
                h1 {
                    style: "font-size: 1.5rem; font-weight: bold; color: #111827; margin-bottom: 20px;",
                    "MyHealthLog"
                }

                // 4時間ウィンドウ最適化トグル（削除）

                for tab in [Tab::BodyComposition, Tab::Vitals, Tab::Liver, Tab::Rbc, Tab::Wbc] {
                    button {
                        style: "padding: 12px 16px; border-radius: 8px; border: none; text-align: left; font-size: 1rem; cursor: pointer; transition: all 0.2s;",
                        background_color: if active_tab() == tab { "#eff6ff" } else { "transparent" },
                        color: if active_tab() == tab { "#2563eb" } else { "#4b5563" },
                        font_weight: if active_tab() == tab { "bold" } else { "normal" },
                        onclick: move |_| active_tab.set(tab),
                        "{tab.label()}"
                    }
                }
            }

            // メインコンテンツエリア
            div {
                style: "flex: 1; padding: 40px; overflow-y: auto;",

                div {
                    style: "background-color: #ffffff; border-radius: 12px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1); padding: 30px; min-height: 400px;",
                    div {
                        style: "display: flex; justify-content: space-between; align-items: baseline; border-bottom: 2px solid #f3f4f6; padding-bottom: 10px; margin-bottom: 20px;",
                        h2 {
                            style: "font-size: 1.8rem; font-weight: bold; color: #1f2937; margin: 0;",
                            "{active_tab().label()} のデータ"
                        }
                        if is_window_optimized() && active_tab() == Tab::BodyComposition {
                            span {
                                style: "font-size: 0.9rem; color: #10b981; font-weight: bold; background-color: #d1fae5; padding: 4px 12px; border-radius: 9999px; margin-right: auto; margin-left: 15px;",
                                "✨最適化適用中 (最も密度の高い4時間のデータのみを表示)"
                            }
                        }

                        if active_tab() == Tab::BodyComposition {
                            label {
                                style: "display: flex; align-items: center; gap: 8px; cursor: pointer; padding: 8px 12px; background-color: #f3f4f6; border-radius: 8px; font-size: 0.9rem;",
                                input {
                                    r#type: "checkbox",
                                    checked: "{is_window_optimized()}",
                                    onchange: move |evt| is_window_optimized.set(evt.value() == "true")
                                }
                                "4時間ウィンドウ最適化"
                            }
                        }
                    }

                    // タブに応じたデータをフィルタリングして表示
                    match active_tab() {
                        Tab::BodyComposition => rsx! {
                            MeasurementTable {
                                measurements: displayed_measurements.into_iter().filter(|m| matches!(m.metric_type, MetricType::Weight | MetricType::BodyFatPercentage | MetricType::BodyFatMass | MetricType::VisceralFatLevel | MetricType::SkeletalMusclePercentage | MetricType::SkeletalMuscleMass | MetricType::Bmi | MetricType::BasalMetabolicRate | MetricType::BodyAge)).collect(),
                                title: "体組成データ一覧".to_string()
                            }
                        },
                        Tab::Vitals => rsx! {
                            MeasurementTable {
                                measurements: displayed_measurements.into_iter().filter(|m| matches!(m.metric_type, MetricType::SystolicBp | MetricType::DiastolicBp | MetricType::Pulse)).collect(),
                                title: "バイタルデータ一覧".to_string()
                            }
                        },
                        Tab::Liver => rsx! {
                            MeasurementTable {
                                measurements: displayed_measurements.into_iter().filter(|m| matches!(m.metric_type, MetricType::Alt | MetricType::Ggtp | MetricType::TotalProtein | MetricType::Albumin | MetricType::AgRatio | MetricType::Cholesterol)).collect(),
                                title: "肝機能データ一覧".to_string()
                            }
                        },
                        Tab::Rbc => rsx! {
                            MeasurementTable {
                                measurements: displayed_measurements.into_iter().filter(|m| matches!(m.metric_type, MetricType::Rbc | MetricType::Hemoglobin | MetricType::Hematocrit | MetricType::Mcv | MetricType::Mch | MetricType::Mchc)).collect(),
                                title: "赤血球データ一覧".to_string()
                            }
                        },
                        Tab::Wbc => rsx! {
                            MeasurementTable {
                                measurements: displayed_measurements.into_iter().filter(|m| matches!(m.metric_type, MetricType::Wbc | MetricType::Platelets)).collect(),
                                title: "白血球・血小板データ一覧".to_string()
                            }
                        },
                    }
                }
            }
        }
    }
}
