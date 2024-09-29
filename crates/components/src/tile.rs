use dioxus::prelude::*;
use freya_elements::{
    elements as dioxus_elements,
    events::{
        KeyboardEvent,
        MouseEvent,
    },
};
use freya_hooks::{
    use_applied_theme,
    use_platform,
    TileTheme,
    TileThemeWith,
};
use winit::window::CursorIcon;

/// Indicates the current status of the Tile.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TileStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the tile.
    Hovering,
}

/// Tile component to be used with [`Radio`] and [`Checkbox`].
/// # Styling
/// Inherits the [`TileTheme`](freya_hooks::TileTheme) theme.
///
/// # Example
///
/// Tile is meant to be used with other components, take a look at [`Radio`](crate::Radio) for instance.
#[allow(non_snake_case)]
#[component]
pub fn Tile(
    /// Inner children for the Tile.
    children: Element,
    /// Optional element to be placed before the inner children of the Tile. Such as a [`Radio`](crate::Radio)
    leading: Option<Element>,
    /// Event handler for when the Tile is selected, e.g when clicking on it.
    onselect: Option<EventHandler<()>>,
    /// Theme override.
    theme: Option<TileThemeWith>,

    a11y_name: Option<String>,
) -> Element {
    let mut status = use_signal(TileStatus::default);
    let platform = use_platform();
    let TileTheme { padding } = use_applied_theme!(&theme, tile);

    use_drop(move || {
        if *status.read() == TileStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onkeydown = move |e: KeyboardEvent| {
        if e.key == Key::Enter {
            if let Some(onselect) = &onselect {
                e.stop_propagation();
                onselect.call(())
            }
        }
    };

    let onclick = move |e: MouseEvent| {
        if let Some(onselect) = &onselect {
            e.stop_propagation();
            onselect.call(())
        }
    };

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(TileStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(TileStatus::default());
    };

    rsx!(
        rect {
            onkeydown,
            onclick,
            onmouseenter,
            onmouseleave,
            direction: "horizontal",
            padding: "{padding}",
            cross_align: "center",
            if let Some(leading) = leading {
                rect {
                    padding: "4",
                    {leading}
                }
            }
            {children}
        }
    )
}
