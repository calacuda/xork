use super::{InvIndex, InvNavDir, default_clear_main_window};
use crate::{
    items::{InventoryEntry, ItemAsset, ItemTypeName, Items},
    state::{InventoryState, MainScreenState},
    ui::MainTextUiNode,
};
use bevy::{color::palettes::tailwind::AMBER_500, prelude::*};
use std::cmp::min;

#[derive(Component)]
pub struct ItemDisplayText;

#[derive(Component, Debug)]
pub struct InventoryOrder(pub usize);

#[derive(Clone, Debug)]
pub struct AllItemsPlugin;

impl Plugin for AllItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(InventoryState::All), default_clear_main_window)
            .add_systems(OnEnter(InventoryState::All), setup_all_items_inventory_menu)
            .add_systems(
                Update,
                (display_items, navigate_inventory)
                    .run_if(in_state(MainScreenState::Inventory))
                    .run_if(in_state(InventoryState::All)),
            );
    }
}

fn navigate_inventory(
    mut nav_evs: EventReader<InvNavDir>,
    inventory: Query<&InventoryEntry>,
    mut inv_index: ResMut<InvIndex>,
    // mut inv_screen: ResMut<NextState<InventoryState>>,
) {
    let inv_size = inventory.iter().len();

    for ev in nav_evs.read() {
        match ev {
            InvNavDir::Up if inv_index.0 > 0 => {
                inv_index.0 -= 1;

                if inv_index.1 + 1 > inv_index.0 && inv_index.1 > 0 {
                    inv_index.1 -= 1;
                }
            }
            // InvNavDir::Up if inv_index.1 > 0 => {
            //     warn!("up");
            //     inv_index.1 -= 1;
            // }
            InvNavDir::Down if inv_index.0 < inv_size - 1 => {
                inv_index.0 += 1;

                if inv_index.0 + 1 >= inv_index.1 + 18 {
                    inv_index.1 += 1;
                }
            }
            // InvNavDir::Down if inv_index.1 < inv_size - 1 => {
            //     warn!("down");
            //     inv_index.1 += 1;
            // }
            // InvNavDir::NextScreen => inv_screen.set(),
            _ => {}
        }
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
                // ItemDisplayText,
            ));

            for i in 0..18 {
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
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

pub fn get_inventory_ordered(
    inventory: Query<&InventoryEntry>,
    item_assets: ResMut<Assets<ItemAsset>>,
    items: Res<Items>,
) -> Vec<ItemAsset> {
    let inv: Vec<_> = inventory.iter().collect();
    // inv.sort_by_key(|entry| inv_ord.0);
    let mut filtered: Vec<ItemAsset> = inv
        .clone()
        .into_iter()
        .filter_map(|item| {
            if let Some(item_asset) = items.0.get(&item.asset_path) {
                item_assets.get(item_asset)
            } else {
                None
            }
        })
        .map(|asset| asset.clone())
        .collect();
    filtered.sort_by_key(|asset| asset.name.clone());

    filtered
}

pub fn display_items(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    item_assets: ResMut<Assets<ItemAsset>>,
    items: Res<Items>,
    text_q: Query<(Entity, &InventoryOrder), (With<ItemDisplayText>,)>,
    inventory: Query<&InventoryEntry>,
    inv_index: Res<InvIndex>,
) {
    let text_font = TextFont {
        font: asset_server.load("fonts/AnonymousPro.ttf"),
        ..default()
    };

    let mut text_nodes: Vec<_> = text_q.iter().collect();
    text_nodes.sort_by_key(|(_text, inv_ord)| inv_ord.0);
    let inv = get_inventory_ordered(inventory, item_assets, items);

    if !inv.is_empty() {
        (0..(min(18, inv.len()))).for_each(|i| {
            // let entry = inv[i];
            cmds.entity(text_nodes[i].0).despawn_descendants();
            // let item = inv[i + inv_index.1].clone();

            if let Some(item) = inv.get(i + inv_index.1) {
                let parent = cmds
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    })
                    .id();

                let intermediate = if i + inv_index.1 == inv_index.0 {
                    let tmp = cmds
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceEvenly,
                                ..default()
                            },
                            Outline {
                                width: Val::Px(5.),
                                offset: Val::Px(0.0),
                                color: AMBER_500.into(),
                            },
                        ))
                        .id();

                    cmds.entity(tmp).add_child(parent);
                    tmp
                } else {
                    let tmp = cmds
                        .spawn((Node {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceEvenly,
                            ..default()
                        },))
                        .id();
                    cmds.entity(tmp).add_child(parent);
                    tmp
                };

                // cmds.entity(text_nodes[i].0).add_child(parent);

                cmds.entity(text_nodes[i].0).add_child(intermediate);

                cmds.entity(parent).with_children(|parent| {
                    parent.spawn((
                        Text::new(format!("#{: <3} =>", i + 1 + inv_index.1,)),
                        text_font.clone().with_font_size(30.0),
                        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                        TextColor(AMBER_500.into()),
                        ItemDisplayText,
                    ));
                    parent.spawn((
                        Text::new(item.name.clone()),
                        text_font.clone().with_font_size(30.0),
                        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                        TextColor(AMBER_500.into()),
                        ItemDisplayText,
                    ));
                    parent.spawn((
                        Text::new(format!("{}", ItemTypeName::from(item.item_data.clone()))),
                        text_font.clone().with_font_size(30.0),
                        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                        TextColor(AMBER_500.into()),
                        ItemDisplayText,
                    ));
                });

                if i + inv_index.1 == inv_index.0 {
                    cmds.entity(intermediate).with_children(|parent| {
                        parent.spawn((
                            Text::new(item.description.clone()),
                            text_font.clone().with_font_size(30.0),
                            TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                            TextColor(AMBER_500.into()),
                            ItemDisplayText,
                        ));
                    });
                }
                //     }
                // }
            }
        })
    } else {
        cmds.entity(text_nodes[0].0).despawn_descendants();
        cmds.entity(text_nodes[0].0).with_children(|parent| {
            parent.spawn((
                Text::new("Inventory Empty"),
                text_font.clone().with_font_size(30.0),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                TextColor(AMBER_500.into()),
                ItemDisplayText,
                InventoryOrder(0),
            ));
        });
    }
}
