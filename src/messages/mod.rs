use bevy::prelude::*;
use bevy_renet::renet::{ClientId, ConnectionConfig};
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use client::{ClientChannel, ClientGameCmd, ClientSlashCmd};
use serde::{Deserialize, Serialize};
use server::ServerChannel;

pub mod client;
pub mod server;

pub type PlayerTextCmd = ClientGameCmd;
pub type PlayerSystemCmd = ClientSlashCmd;
pub type ChatTimeStamp = DateTime<Utc>;
pub type PlayerName = String;
pub type Item = String;
pub type ItemId = u64;
pub type Spell = String;
pub type Enemy = String;
pub type Ally = String;

#[derive(Debug, Component)]
pub struct Player {
    pub id: ClientId,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Resource, ValueEnum,
)]
// #[command()]
pub enum ChatRoom {
    // #[arg()]
    Server,
    // #[arg()]
    Zone,
    // #[arg()]
    Guild,
    // #[arg()]
    Party,
    #[clap(skip)]
    Dm(PlayerName),
    // Dm,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub enum PlayerGameCommand {
    GameComamnd { command: PlayerTextCmd },
    SlashCommand { command: PlayerSystemCmd },
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channels_config(),
        server_channels_config: ServerChannel::channels_config(),
    }
}
