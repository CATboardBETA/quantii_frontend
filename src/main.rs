#![allow(non_snake_case)]

mod components;

use dioxus::prelude::*;

use components::Sidebar;

pub fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        Sidebar {

        }
    })
}