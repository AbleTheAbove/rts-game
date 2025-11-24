use bevy::app::{App, Plugin, Startup};

pub struct Init;

impl Plugin for Init {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, run);
    }
}

pub fn run() {
    ensure_asset_dir();
}

pub struct Directories {
    assets: std::path::PathBuf,
    local:  std::path::PathBuf,
}

fn ensure_asset_dir() {
    let dirs = Directories {
        assets: assets_path().expect("Failed to get assets path"),
        local:  {
            let mut path = assets_path().expect("Failed to get assets path");
            path.push("local");
            path
        },
    };

    if !dirs.assets.exists() {
        std::fs::create_dir_all(&dirs.assets).expect("Failed to create assets directory");
    }

    if !dirs.local.exists() {
        std::fs::create_dir_all(&dirs.local).expect("Failed to create assets/local directory");
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
