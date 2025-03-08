use crate::{
    NewZone, PlayerMovement,
    commands::commands::Direction,
    state::GameState,
    ui::{
        CompassDownText, CompassEastText, CompassNorthEastText, CompassNorthText,
        CompassNorthWestText, CompassSouthEastText, CompassSouthText, CompassSouthWestText,
        CompassUpText, CompassWestText, LookTextBody, update::UpdateMainSectionText,
    },
    zones::{Location, ZoneAsset, Zones},
};
use bevy::{
    color::palettes::tailwind::{AMBER_500, GRAY_500},
    prelude::*,
};

/// moves players from zone to zone
pub fn handle_player_movement(
    mut player_move_events: EventReader<PlayerMovement>,
    mut new_zone_ev: EventWriter<NewZone>,
    zone_assets: Res<Assets<ZoneAsset>>,
    zones: Res<Zones>,
    mut location: ResMut<Location>,
    mut look_text: Query<&mut Text, With<LookTextBody>>,
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
                _ = look_text
                    .get_single_mut()
                    .map(|mut text| text.0 = String::new());
                debug!("player moved {:?}", ev.0);
                new_zone_ev.send_default();
            } else {
                error!("player tried to move {:?}, but failed.", ev.0);
                debug!("{zones:?}.get({new_zone_asset_path})")
            }
        } else {
            error!(
                "the player is at a location that is unknown to the engine. something went VERY wrong."
            );
            error!("{zones:?}.get({loc})")
        }
    }
}

pub fn send_new_zone(
    mut new_zone_ev: EventWriter<NewZone>,
    zone_assets: Res<Assets<ZoneAsset>>,
    zones: Res<Zones>,
    location: Res<Location>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if zone_assets.get(zones.0.get(&location.0).unwrap()).is_some() {
        new_zone_ev.send_default();
        next_state.set(GameState::Adventure);
    }
}

/// updated the main text display
pub fn set_main_body(
    mut new_zone_evs: EventReader<NewZone>,
    zone_assets: Res<Assets<ZoneAsset>>,
    zones: Res<Zones>,
    location: Res<Location>,
    mut update_event: EventWriter<UpdateMainSectionText>,
) {
    for _ev in new_zone_evs.read() {
        zone_assets
            .get(zones.0.get(&location.0).unwrap())
            .map(|zone_asset| {
                update_event.send(UpdateMainSectionText(zone_asset.description.clone()));
            });
    }
}

/// moves players from zone to zone
pub fn compass_update(
    mut new_zone_evs: EventReader<NewZone>,
    zone_assets: Res<Assets<ZoneAsset>>,
    zones: Res<Zones>,
    location: Res<Location>,
    // mut text_body: Query<&mut Text, With<LookTextBody>>,
    mut c_u: Query<
        &mut TextColor,
        (
            With<CompassUpText>,
            // Without<CompassUpText>,
            Without<CompassDownText>,
            Without<CompassNorthText>,
            Without<CompassSouthText>,
            Without<CompassEastText>,
            Without<CompassWestText>,
            Without<CompassNorthEastText>,
            Without<CompassNorthWestText>,
            Without<CompassSouthEastText>,
            Without<CompassSouthWestText>,
        ),
    >,
    mut c_d: Query<
        &mut TextColor,
        (
            With<CompassDownText>,
            Without<CompassUpText>,
            // Without<CompassDownText>,
            Without<CompassNorthText>,
            Without<CompassSouthText>,
            Without<CompassEastText>,
            Without<CompassWestText>,
            Without<CompassNorthEastText>,
            Without<CompassNorthWestText>,
            Without<CompassSouthEastText>,
            Without<CompassSouthWestText>,
        ),
    >,
    mut c_n: Query<
        &mut TextColor,
        (
            With<CompassNorthText>,
            Without<CompassUpText>,
            Without<CompassDownText>,
            // Without<CompassNorthText>,
            Without<CompassSouthText>,
            Without<CompassEastText>,
            Without<CompassWestText>,
            Without<CompassNorthEastText>,
            Without<CompassNorthWestText>,
            Without<CompassSouthEastText>,
            Without<CompassSouthWestText>,
        ),
    >,
    mut c_s: Query<
        &mut TextColor,
        (
            With<CompassSouthText>,
            Without<CompassUpText>,
            Without<CompassDownText>,
            Without<CompassNorthText>,
            // Without<CompassSouthText>,
            Without<CompassEastText>,
            Without<CompassWestText>,
            Without<CompassNorthEastText>,
            Without<CompassNorthWestText>,
            Without<CompassSouthEastText>,
            Without<CompassSouthWestText>,
        ),
    >,
    mut c_e: Query<
        &mut TextColor,
        (
            With<CompassEastText>,
            Without<CompassUpText>,
            Without<CompassDownText>,
            Without<CompassNorthText>,
            Without<CompassSouthText>,
            // Without<CompassEastText>,
            Without<CompassWestText>,
            Without<CompassNorthEastText>,
            Without<CompassNorthWestText>,
            Without<CompassSouthEastText>,
            Without<CompassSouthWestText>,
        ),
    >,
    mut c_w: Query<
        &mut TextColor,
        (
            With<CompassWestText>,
            Without<CompassUpText>,
            Without<CompassDownText>,
            Without<CompassNorthText>,
            Without<CompassSouthText>,
            Without<CompassEastText>,
            // Without<CompassWestText>,
            Without<CompassNorthEastText>,
            Without<CompassNorthWestText>,
            Without<CompassSouthEastText>,
            Without<CompassSouthWestText>,
        ),
    >,
    mut c_ne: Query<
        &mut TextColor,
        (
            With<CompassNorthEastText>,
            Without<CompassUpText>,
            Without<CompassDownText>,
            Without<CompassNorthText>,
            Without<CompassSouthText>,
            Without<CompassEastText>,
            Without<CompassWestText>,
            // Without<CompassNorthEastText>,
            Without<CompassNorthWestText>,
            Without<CompassSouthEastText>,
            Without<CompassSouthWestText>,
        ),
    >,
    mut c_nw: Query<
        &mut TextColor,
        (
            With<CompassNorthWestText>,
            Without<CompassUpText>,
            Without<CompassDownText>,
            Without<CompassNorthText>,
            Without<CompassSouthText>,
            Without<CompassEastText>,
            Without<CompassWestText>,
            Without<CompassNorthEastText>,
            // Without<CompassNorthWestText>,
            Without<CompassSouthEastText>,
            Without<CompassSouthWestText>,
        ),
    >,
    mut c_se: Query<
        &mut TextColor,
        (
            With<CompassSouthEastText>,
            Without<CompassUpText>,
            Without<CompassDownText>,
            Without<CompassNorthText>,
            Without<CompassSouthText>,
            Without<CompassEastText>,
            Without<CompassWestText>,
            Without<CompassNorthEastText>,
            Without<CompassNorthWestText>,
            // Without<CompassSouthEastText>,
            Without<CompassSouthWestText>,
        ),
    >,
    mut c_sw: Query<
        &mut TextColor,
        (
            With<CompassSouthWestText>,
            Without<CompassUpText>,
            Without<CompassDownText>,
            Without<CompassNorthText>,
            Without<CompassSouthText>,
            Without<CompassEastText>,
            Without<CompassWestText>,
            Without<CompassNorthEastText>,
            Without<CompassNorthWestText>,
            Without<CompassSouthEastText>,
            // Without<CompassSouthWestText>,
        ),
    >,
) {
    let loc = location.0.clone();

    for _ev in new_zone_evs.read() {
        let colors = [
            (Direction::Up, c_u.get_single_mut()),
            (Direction::Down, c_d.get_single_mut()),
            (Direction::North, c_n.get_single_mut()),
            (Direction::South, c_s.get_single_mut()),
            (Direction::East, c_e.get_single_mut()),
            (Direction::West, c_w.get_single_mut()),
            (Direction::NorthEast, c_ne.get_single_mut()),
            (Direction::NorthWest, c_nw.get_single_mut()),
            (Direction::SouthEast, c_se.get_single_mut()),
            (Direction::SouthWest, c_sw.get_single_mut()),
        ];

        if let Some(from) = zones.0.get(&loc) {
            // get where they're going based on direction
            // set players location to the new one
            let Some(connections) = zone_assets.get(from).map(|asset| asset.connections.clone())
            else {
                // TODO: tell the player they cant go there
                continue;
            };

            for (dir, color) in colors {
                if connections.contains_key(&dir) {
                    _ = color.map(|mut color| color.0 = AMBER_500.into());
                } else {
                    _ = color.map(|mut color| color.0 = GRAY_500.into());
                }
            }
        } else {
            error!(
                "the player is at a location that is unknown to the engine. something went VERY wrong."
            );
            error!("{zones:?}.get({loc})")
        }
    }
}
