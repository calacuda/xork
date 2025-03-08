use bevy::prelude::*;
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
    /// sets up the client to get ready to login
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
    /// set when the player is battleing another player.
    Player,
    /// set when the player's party is duling another party.
    Party,
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
#[source(GameState = GameState::StatScreen)]
pub enum StatScreen {
    /// TODO: make a substate for this for the different inventory windows, consumable,
    /// equipment, weapons, etc.
    Inventory,
    #[default]
    Spells,
    PlayerStats,
}
