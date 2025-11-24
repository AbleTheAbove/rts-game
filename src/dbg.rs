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

    let time_fmt = format!(
        ".assets/local/screenshot-{}-{}-{}-{}-{}-{}-{}.png",
        // "{}/screenshot-{}-{}-{}-{}-{}-{}-{}.png",
        // path.to_str().unwrap(),
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
