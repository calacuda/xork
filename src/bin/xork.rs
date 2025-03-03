use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_common_assets::ron::RonAssetPlugin;
use ratatui::crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::{error::Error, fs::read_dir, io, path::PathBuf};
use xork::{
    CommandEntered, CommandResultEvent, PlayerLook, PlayerMovment,
    mobs::{MobAsset, Mobs},
    state::{BattleWith, GameState},
    tui::{ClientTui, Term, tui_update},
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
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::INFO,
                ..default()
            }),
            RonAssetPlugin::<ZoneAsset>::new(&["zone.ron"]),
            RonAssetPlugin::<MobAsset>::new(&["mob.ron"]),
        ))
        .insert_resource(Zones::default())
        .insert_resource(Mobs::default())
        .configure_sets(Update, Adventure.run_if(in_state(GameState::Adventure)))
        .configure_sets(Update, InGame.run_if(not(in_state(GameState::Startup))))
        .init_resource::<ClientTui>()
        .init_resource::<Zone>()
        .insert_resource(Term(terminal))
        .init_state::<GameState>()
        .init_state::<BattleWith>()
        .add_event::<CommandEntered>()
        // .add_event::<SystemCmd>()
        // .add_event::<PlayerGameCommand>()
        .add_event::<PlayerMovment>()
        .add_event::<PlayerLook>()
        .add_event::<CommandResultEvent>()
        .init_asset::<ZoneAsset>()
        .init_asset::<MobAsset>()
        .add_systems(Startup, (load_zone_assets, load_mob_assets))
        // .add_systems(
        //     Update,
        //     (
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
        .add_systems(Update, (tui_update).in_set(InGame))
        // .set_runner(minimal_runner)
        .run();

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

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
    let to_assests = PathBuf::from("assets/mobs");

    read_dir(to_assests).unwrap().for_each(|asset| {
        // if let Ok(asset_dir) = asset_dir {
        // read_dir(asset_dir.path()).unwrap().for_each(|zone| {
        if let Ok(asset) = asset {
            // let sub_path = format!(
            // "{}",
            // asset.file_name().to_str().unwrap()
            // );
            let path = format!("mobs/{}", asset.file_name().to_str().unwrap());
            info!("{path}");
            mobs.0.insert(path.clone(), asset_server.load(path));
        }
        // })
        // }
    });
}
