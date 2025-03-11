use super::{default_clear_main_window, disable_cmd_prompt};
use crate::state::MainScreenState;
use bevy::prelude::*;

pub mod all;

#[derive(Clone, Debug)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((all::AllItemsPlugin,))
            .add_systems(OnEnter(MainScreenState::Inventory), disable_cmd_prompt)
            .add_systems(
                Update,
                (toggle_inventory, navigate_inventory).run_if(in_state(MainScreenState::Inventory)),
            )
            .add_systems(
                OnExit(MainScreenState::Inventory),
                default_clear_main_window,
            );
    }
}

fn toggle_inventory(
    keys: Res<ButtonInput<KeyCode>>,
    mut screen_state: ResMut<NextState<MainScreenState>>,
) {
    if keys.just_released(KeyCode::Escape) {
        screen_state.set(MainScreenState::MainGame);
    }
}

fn navigate_inventory(
    // mut history: ResMut<CmdHistory>,
    // TODO: add an inventory index that resets to zero in entering/exiting the inventory menu (or
    // any sub menu)
    keys: Res<ButtonInput<KeyCode>>,
    // mut text_input: Query<&mut TextInputValue>,
) {
    // TODO: Move up and down through the inventory list by publishing up and down events &
    // displaying a more detailed description of the item ONLY when selected.
}
