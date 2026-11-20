pub mod assets;
pub mod generate;
pub mod models;
pub mod rules;
pub mod sockets;
pub mod tilemap;

use bevy_procedural_tilemaps::proc_gen::generator::Generator;
use bevy_procedural_tilemaps::proc_gen::grid::{Cartesian2D, CartesianGrid};

pub trait GridPrinter {
    fn print(&self) -> String;
}

impl GridPrinter for Generator<Cartesian2D, CartesianGrid<Cartesian2D>> {
    fn print(&self) -> String {
        // not sure if "grid" is what actually got generated...
        // this is just the geometric data, not the actual node data,
        // only has the size and looping constraints we passed in before
        // calling generate
        let grid = self.grid().to_string();

        // think we want the "GenInfo" struct from this somehow
        grid
    }
}
