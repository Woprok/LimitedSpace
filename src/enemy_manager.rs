use crate::game_data::*;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const SIZE_EXPANSION: f32 = 0.2;
const TIME_REDUCE: f32 = 0.2;
const TIME_MIN: f32 = 0.8;


pub struct EnemyManagerPlugin;
impl Plugin for EnemyManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>()
            .init_resource::<ChangeTimer>()
            .init_resource::<MutableCounter>()
            .add_systems(Update, spawn_projectile)
            .add_systems(Update, check_player_collision)
            .add_systems(Update, check_projectile_collision)
            .add_systems(Update, update_change_timer)
            .add_systems(Update, spawn_enemy);
    }
}

fn update_change_timer(time: Res<Time>, mut timer: ResMut<ChangeTimer>, mut spawner: ResMut<SpawnTimer>, mut commands: Commands, mut times: ResMut<MutableCounter>) {
    timer.timer.tick(time.delta());
    spawner.timer.tick(time.delta());
    if timer.timer.just_finished() {
        let leftover = spawner.timer.elapsed();
        times.count += 1.0;
        let mut new_timer = SpawnTimer {
            timer: Timer::from_seconds(f32::min(ENEMY_SPAWN_FREQUENCY - TIME_REDUCE * times.count, TIME_MIN), TimerMode::Repeating),
        };
        new_timer.timer.tick(leftover);
        commands.insert_resource(new_timer)
    }
}

fn spawn_enemy(
    commands: Commands,
    wq: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    spawn_timer: ResMut<SpawnTimer>,
    pc_transform: Query<&Transform, With<Player>>,
) {
    if let Ok(transform) = pc_transform.get_single() {
        if spawn_timer.timer.finished() {
            let rand_pos = loop {
                let candidate_pos = get_random_position(wq.get_single().unwrap());
                let dist = transform.translation.distance(candidate_pos);
                if dist > ENEMY_SIZE / 2.0 + PLAYER_SIZE / 2.0 {
                    break candidate_pos;
                }
            };

            let choices = [Shape::Circle, Shape::Square, Shape::Triangle];
            match choices.choose(&mut rand::thread_rng()).unwrap() {
                Shape::Circle => spawn_circle(commands, asset_server, rand_pos),
                Shape::Square => spawn_square(commands, asset_server, rand_pos),
                Shape::Triangle => spawn_triangle(commands, asset_server, rand_pos),
            }
        }
    }
}

fn spawn_circle(mut commands: Commands, asset_server: Res<AssetServer>, rand_pos: Vec3) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(ENEMY_CIRCLE_SPRITE),
            transform: Transform::from_xyz(rand_pos.x, rand_pos.y, 0.0),
            ..default()
        },
        Enemy {},
        Projectile {
            size: 0.0,
            color: Color::BLACK,
        },
        Shape::Circle,
    ));
}

fn spawn_square(mut commands: Commands, asset_server: Res<AssetServer>, rand_pos: Vec3) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(ENEMY_SQUARE_SPRITE),
            transform: Transform::from_xyz(rand_pos.x, rand_pos.y, 0.0),
            ..default()
        },
        Enemy {},
        Projectile {
            size: 0.0,
            color: Color::BLACK,
        },
        Shape::Square,
    ));
}

fn spawn_triangle(mut commands: Commands, asset_server: Res<AssetServer>, rand_pos: Vec3) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(ENEMY_TRIANGLE_SPRITE),
            transform: Transform::from_xyz(rand_pos.x, rand_pos.y, 0.0),
            ..default()
        },
        Enemy {},
        Projectile {
            size: 0.0,
            color: Color::BLACK,
        },
        Shape::Triangle,
    ));
}

fn calculate_equilateral_triangle_vertices(center: Vec2, side_length: f32) -> [Vec2; 3] {
    let angle_offset = 2.0 * std::f32::consts::PI / 3.0; // 120 degrees in radians
    let mut vertices = [Vec2::new(0.0, 0.0); 3];

    for i in 0..3 {
        let angle = angle_offset * i as f32 + std::f32::consts::PI / 2.0;
        let x = center.x + side_length * angle.cos();
        let y = center.y + side_length * angle.sin();
        vertices[i] = Vec2::new(x, y);
    }

    vertices
}

fn spawn_projectile(
    mut gizmos: Gizmos,
    mut query: Query<(&mut Projectile, &mut Transform, &mut Shape), With<Enemy>>,
) {
    query.for_each_mut(|(mut projectile, transform, shape)| {
        projectile.size += SIZE_EXPANSION;

        match *shape {
            Shape::Circle => {
                gizmos.circle_2d(
                    Vec2 {
                        x: transform.translation.x,
                        y: transform.translation.y,
                    },
                    projectile.size,
                    projectile.color,
                );
            }
            Shape::Square => {
                gizmos.rect_2d(
                    Vec2 {
                        x: transform.translation.x,
                        y: transform.translation.y,
                    },
                    0.0,
                    Vec2 {
                        x: projectile.size,
                        y: projectile.size,
                    },
                    projectile.color,
                );
            }
            Shape::Triangle => {
                let vecs = calculate_equilateral_triangle_vertices(
                    Vec2::new(transform.translation.x, transform.translation.y),
                    projectile.size,
                );
                gizmos.linestrip_2d([vecs[0], vecs[1], vecs[2], vecs[0]], projectile.color);
            }
        }
    });
}

fn get_random_position(window: &Window) -> Vec3 {
    // note this does not work on startup, as width and height change only after that...
    let width = f32::min(window.width(), WINDOW_WIDTH);
    let height = f32::min(window.height(), WINDOW_HEIGHT);
    let x_min = -(width - PLAYER_SIZE) / 2.0;
    let y_min = -(height - PLAYER_SIZE) / 2.0;
    let random_x = x_min + random::<f32>() * (width - PLAYER_SIZE);
    let random_y = y_min + random::<f32>() * (height - PLAYER_SIZE);
    Vec3::new(random_x, random_y, 0.0)
}

fn check_player_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    mut score: ResMut<Score>,
) {
    if let Ok(pc_tranform) = player_query.get_single() {
        enemy_query.for_each(|(e_transform, e_entity)| {
            let distance = pc_tranform.translation.distance(e_transform.translation);
            if distance < ENEMY_SIZE / 2.0 + PLAYER_SIZE / 2.0 {
                println!("Player collided with enemy.");
                commands.entity(e_entity).despawn();
                score.current += 1.0;
            }
        });
    }
}

fn sdf_square(pc: Vec3, tar: Vec3, side_size: f32) -> f32 {
    let rad = side_size / 2.0;
    let dis = Vec3::abs(pc - tar);
    if dis.x > rad && dis.y > rad {
        return f32::sqrt(f32::powi(dis.x - rad, 2) + f32::powi(dis.y - rad, 2));
    }
    if dis.x > rad || dis.y > rad {
        return f32::max(dis.x - rad, dis.y - rad);
    } else {
        return -f32::max(rad - dis.x, rad - dis.y);
    }
}

fn sdf_triangle(pos: Vec3, r: f32) -> f32 {
    let mut p = pos;
    let k = f32::sqrt(3.0);
    p.x = f32::abs(p.x) - r;
    p.y = p.y + r / k;
    if p.x + k * p.y > 0.0 {
        p = Vec3 {
            x: p.x - k * p.y,
            y: -k * p.x - p.y,
            z: 0.0,
        } / 2.0;
    }
    p.x -= f32::clamp(p.x, -2.0 * r, 0.0);
    return -Vec3::length(p) * f32::signum(p.y);
}

fn check_projectile_collision(
    mut commands: Commands,
    player_query: Query<(&Transform, &Shape, Entity), With<Player>>,
    enemies_query: Query<(&Transform, &Projectile, &Shape), With<Enemy>>,
) {
    if let Ok((pc_transform, pc_shape, pc_entity)) = player_query.get_single() {
        enemies_query.for_each(|(e_transform, e_projectile, e_shape)| {
            if pc_shape != e_shape {
                match *e_shape {
                    Shape::Circle => {
                        let player_distance =
                            pc_transform.translation.distance(e_transform.translation);
                        if player_distance > e_projectile.size - PLAYER_SIZE / 2.0
                            && player_distance < e_projectile.size + PLAYER_SIZE / 2.0
                        {
                            commands.entity(pc_entity).despawn();
                        }
                    }
                    Shape::Square => {
                        let sdf = sdf_square(
                            pc_transform.translation,
                            e_transform.translation,
                            e_projectile.size,
                        );
                        if f32::abs(sdf) <= PLAYER_SIZE / 2.0 {
                            commands.entity(pc_entity).despawn();
                        }
                    }
                    Shape::Triangle => {
                        let dif_pos = pc_transform.translation - e_transform.translation;
                        let sdf = sdf_triangle(dif_pos, e_projectile.size);
                        if f32::abs(sdf) <= PLAYER_SIZE / 2.0 {
                            commands.entity(pc_entity).despawn();
                        }
                    }
                }
            }
        });
    }
}
