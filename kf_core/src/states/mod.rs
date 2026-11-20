pub mod generate;
pub mod main_menu;

use crate::map::generate::{RunOnce, despawn_generator, print_grid_info, setup_generator};
use crate::states::generate::check_exit;
use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Generate,
}

// expose state information as a plugin.
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        use main_menu::{despawn_main_menu, spawn_main_menu};
        app.init_state::<GameState>()
            .insert_resource(RunOnce::default())
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                OnEnter(GameState::Generate),
                (setup_generator, print_grid_info).chain(),
            )
            .add_systems(Update, check_exit.run_if(in_state(GameState::Generate)))
            .add_systems(OnExit(GameState::Generate), despawn_generator);
    }
}
