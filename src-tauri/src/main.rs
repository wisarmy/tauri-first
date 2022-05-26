#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{AboutMetadata, Menu, MenuEntry, MenuItem, Submenu};

fn main() {
    let ctx = tauri::generate_context!();
    let menu = Menu::with_items([Submenu::new(
        &ctx.package_info().name,
        Menu::with_items([MenuItem::About(
            ctx.package_info().name.clone(),
            AboutMetadata::default(),
        )
        .into()]),
    )
    .into()]);
    tauri::Builder::default()
        .menu(menu)
        .run(ctx)
        .expect("error while running tauri application");
}
