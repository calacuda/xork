use crate::{ChangeScreen, ExitGame, commands::commands::SlashCmd};
use bevy::prelude::*;

pub fn slash_cmd(
    mut commands: MessageReader<SlashCmd>,
    mut exit_ev: MessageWriter<ExitGame>,
    mut view_ev: MessageWriter<ChangeScreen>,
    // mut player_look_ev: MessageWriter<PlayerLook>,
) {
    for command in commands.read() {
        match command {
            SlashCmd::Exit {} => {
                exit_ev.write_default();
            }
            SlashCmd::Help { with: _ } => {
                // TODO: get help
            }
            SlashCmd::Save { save_slot: _ } => {
                // TODO: save to save slot
            }
            SlashCmd::View { screen } => {
                view_ev.write(ChangeScreen {
                    to_screen: screen.clone(),
                });
            }
        }
    }
}
