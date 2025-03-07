#![feature(let_chains)]
use bevy::prelude::*;
use commands::commands::Direction;
use fxhash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use zones::{FlavorTextId, FlavorTextType};

pub mod commands;
pub mod mobs;
pub mod state;
pub mod ui;
pub mod zones;

pub type Hash = u64;
pub type HashMap<K, V> = FxHashMap<K, V>;
pub type HashSet<V> = FxHashSet<V>;
pub type CommandResponseType = Result<GenerincFlavorText, GenerincFlavorText>;

#[derive(Debug, Clone, Event)]
pub struct CommandEntered(pub String);

#[derive(Debug, Clone, Event)]
pub struct CommandResultEvent(pub CommandResponseType);

#[derive(Event)]
pub struct PlayerMovement(pub Direction);

#[derive(Event)]
pub enum UiMessage {}

#[derive(Event)]
pub struct PlayerLook();

#[derive(Component, Clone, Debug)]
pub enum NotificationLevel {
    Error,
    Alert,
    Normal,
}

#[derive(Event, Clone, Debug)]
pub struct Notification {
    pub level: NotificationLevel,
    pub time_stamp: Instant,
    pub mesg: String,
}

/// used by the server to instruct the client on what flavor text to show the player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerincFlavorText {
    /// a specific server generated flavor text.
    Message(String),
    /// tells the client to pick a random flavor text of type FlavorTextType
    Random(FlavorTextType),
    /// tells the client to get a specific flavor text of type FlavorTextType, with an id
    Specific {
        flavor_type: FlavorTextType,
        /// the ID of the flavor text to show.
        id: FlavorTextId,
    },
}
