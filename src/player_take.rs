use crate::{
    PlayerTake,
    items::InventoryEntry,
    zones::{Location, ZoneAsset, Zones},
};
use bevy::prelude::*;

/// displays more information to the player.
pub fn handle_player_take(
    mut cmds: Commands,
    mut player_take_evs: EventReader<PlayerTake>,
    zone_assets: Res<Assets<ZoneAsset>>,
    zones: Res<Zones>,
    location: Res<Location>,
) {
    let loc = location.0.clone();

    for _ev in player_take_evs.read() {
        if let Some(at) = zones.0.get(&loc) {
            // get where they are and the "examine" text in one step.
            let Some(items) = zone_assets.get(at).map(|asset| asset.items.clone()) else {
                continue;
            };

            if items.len() > 0 {
                for item in items {
                    info!("got: {item}");
                    cmds.spawn(InventoryEntry { asset_path: item });
                }
            } else {
                // TODO: issue "no items to take, command failed" notification.
            }

            info!("player acquired all items from zone: {loc:?}");
        } else {
            error!(
                "the player is at a location that is unknown to the engine. something went VERY wrong."
            );
            error!("{zones:?}.get({loc})")
        }
    }
}
