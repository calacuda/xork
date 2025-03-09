use super::default_clear_main_window;
use crate::{
    NewZone,
    state::MainScreenState,
    ui::{
        LookTextBody, MainTextBody, MainTextUiNode,
        update::{update_look_section, update_main_section},
    },
};
use bevy::{color::palettes::tailwind::AMBER_500, prelude::*};

#[derive(Clone, Debug)]
pub struct MainUiPlugin;

impl Plugin for MainUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MainScreenState::MainGame),
            (setup_main_menu, refresh_game_display).chain(),
        )
        .add_systems(OnExit(MainScreenState::MainGame), default_clear_main_window)
        .add_systems(
            Update,
            (update_main_section, update_look_section).run_if(in_state(MainScreenState::MainGame)),
        );
    }
}

pub fn refresh_game_display(mut new_zone_ev: EventWriter<NewZone>) {
    new_zone_ev.send_default();
}

pub fn setup_main_menu(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    main_screen: Query<Entity, With<MainTextUiNode>>,
) {
    let text_font = TextFont {
        font: asset_server.load("fonts/AnonymousPro.ttf"),
        ..default()
    };

    if let Ok(main_screen) = main_screen.get_single() {
        cmds.entity(main_screen).with_children(|parent| {
            parent.spawn((
                Text::default(),
                text_font.clone().with_font_size(30.0),
                TextLayout::new_with_justify(JustifyText::Left)
                    .with_linebreak(LineBreak::WordBoundary),
                TextColor(AMBER_500.into()),
                Node {
                    margin: UiRect {
                        left: Val::Percent(2.5),
                        right: Val::Percent(2.5),
                        top: Val::Percent(1.25),
                        bottom: Val::Percent(1.25),
                    },
                    ..Default::default()
                },
                MainTextBody,
            ));
            parent.spawn((
                Text::default(),
                text_font.clone().with_font_size(30.0),
                TextLayout::new_with_justify(JustifyText::Left)
                    .with_linebreak(LineBreak::WordBoundary),
                TextColor(AMBER_500.into()),
                Node {
                    margin: UiRect {
                        left: Val::Percent(2.5),
                        right: Val::Percent(2.5),
                        top: Val::Percent(1.25),
                        bottom: Val::Percent(1.25),
                    },
                    ..Default::default()
                },
                LookTextBody,
            ));
        });
    }
}
