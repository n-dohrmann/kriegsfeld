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
