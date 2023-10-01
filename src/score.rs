use crate::game_data::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreTag;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, setup_score)
            .add_systems(Update, update_score);
    }
}

fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut full_score = TextBundle::from_section(
        format!("Score: {}", 0.0),
        TextStyle {
            font: asset_server.load(DEFAULT_FONT),
            font_size: DEFAULT_FONT_SIZE,
            color: DEFAULT_FONT_COLOR,
        },
    );
    full_score.visibility = Visibility::Visible;
    commands.spawn((full_score, ScoreTag));
}

fn update_score(
    mut query: Query<&mut Text, With<ScoreTag>>,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    if score.is_changed() {
        *query.single_mut() = Text::from_section(
            format!("Score: {}", score.current),
            TextStyle {
                font: asset_server.load(DEFAULT_FONT),
                font_size: DEFAULT_FONT_SIZE,
                color: DEFAULT_FONT_COLOR,
            },
        );
    }
}
