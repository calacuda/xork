use crate::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Asset, TypePath)]
pub struct MobAsset {
    name: String,
    spawn_rate: f32,
    atk: f32,
    def: f32,
    mag_atk: f32,
    mag_def: f32,
    speed: f32,
    agro: f32,
}

#[derive(Debug, Clone, Default, Resource)]
pub struct Mobs(pub HashMap<String, Handle<MobAsset>>);
