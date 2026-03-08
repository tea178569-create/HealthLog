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

#[component]
pub fn App() -> Element {
    let mut active_tab = use_signal(|| Tab::BodyComposition);

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
                    h2 {
                        style: "font-size: 1.8rem; font-weight: bold; color: #1f2937; margin-bottom: 20px; border-bottom: 2px solid #f3f4f6; padding-bottom: 10px;",
                        "{active_tab().label()}"
                    }

                    // タブごとのダミーコンテンツ
                    match active_tab() {
                        Tab::BodyComposition => rsx! { p { color: "#6b7280", "ここにオムロンの体組成データ（体重、体脂肪率など）のグラフと表が表示される予定だ。" } },
                        Tab::Vitals => rsx! { p { color: "#6b7280", "ここに献血ログから取得したバイタルデータ（血圧、脈拍）が表示される予定だ。" } },
                        Tab::Liver => rsx! { p { color: "#6b7280", "ここに肝機能のデータ（ALT, γ-GTP, TP, ALBなど）が表示される予定だ。" } },
                        Tab::Rbc => rsx! { p { color: "#6b7280", "ここに赤血球関連のデータ（RBC, Hb, Htなど）が表示される予定だ。" } },
                        Tab::Wbc => rsx! { p { color: "#6b7280", "ここに白血球・血小板のデータ（WBC, PLT）が表示される予定だ。" } },
                    }
                }
            }
        }
    }
}
