#[allow(non_snake_case)]

use dioxus::prelude::*;
use crate::PAGE;


pub fn Sidebar(cx: Scope) -> Element {
    let sidebar_items = vec![
        SidebarItemProps {
            name: "Home",
            icon: "home",
            onclick_to: "Home",
            body: rsx!{
                button {
                    alt: "Home",
                    onclick: move |_| {
                        *PAGE.lock().unwrap() = "home"
                        cx.needs_update();
                    }
                }
            }
        },
        SidebarItemProps {
            name: "Our Mission",
            icon: "target",
            onclick_to: "Mission",
            body: rsx!{
                button {
                    alt: "Mission",
                    onclick: move |_| {
                        *PAGE.lock().unwrap() = "mission"
                        cx.needs_update();
                    }
                }
            }
        },
        SidebarItemProps {
            name: "About",
            icon: "info",
            onclick_to: "About",
            body: rsx!{
                button {
                    alt: "About",
                    onclick: move |_| {
                        *PAGE.lock().unwrap() = "about"
                        cx.needs_update();
                    }
                }
            }
        },
        SidebarItemProps {
            name: "Contact",
            icon: "mail",
            onclick_to: "Contact",
            body: rsx!{
                button {
                    alt: "Contact",
                    onclick: move |_| {
                        *PAGE.lock().unwrap() = "contact"
                        cx.needs_update();
                    }
                }
            }
        }
    ];
    cx.render(rsx!{
        div {
            id: "sidebar",
            link {
                href: "sidebar.css",
            },
            SidebarItems {
                items: &sidebar_items
            }
        }
    })
}

#[derive(Props)]
pub struct SidebarItemsProps {
    pub items: &'static Vec<SidebarItemProps>,
}

impl PartialEq for SidebarItemsProps {
    fn eq(&self, other: &Self) -> bool {
        if self.items.len() != other.items.len() {
            return false;
        }
        self.items.clone().into_iter().zip(other.items.clone().into_iter()).all(|(a, b)| a == b)
    }
}

pub fn SidebarItems(cx: Scope<SidebarItemsProps>) -> Element {
    cx.render(rsx!{
        div {
            class: "sidebar-items",
            [for item in cx.props.items {
                SidebarItem {
                    name: item.name,
                    icon: item.icon,
                    onclick_to: item.onclick_to,
                    body: item.body
                }
            }]
        }
    })
}

#[derive(Props)]
pub struct SidebarItemProps {
    pub name: &'static str,
    pub icon: &'static str,
    pub onclick_to: &'static str,
    /// Usage:
    /// ```
    /// SidebarItemProps {
    ///     name: "Home",
    ///     icon: "home",
    ///     onclick_to: "Home",
    ///     body: rsx!{
    ///         button {
    ///             alt: "Click me",
    ///             onclick: |_| PAGE.lock().unwrap().replace("Home")
    ///         }
    ///     }
    /// }
    /// ```
    pub body: LazyNodes<'static, 'static>
}

impl PartialEq for SidebarItemProps {
    /// WARNING - This does not compare the body!!!
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.icon == other.icon && self.onclick_to == other.onclick_to
    }
}

pub fn SidebarItem(cx: Scope<SidebarItemProps>) -> Element {
    cx.render(rsx!{
        div {
            class: "sidebar-item",
            div {
                class: "sidebar-item-icon",
                i {
                    class: "{[format!("sidebar-item sidebar-item-{}", cx.props.icon)]"}
                }
            },
            div {
                class: "sidebar-item-name",
                "{cx.props.name}"
            },
            cx.props.body
        }
    })
}