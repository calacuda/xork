use super::ChatRoom;
use bevy::prelude::*;
use bevy_renet::renet::{ChannelConfig, SendType};
use clap::{ArgAction, Parser};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// sent from client -> server
#[derive(Debug, Serialize, Deserialize, Event)]
pub struct ChatSay {
    /// who should hear this message
    pub chat_room: ChatRoom,
    /// what to say
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Event)]
pub enum ClientSystemCmd {
    Auth {
        user_name: String,
        pass_hash: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Parser, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    #[arg()]
    #[clap(alias = "n")]
    North,
    #[arg()]
    #[clap(alias = "s")]
    South,
    #[arg()]
    #[clap(alias = "e")]
    East,
    #[arg()]
    #[clap(alias = "w")]
    West,
    #[arg()]
    Up,
    #[arg()]
    Down,
    #[arg()]
    #[clap(alias = "ne")]
    NorthEast,
    #[arg()]
    #[clap(alias = "nw")]
    NorthWest,
    #[arg()]
    #[clap(alias = "se")]
    SouthEast,
    #[arg()]
    #[clap(alias = "sw")]
    SouthWest,
    /// go in shop
    #[arg()]
    In { place: String },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct GameCommand {
    #[command(subcommand)]
    cmd: ClientGameCmd,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event, Parser, PartialEq, Eq, PartialOrd, Ord)]
#[command(version, about, no_binary_name(true), long_about = None)]
pub enum ClientGameCmd {
    // #[serde(rename = "help")]
    // Help { query: Option<Box<ClientGameCmd>> },
    #[serde(rename = "go", alias = "move", alias = "walk")]
    #[clap(alias = "move", alias = "walk")]
    Go {
        #[command(subcommand)]
        direction: Direction,
    },
    #[serde(rename = "look", alias = "examine", alias = "observe")]
    #[clap(alias = "examine", alias = "observe")]
    Look,
    #[serde(rename = "take", alias = "pick-up", alias = "yoink")]
    #[clap(alias = "pick-up", alias = "yoink")]
    // #[command(action = ArgActions::Collect)]
    Take {
        #[arg(action = ArgAction::Append, required = true, value_delimiter = ' ', num_args = 1.., use_value_delimiter = true)]
        thing: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Event, Parser, PartialEq, Eq, PartialOrd, Ord)]
#[command(version, about, no_binary_name(true), long_about = None)]
pub enum ClientSlashCmd {
    #[clap(name = "/say")]
    Say {
        // #[arg(short, long, num_args = 1, /* required_if_eq("to", "dm") */)]
        // player: Option<PlayerName>,
        #[arg(short, long, num_args = 1/* , requires_if("dm", "player") */)]
        // #[command()]
        // #[arg(long, required = false)]
        to: Option<ChatRoom>,
        #[arg(action = ArgAction::Append, required = true)]
        message: Vec<String>,
    },
    #[clap(name = "/help")]
    Help,
}

/// the channels used by the client to send messages to the server.
#[derive(Debug, Clone, Copy)]
pub enum ClientChannel {
    /// ChatSay
    ChatMessage,
    /// PlayerGameCommand
    GameCommand,
    /// login/out, register, etc
    SystemCommand,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::GameCommand => 0,
            ClientChannel::ChatMessage => 1,
            ClientChannel::SystemCommand => 2,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        [
            ChannelConfig {
                channel_id: Self::GameCommand.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
            ChannelConfig {
                channel_id: Self::ChatMessage.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
            ChannelConfig {
                channel_id: Self::SystemCommand.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
        ]
        .into()
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use clap::Parser;

    use crate::common::messages::{ChatRoom, client::ClientSlashCmd};

    use super::ClientGameCmd;

    #[test]
    fn client_cmd_parse() {
        // GameCommand::parse_from(["go", "north"]);
        // let cmd = GameCommand::parse_from(["", "go", "north"]);
        // let cmd = ClientGameCmd::parse_from(["go", "north"]);
        let cmd = ClientGameCmd::parse_from(["take", "nut", "cracker", "9000"]);
        // println!("{:?}", cmd.cmd);
        println!("{:?}", cmd);

        // assert!(1 == 0);
    }

    #[test]
    fn client_slash_cmd_parse() {
        let cmd = match ClientSlashCmd::try_parse_from(["/say", "it's", "over", "9000!"]) {
            Ok(cmd) => cmd,
            Err(e) => panic!("{e}"),
        };

        println!("{:?}", cmd);

        assert_eq!(cmd, ClientSlashCmd::Say {
            // player: None,
            to: None,
            message: ["it's".into(), "over".into(), "9000!".into()].into()
        });

        let cmd = match ClientSlashCmd::try_parse_from([
            "/say", "--to", "server", "it's", "over", "9000!",
        ]) {
            Ok(cmd) => cmd,
            Err(e) => panic!("{e}"),
        };
        println!("{:?}", cmd);

        assert_eq!(cmd, ClientSlashCmd::Say {
            // player: Some("foo".into()),
            to: Some(ChatRoom::Server),
            message: ["it's".into(), "over".into(), "9000!".into()].into()
        });

        let cmd = match ClientSlashCmd::try_parse_from([
            "/say", "--to", "foobar", "it's", "over", "9000!",
        ]) {
            Ok(cmd) => {
                // panic!("when the target (the \"to\" argument) a player name is required")
                panic!("{cmd:?}")
            }
            Err(e) => format!("{e}"),
        };
        println!("{:?}", cmd);

        // assert!(1 == 0);
    }
}
