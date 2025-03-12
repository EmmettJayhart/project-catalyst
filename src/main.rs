#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use project_catalyst::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
