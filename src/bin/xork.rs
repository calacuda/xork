use bevy::{
    color::palettes::css::GREEN,
    log::{Level, LogPlugin},
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_common_assets::ron::RonAssetPlugin;
// use bevy_iced::{
//     IcedContext, IcedPlugin, IcedSettings,
//     iced::{
//         self, Alignment, Color, Length,
//         widget::{Column, Container, Row, container::rounded_box, text},
//     },
// };
use std::{error::Error, fs::read_dir, path::PathBuf};
use xork::{
    CommandEntered, CommandResultEvent, Notification, PlayerLook, PlayerMovement, UiMessage,
    mobs::{MobAsset, Mobs},
    state::{BattleWith, GameState},
    ui::TextUiPlugin,
    zones::{Zone, ZoneAsset, Zones},
};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Adventure;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Battle;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shopping;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InGame;

fn main() -> Result<(), Box<dyn Error>> {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        name: Some("Xork".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            RonAssetPlugin::<ZoneAsset>::new(&["zone.ron"]),
            RonAssetPlugin::<MobAsset>::new(&["mob.ron"]),
            TextUiPlugin,
            WireframePlugin,
            // Wireframe2dPlugin,
        ))
        // .add_plugins(IcedPlugin::default())
        .insert_resource(Zones::default())
        .insert_resource(Mobs::default())
        // .configure_sets(Update, Adventure.run_if(in_state(GameState::Adventure)))
        // .configure_sets(Update, InGame.run_if(not(in_state(GameState::Startup))))
        // .init_resource::<ClientTui>()
        .init_resource::<Zone>()
        // .insert_resource(Term(terminal))
        // .insert_resource(IcedSettings {
        //     scale_factor: None,
        //     theme: iced::Theme::CatppuccinMocha,
        //     // style: Style {
        //     //     text_color: iced::Color::from_rgb(0.0, 1.0, 1.0),
        //     // },
        //     ..Default::default()
        // })
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: GREEN.into(),
        })
        // .insert_resource(Wireframe2dConfig {
        //     // The global wireframe config enables drawing of wireframes on every mesh,
        //     // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
        //     // regardless of the global configuration.
        //     global: true,
        //     // Controls the default color of all wireframes. Used as the default color for global wireframes.
        //     // Can be changed per mesh using the `WireframeColor` component.
        //     default_color: WHITE.into(),
        // })
        .init_state::<GameState>()
        .init_state::<BattleWith>()
        .add_event::<CommandEntered>()
        // .add_event::<SystemCmd>()
        // .add_event::<PlayerGameCommand>()
        .add_event::<UiMessage>()
        .add_event::<PlayerMovement>()
        .add_event::<PlayerLook>()
        .add_event::<Notification>()
        .add_event::<CommandResultEvent>()
        .init_asset::<ZoneAsset>()
        .init_asset::<MobAsset>()
        .add_systems(Startup, (load_zone_assets, load_mob_assets))
        // .add_systems(
        //     Update,
        //     (
        //         // ui_system,
        //         // handle_events_system,
        //         // handle_player_movement,
        //         // handle_player_look,
        //         // handle_game_cmd,
        //         // handle_cmd_res,
        //         // handle_chat,
        //         // handle_send_zone,
        //         // handle_send_sys_msg,
        //     ),
        // )
        // TODO: Write audio login systems
        //
        // .add_systems(OnEnter(ClientState::Login), (auto_login_msg_send, auto_login_msg_recv).in_state(Connected) )
        // .add_systems(Update, (tui_update).in_set(InGame))
        // .set_runner(minimal_runner)
        .run();

    Ok(())
}

// fn spawn_notif(mut cmd: Commands) {
//     cmd.spawn(Notification {
//         level: NotificationLevel::Normal,
//         time_stamp: Instant::now(),
//         mesg: "foobar".into(),
//     });
// }

fn load_zone_assets(mut zones: ResMut<Zones>, asset_server: Res<AssetServer>) {
    let to_zones = PathBuf::from("assets/zones");

    read_dir(to_zones).unwrap().for_each(|zone_dir| {
        if let Ok(zone_dir) = zone_dir {
            read_dir(zone_dir.path()).unwrap().for_each(|zone| {
                if let Ok(zone) = zone {
                    let sub_path = format!(
                        "{}/{}",
                        zone_dir.file_name().to_str().unwrap(),
                        zone.file_name().to_str().unwrap()
                    );
                    let path = format!("zones/{}", sub_path);
                    info!("{path}");
                    zones.0.insert(sub_path, asset_server.load(path));
                }
            })
        }
    });
}

fn load_mob_assets(mut mobs: ResMut<Mobs>, asset_server: Res<AssetServer>) {
    let to_assets = PathBuf::from("assets/mobs");

    read_dir(to_assets).unwrap().for_each(|asset| {
        if let Ok(asset) = asset {
            let path = format!("mobs/{}", asset.file_name().to_str().unwrap());
            info!("{path}");
            mobs.0.insert(path.clone(), asset_server.load(path));
        }
    });
}

// fn ui_system(mut ctx: IcedContext<UiMessage>, notifs: Query<&Notification>) {
//     // ctx.display(text(format!(
//     //     "Hello Iced! Running for {:.2} seconds.",
//     //     time.elapsed_secs()
//     // )));
//     let mut notifs: Vec<Notification> = notifs.into_iter().map(|notif| notif.clone()).collect();
//     notifs.sort_by_key(|notif: &Notification| notif.time_stamp);
//     let notifs = notifs.into_iter().map(|notif| {
//         let (level, color) = match notif.level {
//             NotificationLevel::Error => ("[ERROR] ", Color::from_rgb8(255, 25, 50)),
//             NotificationLevel::Alert => ("[ALERT] ", Color::from_rgb8(255, 200, 25)),
//             NotificationLevel::Normal => ("[LOG] ", Color::from_rgb8(10, 255, 75)),
//         };
//
//         Row::new()
//             .align_y(Alignment::Start)
//             .width(Length::Fill)
//             .push(text(level).color(color))
//             .push(text(notif.mesg))
//             .into()
//     });
//     let notif_hist = Container::new(
//         Column::new()
//             // .spacing(10)
//             .align_x(Alignment::Start)
//             // .height(Length::Fill)
//             // .width(Length::FillPortion(20))
//             .extend(notifs.clone()),
//     )
//     .height(Length::Fill)
//     .width(Length::FillPortion(15))
//     .style(rounded_box);
//
//     let main = Container::new(
//         Column::new()
//             // .spacing(10)
//             .align_x(Alignment::Start)
//             // .height(Length::Fill)
//             // .width(Length::FillPortion(50))
//             .extend(notifs.clone()),
//     )
//     .height(Length::Fill)
//     .width(Length::FillPortion(50))
//     .style(rounded_box);
//
//     let gfx = Container::new(
//         Column::new()
//             // .spacing(10)
//             .align_x(Alignment::End)
//             // .height(Length::Fill)
//             // .width(Length::FillPortion(50))
//             .extend(notifs),
//     )
//     .height(Length::Fill)
//     .width(Length::FillPortion(35))
//     .style(rounded_box);
//
//     let row = Row::new()
//         .spacing(10)
//         .align_y(Alignment::Center)
//         .height(Length::Fill)
//         .width(Length::Fill)
//         .push(notif_hist)
//         .push(main)
//         .push(gfx);
//
//     ctx.display(row);
// }
