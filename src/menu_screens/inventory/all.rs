use super::default_clear_main_window;
use crate::{
    items::{InventoryEntry, ItemAsset, Items},
    state::{InventoryState, MainScreenState},
    ui::MainTextUiNode,
};
use bevy::{color::palettes::tailwind::AMBER_500, prelude::*};
use std::cmp::min;

#[derive(Component)]
pub struct ItemDisplayText;

#[derive(Component)]
pub struct InventoryOrder(pub usize);

#[derive(Clone, Debug)]
pub struct AllItemsPlugin;

impl Plugin for AllItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(InventoryState::All), default_clear_main_window)
            .add_systems(
                OnEnter(MainScreenState::Inventory),
                setup_all_items_inventory_menu,
            )
            .add_systems(
                Update,
                (display_items,)
                    .run_if(in_state(MainScreenState::Inventory))
                    .run_if(in_state(InventoryState::All)),
            );
        // .add_systems(
        //     OnExit(MainScreenState::Inventory),
        //     default_clear_main_window,
        // );
    }
}

pub fn setup_all_items_inventory_menu(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    main_screen: Query<Entity, With<MainTextUiNode>>,
) {
    let text_font = TextFont {
        font: asset_server.load("fonts/AnonymousPro.ttf"),
        ..default()
    };

    // error!("viewing all items");

    if let Ok(main_screen) = main_screen.get_single() {
        cmds.entity(main_screen).with_children(|parent| {
            parent.spawn((
                Text::new("Inventory => All Items"),
                text_font.clone().with_font_size(60.0),
                TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                TextColor(AMBER_500.into()),
                ItemDisplayText,
            ));

            for i in 0..12 {
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ItemDisplayText,
                    InventoryOrder(i),
                ));
            }
        });
    }
}

pub fn display_items(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    // text_screen: Res<TextPipeline>,
    // main_screen: Query<Entity, With<MainTextUiNode>>,
    item_assets: ResMut<Assets<ItemAsset>>,
    items: Res<Items>,
    // text_q: Query<(Entity, &TextLayoutInfo), (With<ItemDisplayText>,)>,
    // node_q: Query<&Node, (With<MainTextUiNode>,)>,
    text_q: Query<(Entity, &InventoryOrder), (With<ItemDisplayText>,)>,
    inventory: Query<&InventoryEntry>,
) {
    let text_font = TextFont {
        font: asset_server.load("fonts/AnonymousPro.ttf"),
        ..default()
    };

    let mut text_nodes: Vec<_> = text_q.iter().collect();
    text_nodes.sort_by_key(|(_text, inv_ord)| inv_ord.0);
    let inv: Vec<_> = inventory.iter().collect();

    if !inv.is_empty() {
        (0..(min(12, inv.len()))).for_each(|i| {
            let entry = inv[i];

            if let Some(item_asset) = items.0.get(&entry.asset_path) {
                if let Some(item) = item_assets.get(item_asset) {
                    // let text = format!("{: >3} => {} | {}", i + 1, item.name, item.description);
                    // text_nodes[i].0.0 = text;
                    cmds.entity(text_nodes[i].0).despawn_descendants();
                    cmds.entity(text_nodes[i].0).with_children(|parent| {
                        parent.spawn((
                            // Text::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
                            Text::new(format!("#{: <3} =>", i + 1,)),
                            text_font.clone().with_font_size(30.0),
                            TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                            // Wrap text in the rectangle
                            // TextBounds::from(box_size),
                            // // ensure the text is drawn on top of the box
                            // Transform::from_translation(Vec3::Z),
                            TextColor(AMBER_500.into()),
                            ItemDisplayText,
                            InventoryOrder(i),
                        ));
                        parent.spawn((
                            // Text::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
                            Text::new(item.name.clone()),
                            text_font.clone().with_font_size(30.0),
                            TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                            // Wrap text in the rectangle
                            // TextBounds::from(box_size),
                            // // ensure the text is drawn on top of the box
                            // Transform::from_translation(Vec3::Z),
                            TextColor(AMBER_500.into()),
                            ItemDisplayText,
                            InventoryOrder(i),
                        ));
                        parent.spawn((
                            // Text::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
                            Text::new(item.description.clone()),
                            text_font.clone().with_font_size(30.0),
                            TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                            // Wrap text in the rectangle
                            // TextBounds::from(box_size),
                            // // ensure the text is drawn on top of the box
                            // Transform::from_translation(Vec3::Z),
                            TextColor(AMBER_500.into()),
                            ItemDisplayText,
                            InventoryOrder(i),
                        ));
                    });
                }
            }
        })
    } else {
        cmds.entity(text_nodes[0].0).despawn_descendants();
        cmds.entity(text_nodes[0].0).with_children(|parent| {
            parent.spawn((
                // Text::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
                Text::new("Inventory Empty"),
                text_font.clone().with_font_size(30.0),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                // Wrap text in the rectangle
                // TextBounds::from(box_size),
                // // ensure the text is drawn on top of the box
                // Transform::from_translation(Vec3::Z),
                TextColor(AMBER_500.into()),
                ItemDisplayText,
                InventoryOrder(0),
            ));
        });
    }
}
