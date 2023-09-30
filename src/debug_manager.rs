use bevy::prelude::*;
use crate::game_data::*;

pub struct DebugManagerPlugin;
impl Plugin for DebugManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DebugManager{
            timer: Timer::from_seconds(5.0, TimerMode::Once)
        })
        .add_systems(Startup, startup_message)
        .add_systems(Startup, print_tiles_system)
        .add_systems(Update, print_tile_with_position_system);
    }
}

#[derive(Resource)]
struct DebugManager {
    timer: Timer
}

fn startup_message() {
    println!("Initializing Limited Space prototype!");
}

fn print_tiles_system(query: Query<&Tile>) {
    for tile in &query {
        println!("Tile: {}", tile.index);
    }
}

fn print_tile_with_position_system(time: Res<Time>, mut debug_manager: ResMut<DebugManager>, query: Query<(&Tile, &Position)>) {
    if !debug_manager.timer.tick(time.delta()).just_finished() {
        return;
    }

    for (tile, position) in &query {
        println!("Tile: {} [{}, {}]", tile.index, position.x, position.y);
    }
}