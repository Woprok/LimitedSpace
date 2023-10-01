use crate::game_data::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct PlayerManagerPlugin;
impl Plugin for PlayerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, process_movement_input)
            .add_systems(Update, process_change_input)
            .add_systems(Update, sprite_swap)
            .add_systems(Update, restrict_player_movement);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(CIRCLE_SPRITE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Shape::Circle,
        Player {},
    ));
}

fn process_change_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_quary: Query<&mut Shape, With<Player>>,
) {
    if let Ok(mut player_shape) = player_quary.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::J) {
            println!("Changing shape");
            *player_shape = Shape::Circle;
        }
        if keyboard_input.just_pressed(KeyCode::K) {
            println!("Changing shape");
            *player_shape = Shape::Square;
        }
        if keyboard_input.just_pressed(KeyCode::L) {
            println!("Changing shape");
            *player_shape = Shape::Triangle;
        }
    }
}

fn sprite_swap(
    mut player_query: Query<(&mut Handle<Image>, &mut Shape), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut sprite, shape)) = player_query.get_single_mut() {
        match *shape {
            Shape::Circle => *sprite = asset_server.load(CIRCLE_SPRITE),
            Shape::Square => *sprite = asset_server.load(SQUARE_SPRITE),
            Shape::Triangle => *sprite = asset_server.load(TRIANGLE_SPRITE),
        }
    }
}

fn process_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * 200.0 * time.delta_seconds();
    }
}

fn restrict_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = -(window.width() - PLAYER_SIZE) / 2.0;
        let y_min = -(window.height() - PLAYER_SIZE) / 2.0;
        let x_max = (window.width() - PLAYER_SIZE) / 2.0;
        let y_max = (window.height() - PLAYER_SIZE) / 2.0;
        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        player_transform.translation = translation;
    }
}
