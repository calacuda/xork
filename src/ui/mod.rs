use crate::{
    commands::{
        BadCommand,
        commands::{GameCmd, SlashCmd},
    },
    handle_player_move::{compass_update, set_main_body},
    menu_screens::MenuScreensPlugin,
    state::{GameState, MainScreenState, MainState},
};
use bevy::{
    color::palettes::{css::GREEN, tailwind::AMBER_500},
    prelude::*,
    render::camera::Viewport,
    window::WindowResized,
};
use bevy_simple_text_input::{
    TextInput, TextInputSubmitEvent, TextInputSystem, TextInputTextColor, TextInputTextFont,
    TextInputValue,
};
use clap::Parser;
use std::{collections::VecDeque, f32::consts::PI};
use update::{UpdateLookSectionText, UpdateMainSectionText};

pub mod update;

#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct VisualizationCamera;

#[derive(Component)]
pub struct TopLevelUiNode;

#[derive(Component)]
pub struct MainTextUiNode;

#[derive(Component)]
pub struct MainTextBody;

#[derive(Component)]
pub struct LookTextBody;

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

#[derive(Resource, Debug)]
pub struct CmdHistory {
    pub history: VecDeque<String>,
    pub scroll_i: usize,
    pub line_storage: Option<String>,
    pub max_len: usize,
}

impl Default for CmdHistory {
    fn default() -> Self {
        let max_len = 100;
        Self {
            history: VecDeque::with_capacity(max_len),
            scroll_i: 0,
            line_storage: None,
            max_len,
        }
    }
}

impl CmdHistory {
    pub fn push(&mut self, cmd: String) {
        self.history.push_front(cmd);

        if self.history.len() == self.max_len {
            self.history.pop_back();
        }
    }

    pub fn get_selected(&self) -> String {
        self.history[self.scroll_i].clone()
    }
}

#[derive(Clone, Debug)]
pub struct TextUiPlugin;

impl Plugin for TextUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameCmd>()
            .add_event::<SlashCmd>()
            .add_event::<BadCommand>()
            .add_event::<UpdateMainSectionText>()
            .add_event::<UpdateLookSectionText>()
            .init_resource::<CmdHistory>()
            .add_plugins(MenuScreensPlugin)
            .add_systems(OnEnter(MainState::InGame), (camera_setup, spawn_cube))
            .add_systems(
                Update,
                (rotate, set_camera_viewports, compass_update)
                    .run_if(in_state(MainState::InGame))
                    .run_if(not(in_state(GameState::Startup))),
            )
            .add_systems(
                Update,
                (
                    listener,
                    set_main_body.run_if(in_state(MainScreenState::MainGame)),
                    update_cmd_history,
                    navigate_cmd_history.run_if(in_state(MainScreenState::MainGame)),
                )
                    .after(TextInputSystem)
                    .run_if(in_state(MainState::InGame))
                    .run_if(not(in_state(GameState::Startup))),
            );
    }
}

fn listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut cmd_event: EventWriter<GameCmd>,
    mut slash_cmd_event: EventWriter<SlashCmd>,
    mut bad_cmd_event: EventWriter<BadCommand>,
) {
    for event in events.read() {
        info!("Player submitted command: {}", event.value);
        let cmd = event.value.clone();

        if !cmd.starts_with("/") {
            // parse to cmd
            let command = GameCmd::try_parse_from(cmd.split_whitespace());

            match command {
                Ok(command) => {
                    // fire command evvent
                    cmd_event.send(command);
                }
                Err(_e) => {
                    // fire unrecognized command event
                    bad_cmd_event.send_default();
                }
            }
        } else {
            // parse to slash cmd
            let command = SlashCmd::try_parse_from(cmd.split_whitespace());

            match command {
                Ok(command) => {
                    // fire shash command event
                    slash_cmd_event.send(command);
                }
                Err(_e) => {
                    // fire unrecognized command event
                    bad_cmd_event.send_default();
                }
            }
        }
    }
}

fn camera_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_font = TextFont {
        font: asset_server.load("fonts/AnonymousPro.ttf"),
        ..default()
    };

    commands.insert_resource(ClearColor(Color::BLACK));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(256, 256),
                ..default()
            }),
            order: 0,
            ..default()
        },
        VisualizationCamera,
        // ClearColorConfig: (Color::BLACK),
    ));

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(1_000.0, 1_000.0, 1_000.0)
            .looking_at(Vec3::new(1_000.0, 1_000.0, 1_000.0), Vec3::Y),
        Camera {
            order: 1,
            ..default()
        },
        UiCamera,
    ));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
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
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
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
                    parent.spawn((
                        Node {
                            height: Val::Percent((8.0 / 9.0) * (100.0 - 7.5)),
                            flex_direction: FlexDirection::Column,
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
                        MainTextUiNode,
                    ));
                    // Spawn Command Prompt
                    parent
                        .spawn((
                            Node {
                                height: Val::Percent(((9.0 - 8.0) / 9.0) * (100.0 - 10.)),
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
                            CmdPrompt,
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
                                // Text::new("go north"),
                                TextInput,
                                TextInputTextColor(TextColor(AMBER_500.into())),
                                TextInputTextFont(text_font.clone().with_font_size(60.0)),
                                TextLayout::new_with_justify(JustifyText::Left),
                                Node {
                                    margin: UiRect {
                                        left: Val::Px(0.0),
                                        right: Val::Px(2.5),
                                        top: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                        // bottom: Val::Px(0.0),
                                    },
                                    ..Default::default()
                                },
                                CmdPrompt,
                            ));
                        });
                });
            parent
                .spawn((Node {
                    width: Val::Percent((6.0 / 16.0) * (100.0 - 5.0)),
                    height: Val::Percent(100.0 - 4.125),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::End,

                    margin: UiRect {
                        left: Val::Percent(0.),
                        right: Val::Percent(2.5),
                        top: Val::Percent(2.5),
                        bottom: Val::Percent(2.5),
                    },
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                // width: Val::Percent((6.0 / 16.0) * (100.0 - 7.5)),
                                width: Val::Percent(100.0),
                                // height: Val::Percent((7.5 / 9.0) * 100.0),
                                height: Val::Percent(0.5 * 100.0),
                                flex_direction: FlexDirection::Column,

                                margin: UiRect {
                                    left: Val::Percent(2.5),
                                    right: Val::Percent(0.0),
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
                                .spawn((Node {
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
                                },))
                                .with_children(|parent| {
                                    // TODO: Spawn "STATS" label text here
                                    parent.spawn((
                                        Text::new("STATS: "),
                                        text_font.clone().with_font_size(60.0),
                                        TextColor(AMBER_500.into()),
                                        TextLayout::new_with_justify(JustifyText::Left)
                                            .with_linebreak(LineBreak::WordBoundary),
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
                                    // Spawn compass text
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
}

fn spawn_cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let cube = meshes.add(Cuboid::default());

    let rot_1 = Quat::from_rotation_x(45.0 * (-PI / 180.0));
    // rot.y = -PI * 2.;
    // rot.z = -PI * 2.0;
    let rot_2 = Quat::from_rotation_y(36.25 * (-PI / 180.0));

    commands.spawn((
        Mesh3d(cube),
        // MeshMaterial3d(debug_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(rot_1 * rot_2),
        Shape,
    ));
}

fn spawn_compass(
    parent: &mut <EntityCommands<'_> as BuildChildren>::Builder<'_>,
    text_font: TextFont,
) {
    parent
        .spawn((
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
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("U"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassUpText,
                    ));
                    // parent.spawn(
                    //     (
                    //         Text::new(" "),
                    //         text_font.clone().with_font_size(25.0),
                    //         TextLayout::new_with_justify(JustifyText::Center),
                    //     )
                    // );
                    parent.spawn((
                        Text::new("D"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassDownText,
                    ));
                });
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("NW"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassNorthWestText,
                    ));
                    parent.spawn((
                        Text::new("W"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassWestText,
                    ));
                    parent.spawn((
                        Text::new("SW"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassSouthWestText,
                    ));
                });
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("N"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassNorthText,
                    ));
                    parent.spawn((
                        Text::new("*"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    parent.spawn((
                        Text::new("S"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassSouthText,
                    ));
                });
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("NE"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassNorthEastText,
                    ));
                    parent.spawn((
                        Text::new("E"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassEastText,
                    ));
                    parent.spawn((
                        Text::new("SE"),
                        text_font.clone().with_font_size(25.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        CompassSouthEastText,
                    ));
                });
        });
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs());
    }
}

pub fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut camera: Query<&mut Camera, With<VisualizationCamera>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let mut camera = camera.single_mut();
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(
                (window.resolution.physical_width() as f32 * (10.0 / 16.0)) as u32,
                0,
            ),
            physical_size: UVec2::new(
                (window.resolution.physical_width() as f32 * (6.0 / 16.0)) as u32,
                window.resolution.physical_height() / 2,
            ),
            ..default()
        });
    }
}

fn update_cmd_history(
    mut events: EventReader<TextInputSubmitEvent>,
    mut history: ResMut<CmdHistory>,
) {
    for ev in events.read() {
        let cmd = ev.value.clone();

        history.push(cmd);
    }
}

fn navigate_cmd_history(
    mut history: ResMut<CmdHistory>,
    keys: Res<ButtonInput<KeyCode>>,
    mut text_input: Query<&mut TextInputValue>,
) {
    if let Ok(ref mut text_input) = text_input.get_single_mut()
        && history.history.len() > 0
    {
        if keys.just_pressed(KeyCode::ArrowUp) {
            if history.line_storage.is_some() {
                history.scroll_i += 1;
                history.scroll_i %= history.history.len();
            } else {
                history.line_storage = Some(text_input.0.clone());
            }

            text_input.0 = history.get_selected();
        } else if keys.just_pressed(KeyCode::ArrowDown) {
            text_input.0 = if history.scroll_i > 0 {
                history.scroll_i -= 1;
                history.get_selected()
            } else if let Some(cmd) = history.line_storage.clone() {
                history.line_storage = None;
                cmd
            } else {
                return;
            };
        }
    }
}

// /// Creates a colorful test pattern
// fn uv_debug_texture() -> Image {
//     const TEXTURE_SIZE: usize = 8;
//
//     let mut palette: [u8; 32] = [
//         255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
//         198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
//     ];
//
//     let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
//     for y in 0..TEXTURE_SIZE {
//         let offset = TEXTURE_SIZE * y * 4;
//         texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
//         palette.rotate_right(4);
//     }
//
//     Image::new_fill(
//         Extent3d {
//             width: TEXTURE_SIZE as u32,
//             height: TEXTURE_SIZE as u32,
//             depth_or_array_layers: 1,
//         },
//         TextureDimension::D2,
//         &texture_data,
//         TextureFormat::Rgba8UnormSrgb,
//         RenderAssetUsages::RENDER_WORLD,
//     )
// }
