use crate::map::assets::SpawnableAsset;
use crate::map::models::TerrainModelBuilder;
use crate::map::sockets::*;
use bevy_procedural_tilemaps::prelude::*;

const WATER_WEIGHT: f32 = 20.;

fn build_water_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_socket: &TerrainSocket,
    socket_collection: &mut SocketCollection,
) {
    terrain_model_builder
        .create_model(
            SocketsCartesian2D::Simple {
                x_pos: terrain_socket.0,
                x_neg: terrain_socket.0,
                y_pos: terrain_socket.0,
                y_neg: terrain_socket.0,
            },
            SpawnableAsset::new("water"),
        )
        .with_weight(WATER_WEIGHT);

    socket_collection.add_connections(vec![(terrain_socket.0, vec![terrain_socket.0])]);
}

pub fn build_world() -> (
    Vec<SpawnableAsset>,
    ModelCollection<Cartesian2D>,
    SocketCollection,
) {
    let mut socket_collection = SocketCollection::new();
    let terrain_sockets = create_socket(&mut socket_collection);

    let mut terrain_model_builder = TerrainModelBuilder::new();

    // Build water layer
    build_water_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection,
    );

    let (assets, models) = terrain_model_builder.into_parts();

    (assets, models, socket_collection)
}
