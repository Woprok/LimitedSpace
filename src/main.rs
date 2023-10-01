#![windows_subsystem = "windows"]

mod debug_manager;
mod enemy_manager;
mod game_data;
mod help;
mod player_manager;
mod score;

use bevy::prelude::*;
// use debug_manager::*;
use enemy_manager::*;
use game_data::*;
use help::*;
use player_manager::*;
use score::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GameDataPlugin,
            // DebugManagerPlugin,
            PlayerManagerPlugin,
            EnemyManagerPlugin,
            HelpPlugin,
            ScorePlugin,
        ))
        //.add_systems(Startup, add_tiles)
        .add_systems(Startup, spawn_camera)
        .run();
}

// fn add_tiles(mut commands: Commands) {
//     for i in 0..IROWS {
//         for j in 0..JCOLS {
//             commands.spawn((
//                 Tile {
//                     index: i * JCOLS + j,
//                 },
//                 Position { x: i, y: j },
//             ));
//         }
//     }
// }

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
