#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::WindowBuilder;
use dioxus_router::{Route, Router};
use highvis::components::*;

fn main() {
    dioxus_desktop::launch_cfg(App, 
        dioxus_desktop::Config::new().with_window(
            WindowBuilder::new()
                .with_title("highvis")
                .with_resizable(true)
                .with_maximizable(true)
                .with_inner_size(
                    dioxus_desktop::wry::application::dpi::LogicalSize::new(1000, 400),
                )
        )
    );
}

fn App(cx: Scope) -> Element {
    cx.render(rsx!{
        style { include_str!("../src/styles/styles.css") }
        main {
            div {
                class: "container",
                Router {
                    Route { to: "/", StartPage {} },
                }
            }
        }
    })
}
