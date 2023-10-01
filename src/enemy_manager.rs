use bevy::{prelude::*, window::PrimaryWindow};
use crate::{game_data::*};
use rand::prelude::*;

const SIZE_EXPANSION: f32 = 0.1;

pub struct EnemyManagerPlugin;
impl Plugin for EnemyManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_enemy)
            .add_systems(Update, spawn_projectile)
            .add_systems(Update, check_player_collision);
    }
}

fn spawn_enemy(mut commands: Commands, wq: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let rand_pos = get_random_position(wq.get_single().unwrap());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cursor.png"),
            transform: Transform::from_xyz(rand_pos.0,rand_pos.1,0.0),
            ..default()
        },
        Enemy {},
        Projectile { size: 0.0, color: Color::BLACK },   
    ));
}

fn spawn_projectile(mut gizmos: Gizmos, mut query: Query<(&mut Projectile, &mut Transform), With<Enemy>>) {
    query.for_each_mut(|mut pt| {
        pt.0.size += SIZE_EXPANSION;
        gizmos.circle_2d(Vec2 {
            x: pt.1.translation.x, 
            y: pt.1.translation.y}, 
            pt.0.size, 
            pt.0.color
        );
    });
}

fn get_random_position(window: &Window) -> (f32,f32) {
    let random_x = window.width() / 2.0 - random::<f32>() * window.width();
    let random_y = window.height() / 2.0 - random::<f32>() * window.height();
    (random_x, random_y)
}

fn check_player_collision(mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(&Transform, &Enemy, Entity)>,
    mut score: ResMut<Score>    
) {
    let player = player_query.single();
    enemy_query.for_each(|enemy| {
        let distance = player.translation.distance(enemy.0.translation); 
        if distance < ENEMY_SIZE / 2.0 + PLAYER_SIZE / 2.0
        {
            println!("Player collided with enemy.");
            commands.entity(enemy.2).despawn();
            score.current += 1.0;
        }
    });
}

fn check_projectile_collision() {

}