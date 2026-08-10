//! Environment-specific (Windows, MacOS, Linux) [`WindowPlugin`] configurations.
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowPlugin, WindowResolution};
use std::env;

/// Sets the [`WindowPlugin`] depending on the set environment.
///
/// # Examples
///
/// ```
/// use std::env;
/// use bevy::window::PresentMode;
/// use home_invasion::cfg::window_plugin::build_platform_window_plugin;
///
/// let window_plugin = build_platform_window_plugin();
/// let window = window_plugin.primary_window.as_ref().unwrap();
///
/// if cfg!(target_os = "linux") {
///     let is_wayland = env::var("WAYLAND_DISPLAY").is_ok()
///         || env::var("XDG_SESSION_TYPE").is_ok_and(|s| s == "wayland");
///
///     let expected_present_mode = if is_wayland {
///         PresentMode::AutoNoVsync
///     } else {
///         PresentMode::Fifo
///     };
///
///     assert_eq!(window.present_mode, expected_present_mode);
/// }
/// ````
pub fn build_platform_window_plugin() -> WindowPlugin {
    let mut primary_window = Window {
        title: "Home Invasion".into(),
        resolution: WindowResolution::new(1280, 720),
        ..default()
    };

    if cfg!(target_os = "windows") {
        primary_window.present_mode = PresentMode::AutoVsync;
    } else if cfg!(target_os = "macos") {
        primary_window.resolution.set(1920.0, 1080.0);
    } else if cfg!(target_os = "linux") {
        let is_wayland = env::var("WAYLAND_DISPLAY").is_ok()
            || env::var("XDG_SESSION_TYPE").is_ok_and(|s| s == "wayland");

        if is_wayland {
            primary_window.present_mode = PresentMode::AutoNoVsync;
            primary_window.mode = WindowMode::Windowed;
        } else {
            primary_window.present_mode = PresentMode::Fifo;
        }
    }

    WindowPlugin {
        primary_window: Some(primary_window),
        ..default()
    }
}
