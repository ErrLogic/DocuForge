use dioxus::prelude::*;

#[component]
pub fn AppHeader(light_theme: bool, on_toggle_theme: EventHandler<MouseEvent>) -> Element {
    rsx! {
        header { class: "app-header",
            span { class: "brand", "DocuForge" }
            button {
                r#type: "button",
                aria_pressed: if light_theme { "true" } else { "false" },
                onclick: move |event| on_toggle_theme.call(event),
                "Light theme"
            }
        }
    }
}
