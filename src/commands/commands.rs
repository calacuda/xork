use bevy::prelude::*;
use clap::{ArgAction, Parser};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumDiscriminants, EnumString};

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
    cmd: GameCmd,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Event,
    Parser,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumDiscriminants,
)]
#[strum_discriminants(derive(
    EnumString,
    Display,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
    Parser
))]
#[strum_discriminants(name(GameCmdName))]
#[command(version, about, no_binary_name(true), long_about = None)]
pub enum GameCmd {
    // #[serde(rename = "help")]
    // Help { query: Option<Box<ClientGameCmd>> },
    #[strum_discriminants(serde(rename = "go", alias = "move", alias = "walk"))]
    #[clap(alias = "move", alias = "walk")]
    Go {
        #[command(subcommand)]
        direction: Direction,
    },
    #[serde(
        rename = "look",
        alias = "examine",
        alias = "observe",
        alias = "behold"
    )]
    #[clap(alias = "examine", alias = "observe", alias = "behold")]
    Look,
    #[serde(rename = "take", alias = "pick-up", alias = "yoink")]
    #[clap(alias = "pick-up", alias = "yoink")]
    Take {
        // #[arg(action = ArgAction::Append, required = true, value_delimiter = ' ', num_args = 1.., use_value_delimiter = true)]
        // thing: Vec<String>,
    },
    /// lists item in the inventory
    #[serde(rename = "inventory", alias = "list", alias = "ls")]
    #[clap(alias = "inv", alias = "list", alias = "ls")]
    Inventory {},
    // TODO: make a "map" command that allow the player to check a mini map.
}

#[derive(Debug, Clone, Serialize, Deserialize, Event, Parser, PartialEq, Eq, PartialOrd, Ord)]
#[command(version, about, no_binary_name(true), long_about = None)]
pub enum SlashCmd {
    // #[clap(name = "/say")]
    // Say {
    //     // #[arg(short, long, num_args = 1, /* required_if_eq("to", "dm") */)]
    //     // player: Option<PlayerName>,
    //     #[arg(short, long, num_args = 1/* , requires_if("dm", "player") */)]
    //     // #[command()]
    //     // #[arg(long, required = false)]
    //     to: Option<ChatRoom>,
    //     #[arg(action = ArgAction::Append, required = true)]
    //     message: Vec<String>,
    // },
    #[clap(name = "/help", alias = "/?")]
    Help {
        // #[arg(required = true)]
        #[command(subcommand)]
        with: GameCmdName,
    },
    #[clap(name = "/save")]
    Save {
        #[arg(required = true)]
        // #[command(subcommand)]
        save_slot: u8,
    },
    #[clap(name = "/exit")]
    Exit {},
}

#[cfg(test)]
mod test {
    use super::{GameCmd, GameCmdName, SlashCmd};
    use clap::Parser;

    #[test]
    fn client_cmd_parse() {
        // GameCommand::parse_from(["go", "north"]);
        // let cmd = GameCommand::parse_from(["", "go", "north"]);
        // let cmd = ClientGameCmd::parse_from(["go", "north"]);
        let cmd = GameCmd::parse_from(["take", "nut", "cracker", "9000"]);
        // println!("{:?}", cmd.cmd);
        println!("{:?}", cmd);

        let cmd = GameCmd::parse_from(["go", "n"]);
        // println!("{:?}", cmd.cmd);
        println!("{:?}", cmd);

        let cmd = GameCmd::try_parse_from(["foo", "bar"]);
        // println!("{:?}", cmd.cmd);
        println!("{:?}", cmd);
        assert!(cmd.is_err(), "foo bar is a command now?, since when?")

        // assert_eq!(
        //     cmd,
        //     GameCmd::Go {
        //         direction: Direction::North
        //     },
        //     "expected cmd: \"go north\", got: {cmd:?}"
        // )

        // assert!(1 == 0);
    }

    #[test]
    fn client_slash_cmd_parse() {
        // TODO: write tests
        // let cmd = match SlashCmd::try_parse_from(["/say", "it's", "over", "9000!"]) {
        //     Ok(cmd) => cmd,
        //     Err(e) => panic!("{e}"),
        // };

        // println!("{:?}", cmd);

        // assert_eq!(
        //     cmd,
        //     SlashCmd::Say {
        //         // player: None,
        //         to: None,
        //         message: ["it's".into(), "over".into(), "9000!".into()].into()
        //     }
        // );

        // let cmd =
        //     match SlashCmd::try_parse_from(["/say", "--to", "server", "it's", "over", "9000!"]) {
        //         Ok(cmd) => cmd,
        //         Err(e) => panic!("{e}"),
        //     };
        // println!("{:?}", cmd);

        // assert_eq!(
        //     cmd,
        //     SlashCmd::Say {
        //         // player: Some("foo".into()),
        //         to: Some(ChatRoom::Server),
        //         message: ["it's".into(), "over".into(), "9000!".into()].into()
        //     }
        // );

        // let cmd =
        //     match SlashCmd::try_parse_from(["/say", "--to", "foobar", "it's", "over", "9000!"]) {
        //         Ok(cmd) => {
        //             // panic!("when the target (the \"to\" argument) a player name is required")
        //             panic!("{cmd:?}")
        //         }
        //         Err(e) => format!("{e}"),
        //     };
        // println!("{:?}", cmd);

        // assert!(1 == 0);
        // let cmd = match SlashCmd::try_parse_from(["/help"]) {
        //     Ok(cmd) => cmd,
        //     Err(e) => panic!("{e}"),
        // };

        // let cmd = SlashCmd::try_parse_from(["/help"]);

        // println!("{:?}", cmd);

        // assert!(
        //     cmd.is_ok_and(|cmd| cmd == SlashCmd::Help),
        //     "expected a \"/help\" command, got: {:?}",
        //     SlashCmd::try_parse_from(["/help"])
        // );
        // assert!(1 == 0);

        let cmd = match SlashCmd::try_parse_from(["/help", "go"]) {
            Ok(cmd) => cmd,
            Err(e) => panic!("{e}"),
        };

        assert_eq!(
            cmd,
            SlashCmd::Help {
                with: GameCmdName::Go
            },
            "expected a \"/help\" command, got: {cmd:?}",
        );
    }
}
