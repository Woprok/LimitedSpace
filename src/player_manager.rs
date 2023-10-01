use bevy::prelude::*;
use crate::game_data::*;

pub struct PlayerManagerPlugin;
impl Plugin for PlayerManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, process_movement_input)
            .add_systems(Update, process_change_input);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cursor.png"),
            transform: Transform::from_xyz(0.0,0.0,0.0),
            ..default()
        },
        Shape::Circle,
        Player {},   
    ));
}

fn process_change_input(keyboard_input: Res<Input<KeyCode>>, mut player_quary: Query<&mut Shape, With<Player>>) {
    if let Ok(mut player_shape) = player_quary.get_single_mut() {
        if keyboard_input.just_released(KeyCode::Space) {
            println!("Changing shape");
            player_shape.iterate();
        }
    }
}

fn process_movement_input(keyboard_input: Res<Input<KeyCode>>, mut player_quary: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    if let Ok(mut transform) = player_quary.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * 200.0 * time.delta_seconds();
    }
}