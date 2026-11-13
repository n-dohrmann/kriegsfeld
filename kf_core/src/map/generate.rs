use bevy::prelude::*;
use bevy_procedural_tilemaps::prelude::*;

use crate::map::{
    assets::{load_assets, prepare_tilemap_handles},
    rules::build_world,
};

// world map size. change these as needed, not sure what looks/plays best
pub const GRID_X: u32 = 25;
pub const GRID_Y: u32 = 18;

const ASSETS_PATH: &str = "assets";
const TILEMAP_FILE: &str = "tiles.png";

pub const TILE_SIZE: f32 = 16.;

// not sure if we want to scale this up or not.
// how large assets get rendered compared to their source
const ASSETS_MODIFIER: f32 = 3.0;
const ASSETS_SCALE: Vec3 = Vec3::new(ASSETS_MODIFIER, ASSETS_MODIFIER, 1.0);
const NODE_SIZE: Vec3 = Vec3::new(ASSETS_MODIFIER * TILE_SIZE, ASSETS_MODIFIER * TILE_SIZE, 1.);

pub fn map_pixel_dimensions() -> Vec2 {
    Vec2::new(ASSETS_MODIFIER * TILE_SIZE * GRID_X as f32, ASSETS_MODIFIER * TILE_SIZE * GRID_Y as f32)
}

pub fn setup_generator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 1. Rules Initialization - Get tile definitions and connection rules
    let (assets_definitions, models, socket_collection) = build_world();

    let rules = RulesBuilder::new_cartesian_2d(models, socket_collection)
        .build()
        .unwrap();

    // 2. Grid - Create 3D world space with wrapping behavior (false, false, false)
    let grid = CartesianGrid::new_cartesian_2d(GRID_X, GRID_Y, false, false);

    // 3. Configuring the Algorithm - Set up WFC behavior
    let gen_builder = GeneratorBuilder::new()
        .with_rules(rules)
        .with_grid(grid.clone())
        .with_rng(RngMode::RandomSeed)
        .with_node_heuristic(NodeSelectionHeuristic::MinimumRemainingValue)
        .with_model_heuristic(ModelSelectionHeuristic::WeightedProbability);

    let generator = gen_builder.build().unwrap();

    // 4. Loading Assets - Load sprite atlas and convert to renderable assets
    let tilemap_handles =
        prepare_tilemap_handles(&asset_server, &mut atlas_layouts, ASSETS_PATH, TILEMAP_FILE);
    let models_assets = load_assets(&tilemap_handles, assets_definitions);

    // 5. Spawning the Generator - Create entity with Transform and NodesSpawner
    commands.spawn((
        Transform::from_translation(Vec3 {
            x: -TILE_SIZE * grid.size_x() as f32 * ASSETS_MODIFIER / 2.,
            y: -TILE_SIZE * grid.size_y() as f32 * ASSETS_MODIFIER / 2.,
            z: 0.,
        }),
        grid,
        generator,
        NodesSpawner::new(models_assets, NODE_SIZE, ASSETS_SCALE).with_z_offset_from_y(true),
    ));
}
