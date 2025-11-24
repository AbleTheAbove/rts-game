/// A module for debuging and similar stuff.
use bevy::{
    prelude::*,
    render::view::screenshot::{Capturing, Screenshot, save_to_disk},
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};

pub struct DbgPlugin;
impl Plugin for DbgPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (screenshot_on_spacebar, screenshot_saving));
    }
}

pub fn screenshot_on_spacebar(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    use chrono::prelude::*;

    let local: DateTime<Local> = Local::now();

    let mut path = assets_path().expect("Failed to get assets path");
    path.push("local");

    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create assets/local directory");
    }

    let time_fmt = format!(
        "{}/screenshot-{}-{}-{}-{}-{}-{}-{}.png",
        path.to_str().unwrap(),
        local.year(),
        local.month(),
        local.day(),
        local.hour(),
        local.minute(),
        local.second(),
        local.nanosecond()
    );

    if input.just_pressed(KeyCode::Space) {
        let path = time_fmt;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

fn assets_path() -> Result<std::path::PathBuf, String> {
    let mut path = current_path()?;
    path.push("assets");
    Ok(path)
}

#[allow(dead_code)]
#[cfg(not(debug_assertions))]
fn current_path() -> Result<std::path::PathBuf, String> {
    std::env::current_exe().map_err(|e| e.to_string())
}

#[allow(dead_code)]
#[cfg(debug_assertions)]
fn current_path() -> Result<std::path::PathBuf, String> {
    std::env::current_dir().map_err(|e| e.to_string())
}

pub fn screenshot_saving(
    mut commands: Commands,
    screenshot_saving: Query<Entity, With<Capturing>>,
    window: Single<Entity, With<Window>>,
) {
    match screenshot_saving.iter().count() {
        0 => {
            commands.entity(*window).remove::<CursorIcon>();
        }
        x if x > 0 => {
            commands
                .entity(*window)
                .insert(CursorIcon::from(SystemCursorIcon::Progress));
        }
        _ => {}
    }
}
