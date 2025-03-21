#![feature(let_chains)]
use bevy::{prelude::*, window::WindowResized};
use commands::commands::{Direction, ViewScreen};
use fxhash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use state::MainState;
use std::time::Instant;
use zones::{FlavorTextId, FlavorTextType};

pub mod commands;
pub mod handle_exit_command;
pub mod handle_game_cmd;
pub mod handle_player_look;
pub mod handle_player_move;
pub mod handle_slash_cmd;
pub mod items;
pub mod menu_screens;
pub mod mobs;
pub mod player_take;
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

#[derive(Event, Default)]
pub struct PlayerLook;

#[derive(Event, Default)]
pub struct ExitGame;

#[derive(Event, Default)]
pub struct NewZone;

#[derive(Event, Default)]
pub struct PlayerTake;

#[derive(Event)]
pub struct ChangeScreen {
    to_screen: ViewScreen,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct WindowSize(pub Vec2);

pub fn enter_in_game_state(mut next_state: ResMut<NextState<MainState>>) {
    next_state.set(MainState::InGame);
}

pub fn exit_game(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit::Success);
}

pub fn enter_exit_state(mut next_state: ResMut<NextState<MainState>>) {
    next_state.set(MainState::Exit);
}

pub fn maintain_window_size(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut window_size: ResMut<WindowSize>,
    // mut camera: Query<&mut Camera, With<VisualizationCamera>>,
) {
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        window_size.0 = window.size();
    }
}
