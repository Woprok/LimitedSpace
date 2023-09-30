mod debug_manager;
mod game_data;

use bevy::prelude::*;
use debug_manager::*;
use game_data::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DebugManagerPlugin
        ))
        .add_systems(Startup, add_tiles)
        .run();
}

fn add_tiles(mut commands: Commands) {
    for i in 0..IROWS {
        for j in 0..JCOLS {
            commands.spawn((
                Tile { index: i*JCOLS + j }, 
                Position { x: i, y: j }
            ));
        }
    }
}