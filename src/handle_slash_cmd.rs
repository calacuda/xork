use crate::{ExitGame, commands::commands::SlashCmd};
use bevy::prelude::*;

pub fn slash_cmd(
    mut commands: EventReader<SlashCmd>,
    mut exit_ev: EventWriter<ExitGame>,
    // mut player_look_ev: EventWriter<PlayerLook>,
) {
    for command in commands.read() {
        match command {
            SlashCmd::Exit {} => {
                exit_ev.send_default();
            }
            SlashCmd::Help { with: _ } => {
                // TODO: get help
            }
            SlashCmd::Save { save_slot: _ } => {
                // TODO: save to save slot
            }
        }
    }
}
