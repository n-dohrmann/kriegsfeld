
use bevy::math::{URect, UVec2};

pub struct TilemapSprite {
    pub name: &'static str,
    // coordinates of sprite within the tiles.png
    // file
    pub pixel_x: u32,
    pub pixel_y: u32,
}

pub struct TilemapDefinition {
    // sprite size is 16x16; this is more or less fixed
    pub tile_width: u32,
    pub tile_height: u32,
    // total tileset size, will vary during dev
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub sprites: &'static [TilemapSprite],
}

impl TilemapDefinition {
    pub const fn tile_size(&self) -> UVec2 {
        UVec2::new(self.tile_width, self.tile_height)
    }

    pub const fn atlas_size(&self) -> UVec2 {
        UVec2::new(self.atlas_width, self.atlas_height)
    }

    pub fn sprite_index(&self, name: &str) -> Option<usize> {
        self.sprites.iter().position(|sprite| sprite.name == name)
    }

    pub fn sprite_rect(&self, index: usize) -> URect {
        let sprite = &self.sprites[index];
        let min = UVec2::new(sprite.pixel_x, sprite.pixel_y);
        URect::from_corners(min, min + self.tile_size())
    }
}

// 16 x 16 tiles
const TILE_SZ: u32 = 16;
// 2 by 6 tilemap grid at the moment
const TILE_ROWS: u32 = 2;
const TILE_COLS: u32 = 6;

// TILEMAP DEFINITION
pub const TILEMAP: TilemapDefinition = TilemapDefinition {
    tile_width: TILE_SZ,
    tile_height: TILE_SZ,
    atlas_width: TILE_COLS * TILE_SZ,
    atlas_height: TILE_ROWS * TILE_SZ,
    sprites: todo!(),
};
