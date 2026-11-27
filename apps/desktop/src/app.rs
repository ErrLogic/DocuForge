use dioxus::prelude::*;

use crate::components::{layout::AppHeader, pdf::MergePanel};

#[component]
pub fn App() -> Element {
    let mut light_theme = use_signal(|| false);
    let theme = if light_theme() { "light" } else { "dark" };

    rsx! {
        document::Title { "DocuForge" }
        style { {include_str!("../styles/app.css")} }
        div { class: "app", "data-theme": theme,
            AppHeader {
                light_theme: light_theme(),
                on_toggle_theme: move |_| light_theme.toggle(),
            }
            main { class: "workspace",
                h1 { "Workspace" }
                MergePanel {}
            }
        }
    }
}
