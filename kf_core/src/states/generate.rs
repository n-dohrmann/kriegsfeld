use crate::states::GameState;
use bevy::prelude::*;

pub fn check_exit(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
