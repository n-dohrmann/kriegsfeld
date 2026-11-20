// placeholder - not sure where main component defs should go
// at the moment

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};

use kf_core::map::generate::{map_pixel_dimensions, print_grid_info};
use kf_core::{bevy_procedural_tilemaps::prelude::*, states::StatePlugin};

fn main() {
    let map_size = map_pixel_dimensions();

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "../".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(map_size.x as u32, map_size.y as u32),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // not sure if we should keep this as 2d or revert to 3d
        .add_plugins(ProcGenSimplePlugin::<Cartesian2D, Sprite>::default())
        .add_plugins(StatePlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, print_grid_info)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
