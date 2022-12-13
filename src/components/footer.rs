#[allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            id: "footer",
            link {
                href: "footer.css",
            }
        }
    })
}
