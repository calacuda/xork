use crate::{ChangeScreen, ExitGame, commands::commands::SlashCmd};
use bevy::prelude::*;

pub fn slash_cmd(
    mut commands: EventReader<SlashCmd>,
    mut exit_ev: EventWriter<ExitGame>,
    mut view_ev: EventWriter<ChangeScreen>,
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
            SlashCmd::View { screen } => {
                view_ev.send(ChangeScreen {
                    to_screen: screen.clone(),
                });
            }
        }
    }
}
