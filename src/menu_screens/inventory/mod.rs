use super::default_clear_main_window;
use crate::state::MainScreenState;
use bevy::prelude::*;

pub mod all;

#[derive(Clone, Debug)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((all::AllItemsPlugin,)).add_systems(
            OnExit(MainScreenState::Inventory),
            default_clear_main_window,
        );
    }
}
