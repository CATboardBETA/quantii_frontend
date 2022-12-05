#[allow(non_snake_case)]

use dioxus::prelude::*;


pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            id: "sidebar",
            link {
                href: "sidebar.css",
            },
            SidebarItems {
                name: "Home",
                icon: "home",
                onclick_to: "home",
            },
        }
    })
}

#[derive(Props)]
pub struct SidebarItemsProps {
    pub items: &'static Vec<SidebarItemProps>,
}

pub fn SidebarItems(cx: Scope<SidebarItemsStruct>) -> Element {
    cx.render(rsx!{
        div {
            class: "sidebar-items",
            SidebarItem {
                name: "{cx.props.name}",
                icon: "{cx.props.icon}",
                button {
                    class: "sidebar-item-button",
                    onclick: move |_| crate::PAGE.lock().unwrap().clone_from(&cx.props.onclick_to),
                    "{cx.props.name}"
                }
            }
        }
    })
}

#[derive(Props)]
pub struct SidebarItemProps {
    pub name: &'static str,
    pub icon: &'static str,
    pub onclick_to: &'static str,
    #[props(optional)]
    pub body: Element<'static>,
}

pub fn SidebarItem(cx: Scope<SidebarItemProps>) -> Element {
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
