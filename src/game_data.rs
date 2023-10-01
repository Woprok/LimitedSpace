use bevy::prelude::*;
use bevy::window::*;

pub struct GameDataPlugin;
impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_window_settings);
    }
}

// pub const IROWS: i32 = 4;
// pub const JCOLS: i32 = 4;
pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 640.0;

pub const PLAYER_SIZE: f32 = 32.0;
pub const ENEMY_SIZE: f32 = 32.0;
pub const ENEMY_SPAWN_FREQUENCY: f32 = 2.0;
pub const SPAWN_FREQUENCY_CHANGER: f32 = 8.0;
// Text constants
pub static DEFAULT_FONT: &str = "Roboto-Medium.ttf";
pub const DEFAULT_FONT_SIZE: f32 = 24.0;
pub const DEFAULT_FONT_COLOR: Color = Color::WHITE;
// Models
pub static CIRCLE_SPRITE: &str = "circle_player.png";
pub static SQUARE_SPRITE: &str = "square_player.png";
pub static TRIANGLE_SPRITE: &str = "triangle_player.png";
// Models
pub static ENEMY_CIRCLE_SPRITE: &str = "circle_enemy.png";
pub static ENEMY_SQUARE_SPRITE: &str = "square_enemy.png";
pub static ENEMY_TRIANGLE_SPRITE: &str = "triangle_enemy.png";

// #[derive(Component)]
// pub struct Position {
//     pub x: i32,
//     pub y: i32,
// }

// #[derive(Component)]
// pub struct Tile {
//     pub index: i32,
// }

#[derive(Component)]
pub struct Player {}

#[derive(Component, PartialEq, Eq)]
pub enum Shape {
    Circle,
    Square,
    Triangle,
}

// impl Shape {
//     pub fn iterate(&mut self) {
//         *self = match self {
//             Self::Circle => Self::Square,
//             Self::Square => Self::Triangle,
//             Self::Triangle => Self::Circle,
//         }
//     }
// }

#[derive(Component)]
pub struct Enemy {}

#[derive(Resource)]
pub struct MutableCounter {
    pub count: f32,
}
impl Default for MutableCounter {
    fn default() -> MutableCounter {
        MutableCounter {
            count: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

impl Default for SpawnTimer {
    fn default() -> SpawnTimer {
        SpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_FREQUENCY, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct ChangeTimer {
    pub timer: Timer,
}

impl Default for ChangeTimer {
    fn default() -> ChangeTimer {
        ChangeTimer {
            timer: Timer::from_seconds(SPAWN_FREQUENCY_CHANGER, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub size: f32,
    pub color: Color,
}

#[derive(Resource)]
pub struct Score {
    pub current: f32,
}

impl Default for Score {
    fn default() -> Score {
        Score { current: 0.0 }
    }
}

pub fn set_window_settings(mut query_window: Query<&mut Window>) {
    let mut window = query_window.single_mut();
    window.mode = WindowMode::Windowed;
    window.title = "Limited Shape in limited space by Woprok".into();
    window.resolution = (WINDOW_WIDTH, WINDOW_HEIGHT).into();
    window.window_level = WindowLevel::Normal;
    window.decorations = true;
    window.resizable = false;
    window.position = WindowPosition::Centered(MonitorSelection::Current);
    window.resize_constraints = WindowResizeConstraints {
        min_width: WINDOW_WIDTH,
        min_height: WINDOW_HEIGHT,
        max_width: WINDOW_WIDTH,
        max_height: WINDOW_HEIGHT,
    }
}
