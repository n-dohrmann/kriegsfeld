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
}
