use crate::PAGE;
use dioxus::core::{DynamicNode, IntoDynNode};
#[allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::{use_read, use_set};

pub fn Sidebar(cx: Scope) -> Element {
    let set_page = use_set(&cx, PAGE);

    let sidebar_items = vec![
        SidebarItemProps {
            name: "Home",
            icon: "home",
            body: rsx! {
                button {
                    alt: "Home",
                    onclick: move |_| {
                        set_page("home");
                        cx.needs_update();
                    }
                }
            }
            .into_vnode(&cx),
        },
        SidebarItemProps {
            name: "Our Mission",
            icon: "target",
            body: rsx! {
                button {
                    alt: "Mission",
                    onclick: move |_| {
                        set_page("mission");
                        cx.needs_update();
                    }
                }
            }
            .into_vnode(&cx),
        },
        SidebarItemProps {
            name: "About",
            icon: "info",
            body: rsx! {
                button {
                    alt: "About",
                    onclick: move |_| {
                        set_page("about");
                        cx.needs_update();
                    }
                }
            }
            .into_vnode(&cx),
        },
        SidebarItemProps {
            name: "Contact",
            icon: "mail",
            body: rsx! {
                button {
                    alt: "Contact",
                    onclick: move |_| {
                        set_page("contact");
                        cx.needs_update();
                    }
                }
            }
            .into_vnode(&cx),
        },
    ];

    let the_sidebar_items = &sidebar_items;
    cx.render(rsx! {
        div {
            id: "sidebar",
            link {
                href: "sidebar.css",
            },
            SidebarItems {
                items: the_sidebar_items,
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
        self.items
            .clone()
            .into_iter()
            .zip(other.items.clone().into_iter())
            .all(|(a, b)| a == b)
    }
}

pub fn SidebarItems(cx: Scope<SidebarItemsProps>) -> Element {
    let items = cx.props.items.clone();

    let return_items = items
        .into_iter()
        .map(|item| {
            rsx!(SidebarItem {
                name: item.name,
                icon: item.icon,
                body: item.body
            })
            .into_vnode(&cx)
        })
        .collect::<Vec<DynamicNode>>();

    let return_items = return_items
        .into_iter()
        .map(|item| -> VNode {

        })
        .collect::<Vec<VNode>>();
}

#[derive(Props)]
pub struct SidebarItemProps {
    pub name: &'static str,
    pub icon: &'static str,
    /// Usage:
    /// ```
    /// SidebarItemProps {
    ///     name: "Home",
    ///     icon: "home",
    ///     body: rsx!{
    ///         button {
    ///             alt: "Click me",
    ///             onclick: |_| page_set("home")
    ///         }
    ///     }
    /// }
    /// ```
    pub body: DynamicNode<'static>,
}

impl PartialEq for SidebarItemProps {
    /// WARNING - This does not compare the body!!!
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.icon == other.icon
    }
}

pub fn SidebarItem(cx: Scope<'static, SidebarItemProps>) -> Element {
    let class = format!(
        "sidebar-item sidebar-item-{}",
        cx.props.name.to_lowercase().replace(" ", "-")
    );

    cx.render(rsx! {
        div {
            class: "sidebar-item",
            div {
                class: "sidebar-item-icon",
                i {
                    class: "{class}",
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
