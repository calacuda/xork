use crate::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub type ItemId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponHands {
    MainHand,
    OffHand,
    TwoHands,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Amount {
    Fixed(i32),
    Percent(f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stat {
    HP,
    Mana,
    Attack,
    Defence,
    MagicAtk,
    MagicDef,
    Speed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsumableEffect {
    Heal {
        amount: Amount,
        /// the duration over which the healing is done. (0.0 mean instant)
        duration: f32,
    },
    Buf {
        /// the stat to buf
        stat: Stat,
        /// how much to buf that stat
        amount: Amount,
        /// how long (in seconds) to apply this buf for.
        duration: f32,
    },
    // Cast {
    //     /// the spell to cast
    //     spell: SpellId
    // },
    InflictStatus {
        // ailment: StatusAilment,
        // duration: f32,
        // chance: f32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EquipmentEffect {
    HealUser {
        amount: Amount,
        /// the duration over which the healing is done. (0.0 means instant)
        duration: f32,
    },
    BufUser {
        /// the stat to buf
        stat: Stat,
        /// how much to buf that stat
        amount: Amount,
        /// how long (in seconds) to apply this buf for.
        duration: f32,
    },
    InflictStatus {
        // ailment: StatusAilment,
        // duration: f32,
        // chance: f32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    /// Single use items, i.e potions.
    Consumable { effects: Vec<ConsumableEffect> },
    Weapon {
        hands: WeaponHands,
        effects: Vec<EquipmentEffect>,
    },
    Armor {
        // armor_type: ArmorType,
        effects: Vec<EquipmentEffect>,
    },
    /// items that can be used multiple times, and items that unlock events/new places on the map.
    KeyItem { effects: Vec<ConsumableEffect> },
}

#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct ItemAsset {
    /// self explanatory (many items may have similar name)
    pub name: String,
    /// flavor text that describes the zone
    pub description: String,
    /// further explanations of the item
    pub examine: Option<String>,
    ///
    pub item_data: ItemType,
}

#[derive(Debug, Clone, Default, Resource)]
pub struct Items(pub HashMap<ItemId, Handle<ItemAsset>>);

#[derive(Debug, Clone, Default, Component)]
pub struct InventoryEntry {
    pub asset_path: ItemId,
}
