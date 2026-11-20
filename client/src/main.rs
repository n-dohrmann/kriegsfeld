// placeholder - not sure where main component defs should go
// at the moment

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};

use kf_core::bevy_procedural_tilemaps::prelude::*;
use kf_core::map::{
    GridPrinter,
    generate::{map_pixel_dimensions, setup_generator},
};

#[derive(Resource, Default)]
pub struct RunOnce(pub bool);

impl RunOnce {
    pub fn run_checked<F: FnOnce()>(&mut self, f: F) {
        if self.0 {
            return;
        }
        self.0 = true;
        f();
    }
}

fn main() {
    let map_size = map_pixel_dimensions();

    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(RunOnce::default())
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
        .add_systems(Startup, (setup_camera, setup_generator))
        .add_systems(Update, print_grid_info)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn print_grid_info(
    mut run: ResMut<RunOnce>,
    grid_info: Query<&Generator<Cartesian2D, CartesianGrid<Cartesian2D>>>,
) {
    run.run_checked(|| {
        if let Ok(info) = grid_info.single() {
            println!("Found grid:\n{}", info.print());
        }
    });
}
