#![allow(non_snake_case)]

extern crate core;

mod components;

use dioxus::prelude::*;
use fermi::{use_read, use_set, Atom};

use components::Sidebar;

pub fn main() {
    dioxus::web::launch_static(App);
}

pub type Page = &'static str;

pub static PAGE: Atom<Page> = |_| "home";

#[derive(Clone)]
pub struct ScopeProvider<'a, T>(Scope<'a, T>);

pub static SCOPE: Atom<Option<ScopeProvider<'static, ()>>> = |_| None;

#[allow(non_snake_case)]
fn App(cx: Scope<'static>) -> Element {
    let page: Page = use_read(&cx, PAGE);
    let set_page = use_set(&cx, PAGE);
    let set_scope = use_set(&cx, SCOPE);

    set_page("home");
    set_scope(Some(ScopeProvider(cx)));

    cx.render(rsx! {
        Sidebar {

        }
    })
}
