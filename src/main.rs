#![allow(non_snake_case)]

extern crate core;

mod components;

use dioxus::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

use components::Sidebar;

pub fn main() {
    dioxus::web::launch(app);
}


pub type Page = &'static str;

lazy_static! {
    /// The current page
    pub static ref PAGE: Mutex<Page> = Mutex::new("home");
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        Sidebar {

        }
    })
}