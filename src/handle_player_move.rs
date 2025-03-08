use crate::{
    PlayerMovement,
    ui::{LookTextBody, update::UpdateMainSectionText},
    zones::{Location, ZoneAsset, Zones},
};
use bevy::prelude::*;

/// moves players from zone to zone
pub fn handle_player_movement(
    mut player_move_events: EventReader<PlayerMovement>,
    zone_assets: Res<Assets<ZoneAsset>>,
    zones: Res<Zones>,
    mut location: ResMut<Location>,
    mut text_body: Query<&mut Text, With<LookTextBody>>,
) {
    let loc = location.0.clone();

    for ev in player_move_events.read() {
        if let Some(from) = zones.0.get(&loc) {
            // get where they're going based on direction
            // set players location to the new one
            let Some(Some(new_zone_asset_path)) = zone_assets
                .get(from)
                .map(|asset| asset.connections.get(&ev.0))
            else {
                // TODO: tell the player they cant go there
                continue;
            };
            if zones.0.get(new_zone_asset_path).is_some() {
                location.0 = new_zone_asset_path.to_owned();
                _ = text_body
                    .get_single_mut()
                    .map(|mut text| text.0 = String::new());
            } else {
                error!("{zones:?}.get({new_zone_asset_path})")
            }
        } else {
            error!("{zones:?}.get({loc})")
        }
    }
}

/// updated the main text display
pub fn set_main_body(
    zone_assets: Res<Assets<ZoneAsset>>,
    zones: Res<Zones>,
    location: Res<Location>,
    mut update_event: EventWriter<UpdateMainSectionText>,
) {
    zone_assets
        .get(zones.0.get(&location.0).unwrap())
        .map(|zone_asset| update_event.send(UpdateMainSectionText(zone_asset.description.clone())));
}
