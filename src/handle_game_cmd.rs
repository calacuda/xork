use crate::{
    /* CommandResultMessage, GenerincFlavorText, */ PlayerLook, PlayerMovement, PlayerTake,
    commands::commands::GameCmd,
};
use bevy::prelude::*;

pub fn handle_game_cmd(
    mut commands: MessageReader<GameCmd>,
    // mut cmd_res_ev: MessageWriter<CommandResultEvent>,
    mut player_move_ev: MessageWriter<PlayerMovement>,
    mut player_look_ev: MessageWriter<PlayerLook>,
    mut player_take_ev: MessageWriter<PlayerTake>,
) {
    for command in commands.read() {
        match command {
            GameCmd::Go { direction } => {
                player_move_ev.write(PlayerMovement(direction.clone()));
            }
            GameCmd::Look => {
                player_look_ev.write_default();
            }
            // GameCmd::Take { thing } => {
            //     let thing = thing.join(" ");
            //     info!("player is picking up {thing}");
            //
            //     cmd_res_ev.write(CommandResultEvent(Err(GenerincFlavorText::Message(
            //             "you entered a valid command but picking-up/taking items isn't implemented yet.".into(),
            //         )),
            //     ));
            // }
            GameCmd::Take {} => {
                player_take_ev.write_default();
            } // GameCmd::Inventory {} => {
              //     warn!("list all inventory items")
              // }
        }
    }
}
