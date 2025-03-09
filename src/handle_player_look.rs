use crate::{
    PlayerLook,
    ui::update::UpdateLookSectionText,
    zones::{Location, ZoneAsset, Zones},
};
use bevy::prelude::*;

/// displays more information to the player.
pub fn handle_player_look(
    mut player_move_events: EventReader<PlayerLook>,
    mut look_event: EventWriter<UpdateLookSectionText>,
    zone_assets: Res<Assets<ZoneAsset>>,
    zones: Res<Zones>,
    location: Res<Location>,
) {
    let loc = location.0.clone();

    for _ev in player_move_events.read() {
        if let Some(at) = zones.0.get(&loc) {
            // get where they are and the "examine" text in one step.
            let Some(look_text) = zone_assets.get(at).map(|asset| asset.examine.clone()) else {
                continue;
            };

            let look_text =
                look_text.unwrap_or("You looked around and saw nothing else of interest...".into());
            look_event.send(UpdateLookSectionText(look_text));
            info!("player took a closer look at zone {loc:?}");
        } else {
            error!(
                "the player is at a location that is unknown to the engine. something went VERY wrong."
            );
            error!("{zones:?}.get({loc})")
        }
    }
}
