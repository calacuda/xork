use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod commands;

#[derive(Default, Debug, Clone, Serialize, Deserialize, Message, PartialEq, Eq, PartialOrd, Ord)]
pub struct BadCommand;
