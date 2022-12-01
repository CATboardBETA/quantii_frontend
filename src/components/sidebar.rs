#[allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Sidebar(cx: Scope) -> Element {
    let page = use_state(&cx, || "home");

    cx.render(rsx!{
        div {
            id: "sidebar",
            link {
                href: "sidebar.css",
            },
            SidebarItems {
                name: "Home",
                icon: "home",
                onclick: move |_| page = "home",
            }
        }
    })
}

pub fn SidebarItems(cx: Scope<SidebarItemStruct>) -> Element {
    cx.render(rsx!{
        div {
            class: "sidebar-items",
            SidebarItem {
                name: "{cx.props.name}",
                icon: "{cx.props.icon}",
                button {
                    class: "sidebar-item-button",
                    onclick: move |_| (cx.props.onclick)(cx),
                    "{cx.props.name}"
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct SidebarItemsProps {
    pub items: Vec<SidebarItemStruct>,
}

#[derive(PartialEq, Props)]
pub struct SidebarItemStruct{
    pub name: &'static str,
    pub icon: &'static str,
    pub onclick: fn(&mut Scope<SidebarItemStruct>) -> (),
}

pub fn SidebarItem(cx: Scope<SidebarItemStruct>) -> Element {
    cx.render(rsx! {
        div {
            class: "sidebar-item",
            div {
                class: "sidebar-item-text",
                "{cx.props.name}"
            },
            div {
                class: "sidebar-item-icon",
                img {
                    src: "{cx.props.icon}",
                    alt: "{cx.props.name}",
                }
            }
        }
    })
}
