use std::f32::consts::PI;

use bevy::{
    asset::RenderAssetUsages, color::palettes::{css::GREEN, tailwind::{AMBER_50, AMBER_500, AMBER_950}}, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}
};

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct TopLevelUiNode;

#[derive(Component)]
pub struct MainTextUiNode;

#[derive(Component)]
pub struct MainTextBody;

#[derive(Component)]
pub struct CmdPrompt;

#[derive(Component)]
pub struct CompassUpText;

#[derive(Component)]
pub struct CompassDownText;

#[derive(Component)]
pub struct CompassNorthText;

#[derive(Component)]
pub struct CompassSouthText;

#[derive(Component)]
pub struct CompassEastText;

#[derive(Component)]
pub struct CompassWestText;

#[derive(Component)]
pub struct CompassNorthEastText;

#[derive(Component)]
pub struct CompassNorthWestText;

#[derive(Component)]
pub struct CompassSouthEastText;

#[derive(Component)]
pub struct CompassSouthWestText;

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

#[derive(Clone, Debug)]
pub struct TextUiPlugin;

impl Plugin for TextUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .add_systems(Update, rotate);
    }
}

fn camera_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let text_font = TextFont {
        font: asset_server.load("fonts/AnonymousPro.ttf"),
        ..default()
    };

    commands.insert_resource(ClearColor(Color::BLACK));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 14.0).looking_at(Vec3::new(-3.5, 0.0, 0.0), Vec3::Y),
        GameCamera,
        // ClearColorConfig: (Color::BLACK),
    ));

    let cube = meshes.add(Cuboid::default());
    // let debug_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(images.add(uv_debug_texture())),
    //     ..default()
    // });

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                // align_items: AlignItems::Center,
                // justify_content: JustifyContent::SpaceEvenly,
                // position_type: PositionType::Absolute,
                // top: Val::ZERO,
                // left: Val::ZERO,
                ..Default::default()
            },
            // BackgroundColor(Color::BLACK),
            TopLevelUiNode,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent((10.0 / 16.0) * (100.0 - 7.5)),
                        // height: Val::Percent((7.5 / 9.0) * 100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        // align_items: AlignItems::Center,
                        // justify_content: JustifyContent::SpaceEvenly,
                        // justify_content: JustifyContent::End,
                        // justify_content: JustifyContent::Center,
                        // position_type: PositionType::Relative,
                        // top: Val::ZERO,
                        // left: Val::ZERO,
                        margin: UiRect {
                            left: Val::Percent(2.5),
                            right: Val::Percent(2.5),
                            top: Val::Percent(2.5),
                            bottom: Val::Percent(2.5),
                        },
                        ..Default::default()
                    },
                    // BackgroundColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            // Text::new("Use the panel on the right to change the Display and Visibility properties for the respective nodes of the panel on the left"),
                            TextSpan::default(),
                            // text_font.clone().with_font_size(30.0),
                            // TextLayout::new_with_justify(JustifyText::Left),
                            Node {
                                height: Val::Percent((8.0 / 9.0) * (100.0 - 7.5)),
                                // max_height: Val::Percent((8.0 / 9.0) * 100.0 - 5.0),
                                // width: Val::Percent(100.),
                                // height: Val::Percent(100.),
                                // margin: UiRect::bottom(Val::Px(10.)),
                                // margin: UiRect::bottom(Val::Percent(())),
                                // justify_content: JustifyContent::FlexStart,
                                flex_direction: FlexDirection::Column,
                                // margin: UiRect {
                                //     left: Val::Px(0.0),
                                //     right: Val::Px(0.0),
                                //     top: Val::Px(0.0),
                                //     bottom: Val::Px(0.0),
                                // },
                                overflow: Overflow::clip_y(),
                                ..Default::default()
                            }, 
                            // BackgroundColor(Color::BLACK),
                            // BorderColor(Color::WHITE),
                            Outline {
                                width: Val::Px(5.),
                                offset: Val::Px(0.0),
                                color: GREEN.into(),
                            },
                            MainTextBody,
                        ))
                        .with_children(|parent| {
                            // parent.spawn((
                            //     Text::new("Use the panel on the right to change the Display and Visibility properties for the respective nodes of the panel on the left"),
                            //     text_font.clone().with_font_size(30.0),
                            //     TextLayout::new_with_justify(JustifyText::Left),
                            // ));

                            // parent.spawn((
                            //     Text::new("Use the panel on the right to change the Display and Visibility properties for the respective nodes of the panel on the left"),
                            //     text_font.clone().with_font_size(45.0),
                            //     TextLayout::new_with_justify(JustifyText::Left),
                            // ));
                            for i in 0..10 {
                                parent.spawn((
                                    Text::new(format!("{i} -> Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")),
                                    text_font.clone().with_font_size(30.0),
                                    TextLayout::new_with_justify(JustifyText::Left).with_linebreak(LineBreak::WordBoundary),
                                    TextColor(AMBER_500.into()),
                                    Node {
                                        margin: UiRect {
                                            left: Val::Percent(2.5),
                                            right: Val::Percent(2.5),
                                            top: Val::Percent(1.25),
                                            bottom: Val::Percent(1.25),
                                        },
                                        ..Default::default()
                                    }
                                ));
                            }
                        });
                    // Spawn Command Prompt
                    parent
                        .spawn((
                            // Text::new("> "),
                            // Text::new("MOCK-CMD"),
                            // TextSpan::new(["> ", "MOCK-CMD"]),
                            TextSpan::default(),
                            // text_font.clone().with_font_size(60.0),
                            // TextLayout::new_with_justify(JustifyText::Left),
                            Node {
                                height: Val::Percent(((9.0 - 8.0) / 9.0) * (100.0 - 10.)),
                                // width: Val::Percent(100.),
                                // height: Val::Percent(100.),
                                // margin: UiRect::bottom(Val::Px(10.)),
                                // margin: UiRect::bottom(Val::Percent(())),
                                // justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                margin: UiRect {
                                    left: Val::Px(0.0),
                                    right: Val::Px(0.0),
                                    top: Val::Percent(2.5),
                                    bottom: Val::Px(0.0),
                                    // bottom: Val::Percent(2.5),
                                },
                                ..Default::default()
                            },
                            // BackgroundColor(Color::BLACK),
                            // BorderColor(Color::WHITE),
                            Outline {
                                width: Val::Px(5.),
                                offset: Val::Px(0.),
                                color: GREEN.into(),
                            },
                            // CmdPrompt,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("> "),
                                TextColor(AMBER_500.into()),
                                text_font.clone().with_font_size(60.0),
                                TextLayout::new_with_justify(JustifyText::Left),
                                Node {
                                    margin: UiRect {
                                        left: Val::Percent(2.5),
                                        right: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        bottom: Val::Percent(1.25),
                                        // bottom: Val::Px(0.0),
                                    },
                                    ..Default::default()
                                },
                            ));

                            parent.spawn((
                                Text::new("go north"),
                                TextColor(AMBER_500.into()),
                                text_font.clone().with_font_size(60.0),
                                TextLayout::new_with_justify(JustifyText::Left),
                                Node {
                                    margin: UiRect {
                                        left: Val::Px(0.0),
                                        right: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        bottom: Val::Percent(1.25),
                                        // bottom: Val::Px(0.0),
                                    },
                                    ..Default::default()
                                },
                                CmdPrompt,
                            ));
                    });
            });
            parent.spawn((
                Node {
                    width: Val::Percent((6.0 / 16.0) * (100.0 - 7.5)),
                    // height: Val::Percent((7.5 / 9.0) * 100.0),
                    height: Val::Percent(100.0 - 4.125),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::End,
                    // justify_content: JustifyContent::SpaceEvenly,
                    justify_content: JustifyContent::End,
                    // justify_content: JustifyContent::Center,
                    // position_type: PositionType::Relative,
                    // top: Val::ZERO,
                    // left: Val::ZERO,
                    margin: UiRect {
                        left: Val::Percent(0.),
                        right: Val::Percent(2.5),
                        top: Val::Percent(2.5),
                        bottom: Val::Percent(2.5),
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            // width: Val::Percent((6.0 / 16.0) * (100.0 - 7.5)),
                            width: Val::Percent(100.0),
                            // height: Val::Percent((7.5 / 9.0) * 100.0),
                            height: Val::Percent(0.5 * 100.0),
                            flex_direction: FlexDirection::Column,
                            // align_items: AlignItems::End,
                            // justify_content: JustifyContent::SpaceEvenly,
                            // justify_content: JustifyContent::End,
                            // justify_content: JustifyContent::Center,
                            // position_type: PositionType::Relative,
                            // top: Val::ZERO,
                            // left: Val::ZERO,
                            margin: UiRect {
                                left: Val::Percent(2.5),
                                right: Val::Percent(2.5),
                                top: Val::Percent(2.5),
                                bottom: Val::Percent(2.5),
                            },
                            ..Default::default()
                        },
                        // BackgroundColor(Color::BLACK),
                        // BorderColor(Color::WHITE),
                        Outline {
                            width: Val::Px(5.),
                            offset: Val::Px(0.),
                            color: GREEN.into(),
                        },
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    width: Val::Percent(100.0),
                                    margin: UiRect {
                                        left: Val::Percent(2.5),
                                        right: Val::Percent(2.5),
                                        top: Val::Percent(2.5),
                                        bottom: Val::Percent(2.5),
                                    },
                                    align_items: AlignItems::Start,
                                    ..Default::default()
                                },
                            ))
                            .with_children(|parent| {
                                // TODO: Spawn "STATS" label text here
                                parent.spawn((
                                    Text::new("STATS: "),
                                    text_font.clone().with_font_size(60.0),
                                    TextColor(AMBER_500.into()),
                                    TextLayout::new_with_justify(JustifyText::Left).with_linebreak(LineBreak::WordBoundary),
                                    Node {
                                        // flex_direction: FlexDirection::Row,
                                        margin: UiRect {
                                            left: Val::Percent(2.5),
                                            right: Val::Percent(7.5),
                                            top: Val::Percent(2.5),
                                            bottom: Val::Percent(2.5),
                                        },
                                        ..Default::default()
                                    },
                                ));
                                // TODO: Spawn "compass" label text here
                                spawn_compass(parent, text_font.clone());
                            });
                        // TODO: spawn stats display.
                    });
            });
    });

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    let rot = Quat::from_rotation_x(-PI / 4.);
    // rot.y = -PI / 4.;

    commands.spawn((
        Mesh3d(cube),
        // MeshMaterial3d(debug_material.clone()),
        Transform::from_xyz(2.0, 2.0, 2.0).with_rotation(rot),
        Shape,
    ));
}

fn spawn_compass(parent: &mut <EntityCommands<'_> as BuildChildren>::Builder<'_>, text_font: TextFont) {
    parent.spawn(
        (
            // Text::new("STATS:"),
            // text_font.clone().with_font_size(60.0),
            // TextLayout::new_with_justify(JustifyText::Left).with_linebreak(LineBreak::WordBoundary),
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                // align_content: AlignContent::SpaceAround,
                // justify_self: JustifySelf::End,
                // align_self: AlignSelf::End,
                margin: UiRect {
                    left: Val::Percent(2.5),
                    right: Val::Percent(2.5),
                    top: Val::Percent(2.5),
                    bottom: Val::Percent(2.5),
                },
                // UiRect::all(Val::Px(10.)),
                ..Default::default()
            },
            // BackgroundColor(Color::BLACK),
            // BorderColor(Color::WHITE),
            Outline {
                width: Val::Px(5.),
                offset: Val::Px(5.),
                color: GREEN.into(),
            },
        )
    ).with_children(
        |parent| {
            parent.spawn(
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                }
            ).with_children(
                |parent| {
                    parent.spawn(
                        (
                            Text::new("U"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassUpText,
                        )
                    );
                    // parent.spawn(
                    //     (
                    //         Text::new(" "),
                    //         text_font.clone().with_font_size(25.0),
                    //         TextLayout::new_with_justify(JustifyText::Center),
                    //     )
                    // );
                    parent.spawn(
                        (
                            Text::new("D"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassDownText,
                        )
                    );
                }
            );
            parent.spawn(
                Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                }
            ).with_children(
                |parent| {
                    parent.spawn(
                        (
                            Text::new("NW"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassNorthWestText,
                        )
                    );
                    parent.spawn(
                        (
                            Text::new("W"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassWestText,
                        )
                    );
                    parent.spawn(
                        (
                            Text::new("SW"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassSouthWestText,
                        )
                    );
                }
            );
            parent.spawn(
                Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                }
            ).with_children(
                |parent| {
                    parent.spawn(
                        (
                            Text::new("N"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassNorthText,
                        )
                    );
                    parent.spawn(
                        (
                            Text::new("*"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                        )
                    );
                    parent.spawn(
                        (
                            Text::new("S"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassSouthText,
                        )
                    );
                }
            );
            parent.spawn(
                Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                }
            ).with_children(
                |parent| {
                    parent.spawn(
                        (
                            Text::new("NE"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassNorthEastText,
                        )
                    );
                    parent.spawn(
                        (
                            Text::new("E"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassEastText,
                        )
                    );
                    parent.spawn(
                        (
                            Text::new("SE"),
                            text_font.clone().with_font_size(25.0),
                            TextLayout::new_with_justify(JustifyText::Center),
                            CompassSouthEastText,
                        )
                    );
                }
            );
        }
    );
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs());
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
