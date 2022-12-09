#![allow(non_snake_case)]

extern crate core;

mod components;

use dioxus::prelude::*;

pub fn main() {
    dioxus::web::launch(App);
}


pub type Page = &'static str;

pub static PAGE: Atom<Page> = |_| "home";

#[derive(Clone)]
pub struct ScopeProvider<'a, T>(Scope<'a, T>);

pub static SCOPE: Atom<Option<ScopeProvider<'static, ()>>> = |_| None;

pub static NODE_FACTORY: Atom<Option<&'static NodeFactory>> = |_| None;


#[allow(non_snake_case)]
fn App(cx: Scope<'static>) -> Element {
    let page: Page = use_read(&cx, PAGE);
    let set_page = use_set(&cx, PAGE);
    let set_scope = use_set(&cx, SCOPE);
    let set_node_factory = use_set(&cx, NODE_FACTORY);


    set_page("home");
    set_scope(Some(ScopeProvider(cx)));
    set_node_factory(Some(&NodeFactory::new(&cx)));


    cx.render(rsx!{
        Sidebar {

        }
    })
}