use crate::map::assets::SpawnableAsset;
use crate::map::models::TerrainModelBuilder;
use crate::map::sockets::*;
use bevy_procedural_tilemaps::prelude::*;

const WATER_WEIGHT: f32 = 30.;
const PLAINS_WEIGHT: f32 = 30.;
const TREES_WEIGHT: f32 = 25.;
const MOUNTAINS_WEIGHT: f32 = 22.;

fn build_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_socket: &TerrainSockets,
    socket_collection: &mut SocketCollection,
) {
    terrain_model_builder
        .create_model(
            SocketsCartesian2D::Simple {
                x_pos: terrain_socket.water,
                x_neg: terrain_socket.water,
                y_pos: terrain_socket.water,
                y_neg: terrain_socket.water,
            },
            SpawnableAsset::new("water"),
        )
        .with_weight(WATER_WEIGHT);

    terrain_model_builder
        .create_model(
            SocketsCartesian2D::Simple {
                x_pos: terrain_socket.plains,
                x_neg: terrain_socket.plains,
                y_pos: terrain_socket.plains,
                y_neg: terrain_socket.plains,
            },
            SpawnableAsset::new("plains"),
        )
        .with_weight(PLAINS_WEIGHT);

    terrain_model_builder
        .create_model(
            SocketsCartesian2D::Simple {
                x_pos: terrain_socket.trees,
                x_neg: terrain_socket.trees,
                y_pos: terrain_socket.trees,
                y_neg: terrain_socket.trees,
            },
            SpawnableAsset::new("trees"),
        )
        .with_weight(TREES_WEIGHT);

    terrain_model_builder
        .create_model(
            SocketsCartesian2D::Simple {
                x_pos: terrain_socket.mountains,
                x_neg: terrain_socket.mountains,
                y_pos: terrain_socket.mountains,
                y_neg: terrain_socket.mountains,
            },
            SpawnableAsset::new("mountains"),
        )
        .with_weight(MOUNTAINS_WEIGHT);

    socket_collection.add_connections(vec![
        (
            terrain_socket.water,
            vec![terrain_socket.water, terrain_socket.plains],
        ),
        (
            terrain_socket.plains,
            vec![terrain_socket.water, terrain_socket.plains],
        ),
        (
            terrain_socket.trees,
            vec![
                terrain_socket.plains,
                terrain_socket.trees,
                terrain_socket.mountains,
            ],
        ),
        (
            terrain_socket.mountains,
            vec![terrain_socket.trees, terrain_socket.mountains],
        ),
    ]);
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
    build_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection,
    );

    let (assets, models) = terrain_model_builder.into_parts();

    (assets, models, socket_collection)
}
