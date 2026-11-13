

use crate::map::assets::SpawnableAsset;
use crate::map::models::TerrainModelBuilder;
use crate::map::sockets::*;
use bevy_procedural_tilemaps::prelude::*;

fn build_dirt_layer(
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
        .with_weight(20.);

    socket_collection.add_connections(vec![(
        terrain_socket.0,
        vec![terrain_socket.0],
    )]);
}
