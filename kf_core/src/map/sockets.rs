use bevy::curve::derivatives;
use bevy_procedural_tilemaps::prelude::*;

#[derive(Debug)]
// define grid connectivity between terrain tiles
pub struct TerrainSocket(pub Socket);

pub fn create_socket(socket_collection: &mut SocketCollection) -> TerrainSocket {
    TerrainSocket(socket_collection.create())
}
