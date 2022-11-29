#![allow(non_snake_case)]

mod components;

use dioxus::prelude::*;

pub fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        Sidebar {

        }
    })
}