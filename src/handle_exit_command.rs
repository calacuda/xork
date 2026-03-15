use crate::{ExitGame, state::MainState};
use bevy::prelude::*;

pub fn slash_exit(
    mut exit_evs: MessageReader<ExitGame>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    for _ev in exit_evs.read() {
        next_state.set(MainState::Wrapup);
    }
}
