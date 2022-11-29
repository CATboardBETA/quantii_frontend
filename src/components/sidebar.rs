#[allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx!{
        div: {
            id: "sidebar",
            link: {
                href: "sidebar.css",
            },
            SidebarItems {

            }
        }
    })
}

pub fn SidebarItems(cx: Scope) -> Element {
    cx.render(rsx!{
        div: {
            class: "sidebar-items",
            SidebarItem {
                name: item.name,
                icon: item.icon,
                button: item.button,
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct SidebarItemsProps {
    pub items: Vec<SidebarItemStruct<'static>>,
}

pub struct SidebarItemStruct<'elem> {
    pub name: String,
    pub icon: String,
    pub button: Element<'elem>,
}

pub fn SidebarItem(cx: Scope) -> Element {
    cx.render(rsx! {
        div: {
            class: "sidebar-item",
            div: {
                class: "sidebar-item-text",
                "{cx.props.name}"
            },
            div: {
                class: "sidebar-item-icon",
                img: {
                    src: "{cx.props.icon}",
                    alt: "{cx.props.name}",
                }
            }
        }
    })
}
