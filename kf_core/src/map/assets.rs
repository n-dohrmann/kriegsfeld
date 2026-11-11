use crate::map::tilemap::TILEMAP;
use bevy::{prelude::*, sprite::Anchor};
use bevy_procedural_tilemaps::prelude::*;

#[derive(Clone)]
pub struct SpawnableAsset {
    sprite_name: &'static str,
    // iff multi-tile assets are needed, add a grid
    // offset here. the current plan is for single
    // tile units only.
    //
    // likewise, if intra-tile positioning is needed
    // (again, current plan is no), add an additional
    // Vec3 offset for within the tile.
    components_spawner: fn(&mut EntityCommands),
}

impl SpawnableAsset {
    pub fn new(sprite_name: &'static str) -> Self {
        Self {
            sprite_name,
            components_spawner: |_| {},
        }
    }

    pub fn with_cmp_spanwer(mut self, components_spawner: fn(&mut EntityCommands)) -> Self {
        self.components_spawner = components_spawner;
        self
    }
}

#[derive(Clone)]
pub struct TilemapHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl TilemapHandles {
    pub fn sprite(&self, atlas_index: usize) -> Sprite {
        Sprite::from_atlas_image(
            self.image.clone(),
            TextureAtlas::from(self.layout.clone()).with_index(atlas_index),
        )
    }
}

pub fn prepare_tilemap_handles(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    assets_directory: &str,
    tilemap_file: &str,
) -> TilemapHandles {
    let image = asset_server.load::<Image>(format!("{assets_directory}/{tilemap_file}"));
    let mut layout = TextureAtlasLayout::new_empty(TILEMAP.atlas_size());
    for index in 0..TILEMAP.sprites.len() {
        layout.add_texture(TILEMAP.sprite_rect(index));
    }
    let layout = atlas_layouts.add(layout);

    TilemapHandles { image, layout }
}

pub fn load_assets(
    tilemap_handles: &TilemapHandles,
    // NOTE: if we decide to allow multi-tile units then
    // this needs to be a vec<vec<T>> (along with a cascade
    // of other changes for the terrain model builder)
    asset_definitions: Vec<SpawnableAsset>,
) -> ModelsAssets<Sprite> {
    let mut models_assets = ModelsAssets::<Sprite>::new();
    for (model_index, asset) in asset_definitions.into_iter().enumerate() {
        let SpawnableAsset {
            sprite_name,
            components_spawner,
        } = asset;

        let Some(atlas_index) = TILEMAP.sprite_index(sprite_name) else {
            panic!("Unknown atlas sprite '{}'", sprite_name);
        };

        models_assets.add(
            model_index,
            ModelAsset {
                assets_bundle: tilemap_handles.sprite(atlas_index),
                // no grid offset by definition for single tile units
                grid_offset: GridDelta::default(),
                // no offset within the grid tile itself
                world_offset: Vec3::ZERO,
                spawn_commands: components_spawner,
            },
        )
    }
    models_assets
}
