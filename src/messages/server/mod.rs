use crate::{
    client::state::{BattleWith, StatScreen},
    common::zones::ZoneId,
};

use super::{Ally, ChatRoom, ChatTimeStamp, Enemy, Item, PlayerName, Spell, client::Direction};
use bevy::prelude::*;
use bevy_renet::renet::{ChannelConfig, ClientId, SendType};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub type CommandResponseType = Result<GenerincFlavorText, GenerincFlavorText>;
pub type FlavorTextId = u64;

/// sent from server -> clients. notifies client that a player said something.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSaid {
    /// who said it
    pub player: PlayerName,
    /// who should hear this message
    pub chat_room: ChatRoom,
    /// what to say
    pub message: String,
    /// the time at which the server recved the message (in Utc)
    pub time_stamp: ChatTimeStamp,
}

// /// sent from server -> clients. notifies client that a player said something.
// #[derive(Debug, Serialize, Deserialize, Event)]
// pub enum

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

/// used by the server to instruct the client on what flavor text to show the player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerincFlavorText {
    /// a specific server generated flavor text.
    Message(String),
    /// tells the client to pick a random flavor text of type FlavorTextType
    Random(
        // TODO: make FlavorTextType enum
        FlavorTextType,
    ),
    /// tells the client to get a specific flavor text of type FlavorTextType, with an id
    Specific {
        flavor_type: FlavorTextType,
        /// the ID of the flavor text to show.
        id: FlavorTextId,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub enum ServerSystemMessagesEvent {
    PlayerEnteredZone {
        /// describes where the player came from. ("the door", "the [CARDINAL DIRECTION]", etc)
        from: Option<Direction>,
        player_name: PlayerName,
        to: ZoneId,
    },
    PlayerLeftZone {
        /// describes where the player went. ("through the door", "[CARDINAL DIRECTION]", etc)
        went: Option<Direction>,
        player_name: PlayerName,
        left: ZoneId,
    },
    ServerAnnouncement {
        message: String,
        is_bbs: bool,
        tts: bool,
    },
    // CommandFailed(String),
    /// used to signify that a command was successful (encoded as Ok(())) or that a command failed
    /// (encoded as Err(GenerincFlavorText), with the GenerincFlavorText representing a description of the error, a message
    /// to the player, etc).
    CommandResponse(CommandResponseType),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerSystemMessages {
    PlayerEnteredZone {
        /// describes where the player came from. ("the door", "the [CARDINAL DIRECTION]", etc)
        from: Option<Direction>,
        player_name: PlayerName,
    },
    PlayerLeftZone {
        /// describes where the player went. ("through the door", "[CARDINAL DIRECTION]", etc)
        went: Option<Direction>,
        player_name: PlayerName,
    },
    ServerAnnouncement {
        message: String,
        is_bbs: bool,
        tts: bool,
    },
    // CommandFailed(String),
    /// used to signify that a command was successful (encoded as Ok(())) or that a command failed
    /// (encoded as Err(GenerincFlavorText), with the GenerincFlavorText representing a description of the error, a message
    /// to the player, etc).
    CommandResponse(CommandResponseType),
}

#[derive(Debug, Serialize, Deserialize, Event)]
pub enum ServerZoneUpdate {
    New {
        name: String,
        flavor: String,
        // /// named npcs around the the zone
        // npc_names: Vec<String>,
        // /// mobs seen in the zone
        // mob_names: Vec<String>,
    },
    Examine {
        /// the further description
        flavor: Option<String>,
        // /// any items found upon closer inspection.
        // items: Vec<ItemId>,
    },
}

#[derive(Debug, Serialize, Deserialize, Event)]
pub enum ServerPlayerUpdate {
    /// player acquired Item
    AcquiredItem(Item),
    /// player lost Item
    LoseItem(Item),
    /// covers gained and lost money
    NewMoney(u32),
    /// xp can only be gained.
    GainedXp(u32),
    /// level can only go up.
    LevelUp(u8),
    NewSpell(Spell),
}

#[derive(Debug, Serialize, Deserialize, Event)]
pub enum StateChange {
    Shop {},
    Battle {
        battle_type: BattleWith,
        with: Vec<Enemy>,
        allies: Vec<Ally>,
    },
    /// normal mode
    Adventure,
    AltScreen(StatScreen),
}

#[derive(Debug, Serialize, Deserialize, Event)]
pub enum AccountMessage {
    LoggedIn,
    LoggedOut,
    Banned { reason: String },
    JoinedGuild { guild_name: String },
    LeftGuiled { kicked: bool, guild_name: String },
    RegisteredNewAccount,
}

/// the channels used by the server to send messages to a client.
#[derive(Debug, Clone, Copy)]
pub enum ServerChannel {
    /// a user said...
    /// sends message: PlayerSaid
    ChatMessage,
    /// players left/entered the zone, server/BBS anouncements, command failed
    /// sends message: ServerSystemMessages
    SystemMessages,
    /// new zone info
    /// sends message: ServerZoneUpdate
    ZoneUpdate,
    /// player gained xp, leveled up, got item, joined guild/party, etc
    /// sends message: ServerPlayerUpdate
    PlayerUpdate,
    /// enter shop mode, enter battle mode, enter adventure mode, etc
    /// sends message: StateChange
    StateChange,
    /// account made, logged in/out, banned, joined/left/kicked-from guild
    /// sends message: AccountMessage
    AccountMessages,
}

/// messages form the server
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate {
        entity: Entity,
        id: ClientId,
        translation: [f32; 3],
    },
    PlayerRemove {
        id: ClientId,
    },
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::SystemMessages => 0,
            ServerChannel::ZoneUpdate => 1,
            ServerChannel::StateChange => 2,
            ServerChannel::PlayerUpdate => 3,
            ServerChannel::ChatMessage => 4,
            ServerChannel::AccountMessages => 5,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        [
            ChannelConfig {
                channel_id: Self::SystemMessages.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::ReliableUnordered {
                    resend_time: Duration::from_millis(200),
                },
            },
            ChannelConfig {
                channel_id: Self::ZoneUpdate.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(100),
                },
            },
            ChannelConfig {
                channel_id: Self::StateChange.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(100),
                },
            },
            ChannelConfig {
                channel_id: Self::PlayerUpdate.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(350),
                },
            },
            ChannelConfig {
                channel_id: Self::ChatMessage.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                // this can be unordered bc the client can reorder the messages using the time
                // stamps
                send_type: SendType::ReliableUnordered {
                    resend_time: Duration::from_millis(400),
                },
            },
            ChannelConfig {
                channel_id: Self::AccountMessages.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(500),
                },
            },
        ]
        .into()
    }
}
