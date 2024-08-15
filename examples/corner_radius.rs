#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_title("Performance Overlay Plugin")
            .with_size(700., 500.)
            .with_plugin(PerformanceOverlayPlugin::default()),
    )
}
fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "50%",
            width: "auto",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            rect {
                height: "100%",
                width: "calc(70% + {count})",
            }
        }
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            onclick: move |_| count -= 5,
        }
    )
}
