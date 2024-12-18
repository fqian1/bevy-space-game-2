use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    InGame,
    Paused,
    GameOver,
    MainMenu,
    #[default]
    Loading,
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            GameState::GameOver => next_state.set(GameState::MainMenu), // TODO: Create UI for this and remove this
            _ => (),
        }
        info!("change state");
    }
}

fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_state(GameState::InGame)
            .add_systems(
                Update,
                start_game.run_if(in_state(GameState::GameOver).or(in_state(GameState::MainMenu))),
            )
            .add_systems(
                Update,
                game_state_input_events.run_if(in_state(GameState::InGame)),
            );
    }
}
