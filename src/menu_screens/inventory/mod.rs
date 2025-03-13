use super::{default_clear_main_window, disable_cmd_prompt};
use crate::state::{InventoryState, MainScreenState};
use bevy::prelude::*;

pub mod all;

#[derive(Event)]
pub enum InvNavDir {
    Up,
    Down,
    // LastScreen,
    // NextScreen,
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct InvIndex(pub usize, pub usize); // .0 is the index of the selected item, and .1 is the
// start window, index

#[derive(Clone, Debug)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InvNavDir>()
            .init_resource::<InvIndex>()
            .add_plugins((all::AllItemsPlugin,))
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
    keys: Res<ButtonInput<KeyCode>>,
    mut nav_ev: EventWriter<InvNavDir>,
    mut inv_index: ResMut<InvIndex>,
    inv_state: Res<State<InventoryState>>,
    mut next_inv_state: ResMut<NextState<InventoryState>>,
) {
    // TODO: Move up and down through the inventory list by publishing up and down events &
    // displaying a more detailed description of the item ONLY when selected.
    if keys.just_released(KeyCode::ArrowUp) {
        nav_ev.send(InvNavDir::Up);
    } else if keys.just_released(KeyCode::ArrowDown) {
        nav_ev.send(InvNavDir::Down);
    } else if keys.just_released(KeyCode::ArrowLeft) {
        // nav_ev.send(InvNavDir::LastScreen);
        inv_index.0 = 0;
        inv_index.1 = 0;

        match inv_state.get() {
            InventoryState::All => next_inv_state.set(InventoryState::KeyItems),
            InventoryState::Consumables => next_inv_state.set(InventoryState::All),
            InventoryState::Weapons => next_inv_state.set(InventoryState::Consumables),
            InventoryState::Equipment => next_inv_state.set(InventoryState::Weapons),
            InventoryState::KeyItems => next_inv_state.set(InventoryState::Equipment),
        }
    } else if keys.just_released(KeyCode::ArrowRight) {
        // nav_ev.send(InvNavDir::NextScreen);
        inv_index.0 = 0;
        inv_index.1 = 0;

        match inv_state.get() {
            InventoryState::All => next_inv_state.set(InventoryState::Consumables),
            InventoryState::Consumables => next_inv_state.set(InventoryState::Weapons),
            InventoryState::Weapons => next_inv_state.set(InventoryState::Equipment),
            InventoryState::Equipment => next_inv_state.set(InventoryState::KeyItems),
            InventoryState::KeyItems => next_inv_state.set(InventoryState::All),
        }
    }
}
