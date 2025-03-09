use crate::{
    ChangeScreen,
    commands::commands::ViewScreen,
    state::{InventoryState, MainScreenState, MainState},
    ui::MainTextUiNode,
};
use bevy::prelude::*;

pub mod inventory;
pub mod main_game;

#[derive(Clone, Debug)]
pub struct MenuScreensPlugin;

impl Plugin for MenuScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((inventory::InventoryPlugin, main_game::MainUiPlugin))
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
    // let text_font = TextFont {
    //     font: asset_server.load("fonts/AnonymousPro.ttf"),
    //     ..default()
    // };

    if let Ok(main_screen) = main_screen.get_single() {
        cmds.entity(main_screen).despawn_descendants();
    }
}
