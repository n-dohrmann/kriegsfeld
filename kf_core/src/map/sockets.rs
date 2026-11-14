use bevy::curve::derivatives;
use bevy_procedural_tilemaps::prelude::*;

#[derive(Debug)]
// define grid connectivity between terrain tiles
pub struct TerrainSockets {
    pub water: Socket,
    pub plains: Socket,
    pub trees: Socket,
    pub mountains: Socket,
}

pub fn create_socket(socket_collection: &mut SocketCollection) -> TerrainSockets {
    // TerrainSockets(socket_collection.create())
    let mut new_socket = || -> Socket { socket_collection.create() };

    TerrainSockets {
        water: new_socket(),
        plains: new_socket(),
        trees: new_socket(),
        mountains: new_socket(),
    }
}
