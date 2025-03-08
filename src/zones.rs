// use super::messages::{
//     client::Direction,
//     server::{FlavorTextId, FlavorTextType},
// };
use crate::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::commands::commands::Direction;

pub type ZoneId = String;
pub type FlavorTextId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlavorTextType {
    Help,
    ZoneDescription,
    ZoneInspection,
    ItemDescription,
    SpellDescription,
    // ItemDescription,
    // ItemDescription,
}

#[derive(Debug, Serialize, Deserialize, Clone, Event)]
pub struct ZoneBuilder {
    /// self explanatory (many zones my have the same name)
    pub name: String,
    /// used for idetification purposes (not displayed to the user)
    pub id: ZoneId,
    /// which flavor text to use for the zone
    pub description: (FlavorTextType, FlavorTextId),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct Zone {
    /// self explanatory (many zones my have the same name)
    pub name: String,
    // /// used for idetification purposes (not displayed to the user)
    // pub id: ZoneId,
    /// flavor text that describes the zone
    pub description: String,
    /// the "look" text is none by default, gets set when the player uses the "look" command and
    /// the server response
    pub examine: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Asset, TypePath)]
pub struct ZoneAsset {
    /// self explanatory (many zones my have the same name)
    pub name: String,
    // /// used for idetification purposes (not displayed to the user)
    // pub id: ZoneId,
    /// flavor text that describes the zone
    pub description: String,
    /// the "look" text is none by default, gets set when the player uses the "look" command and
    /// the server response
    pub examine: Option<String>,
    pub connections: HashMap<Direction, String>,
    pub mob_spawn_rate: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct Location(pub ZoneId);

#[derive(Debug, Clone, Default, Resource)]
pub struct Zones(pub HashMap<ZoneId, Handle<ZoneAsset>>);
