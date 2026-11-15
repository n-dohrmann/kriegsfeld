use crate::map::assets::SpawnableAsset;
use bevy_procedural_tilemaps::prelude::*;

/// Utility wrapper that ensures model declarations and their asset bindings stay aligned.
pub struct TerrainModelBuilder {
    // NOTE: not using any layering for now, just
    // plain 2D grid
    pub models: ModelCollection<Cartesian2D>,
    // see NOTE on if we want to reintroduce multitile assets, not
    // allowed currently.
    pub assets: Vec<SpawnableAsset>,
}

impl Default for TerrainModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainModelBuilder {
    pub fn new() -> Self {
        Self {
            models: ModelCollection::new(),
            assets: Vec::new(),
        }
    }

    pub fn create_model<T>(&mut self, template: T, asset: SpawnableAsset) -> &mut Model<Cartesian2D>
    where
        T: Into<ModelTemplate<Cartesian2D>>,
    {
        let model_ref = self.models.create(template);
        self.assets.push(asset);
        model_ref
    }

    pub fn into_parts(self) -> (Vec<SpawnableAsset>, ModelCollection<Cartesian2D>) {
        (self.assets, self.models)
    }

    // lookup corresponding model by asset name. small model count so
    // linear search is fine...;
    pub fn model_on_name(&self, name: &str) -> Option<&Model<Cartesian2D>> {
        let idx = self.assets.iter().position(|a| a.sprite_name == name)?;
        self.models.models().nth(idx)
    }
}
