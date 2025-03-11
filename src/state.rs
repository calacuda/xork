use bevy::prelude::*;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, States)]
pub enum MainState {
    /// sets up the environment and what not.
    #[default]
    Setup,
    /// used when the player is playing the game.
    InGame,
    /// used to save the game state to a SaveState file, and prepare for taring the game down.
    Wrapup,
    /// the game closes upon entering this state.
    Exit,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, SubStates)]
#[source(MainState = MainState::InGame)]
pub enum GameState {
    /// waits for assets to load, etc.
    #[default]
    Startup,
    /// used when in the over world
    Adventure,
    /// used when in the dungeon
    Dungeon,
    /// set when the player is in a battle.
    Battle,
    /// set when the player is shopping from a market place
    Shopping,
    /// set when the player is in the status, inventory, or any alternate (not main game) screen
    StatScreen,
}

#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    SubStates,
)]
#[source(GameState = GameState::Battle)]
pub enum BattleWith {
    /// set when the player encounters & agros a mob.
    #[default]
    Mob,
    // /// set when the player is battleing another player.
    // Player,
    // /// set when the player's party is duling another party.
    // Party,
}

#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    SubStates,
)]
#[source(MainState = MainState::InGame)]
pub enum MainScreenState {
    Inventory,
    /// default view, used for when the player is NOT in a specialized menu.
    #[default]
    MainGame,
    /// view spells.
    Spells,
    /// viewing of the players stats.
    PlayerStats,
    /// view currently active quests, descriptions, & objectives.
    Quests,
    NotificationHistory,
}

#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    SubStates,
    ValueEnum,
)]
#[source(MainScreenState = MainScreenState::Inventory)]
pub enum InventoryState {
    #[default]
    #[serde(rename = "all", alias = "a", alias = "*")]
    #[clap(alias = "all", alias = "a", alias = "*")]
    All,
    #[serde(rename = "consumables", alias = "c")]
    #[clap(alias = "consumables", alias = "c")]
    Consumables,
    #[serde(rename = "equipment", alias = "equip", alias = "e")]
    #[clap(alias = "equipment", alias = "equip", alias = "e")]
    Equipment,
    #[serde(rename = "weapons", alias = "w")]
    #[clap(alias = "weapons", alias = "w")]
    Weapons,
    #[serde(rename = "key-items", alias = "keys")]
    #[clap(alias = "key-items", alias = "keys")]
    KeyItems,
}
