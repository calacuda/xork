use crate::{
    ChangeScreen,
    commands::commands::ViewScreen,
    state::{InventoryState, MainScreenState, MainState},
    ui::{CmdPrompt, MainTextUiNode},
};
use bevy::{
    color::palettes::{css::GREEN, tailwind::GRAY_500},
    prelude::*,
};
use bevy_simple_text_input::TextInputInactive;

pub mod inventory;
pub mod main_game;

#[derive(Clone, Debug)]
pub struct MenuScreensPlugin;

impl Plugin for MenuScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((inventory::InventoryPlugin, main_game::MainUiPlugin))
            .add_systems(OnEnter(MainScreenState::MainGame), enable_cmd_prompt)
            .add_systems(Update, change_screens.run_if(in_state(MainState::InGame)));
    }
}

pub fn change_screens(
    mut view_evs: EventReader<ChangeScreen>,
    mut screen_state: ResMut<NextState<MainScreenState>>,
    mut inv_screen_state: ResMut<NextState<InventoryState>>,
) {
    for ev in view_evs.read() {
        match ev.to_screen {
            ViewScreen::Game => screen_state.set(MainScreenState::MainGame),
            ViewScreen::Inventory { sub_screen } => {
                screen_state.set(MainScreenState::Inventory);

                if let Some(sub_screen) = sub_screen {
                    match sub_screen {
                        InventoryState::All => inv_screen_state.set(InventoryState::All),
                        InventoryState::Consumables => {
                            inv_screen_state.set(InventoryState::Consumables)
                        }
                        InventoryState::Weapons => inv_screen_state.set(InventoryState::Weapons),
                        InventoryState::KeyItems => inv_screen_state.set(InventoryState::KeyItems),
                        InventoryState::Equipment => {
                            inv_screen_state.set(InventoryState::Equipment)
                        }
                    }
                } else {
                    // inv_screen_state.set(InventoryState::All);
                }
            }
            ViewScreen::Stats => screen_state.set(MainScreenState::PlayerStats),
            ViewScreen::Spells => screen_state.set(MainScreenState::Spells),
            ViewScreen::Quests => screen_state.set(MainScreenState::Quests),
            ViewScreen::Notifications => screen_state.set(MainScreenState::NotificationHistory),
        }
    }
}

pub fn default_clear_main_window(
    mut cmds: Commands,
    // asset_server: Res<AssetServer>,
    main_screen: Query<Entity, With<MainTextUiNode>>,
) {
    if let Ok(main_screen) = main_screen.get_single() {
        cmds.entity(main_screen).despawn_descendants();
    }
}

pub fn disable_cmd_prompt(
    mut cmd_prompt: Query<&mut Outline, With<CmdPrompt>>,
    mut text_input: Query<&mut TextInputInactive>,
) {
    for ref mut prompt in cmd_prompt.iter_mut() {
        prompt.color = GRAY_500.into();
    }

    for ref mut input_off in text_input.iter_mut() {
        input_off.0 = true;
    }
}

pub fn enable_cmd_prompt(
    mut cmd_prompt: Query<&mut Outline, With<CmdPrompt>>,
    mut text_input: Query<&mut TextInputInactive>,
) {
    for ref mut prompt in cmd_prompt.iter_mut() {
        prompt.color = GREEN.into();
    }

    for ref mut input_off in text_input.iter_mut() {
        input_off.0 = false;
    }
}
