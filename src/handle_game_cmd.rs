use crate::{
    CommandResultEvent, GenerincFlavorText, PlayerLook, PlayerMovement, commands::commands::GameCmd,
};
use bevy::prelude::*;

pub fn handle_game_cmd(
    mut commands: EventReader<GameCmd>,
    mut cmd_res_ev: EventWriter<CommandResultEvent>,
    mut player_move_ev: EventWriter<PlayerMovement>,
    mut player_look_ev: EventWriter<PlayerLook>,
) {
    for command in commands.read() {
        match command {
            GameCmd::Go { direction } => {
                player_move_ev.send(PlayerMovement(direction.clone()));
            }
            GameCmd::Look => {
                // info!("player took a closer look at the the zone");

                player_look_ev.send_default();
            }
            GameCmd::Take { thing } => {
                let thing = thing.join(" ");
                info!("player is picking up {thing}");

                cmd_res_ev.send(CommandResultEvent(Err(GenerincFlavorText::Message(
                        "you entered a valid command but picking-up/taking items isn't implemented yet.".into(),
                    )),
                ));
            }
        }
    }
}
