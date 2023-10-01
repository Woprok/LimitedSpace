use crate::game_data::*;
use bevy::prelude::*;

static SHOW_HELP_TEXT: &str = "H - show help";
static FULL_HELP_TEXT: &str = "WASD - movement
I - transform to circle
J - transform to square
L - transform to triangle
H - hide help
Change shape to avoid shape attacks. 
Eat shapes to earn score.
Restart the game to play again :)";
const HELP_HOTKEY: KeyCode = KeyCode::H;

#[derive(Component)]
pub struct HelpTag;

trait Toggle {
    fn toggle(&mut self);
}

impl Toggle for Visibility {
    fn toggle(&mut self) {
        *self = match self {
            Self::Visible => Self::Hidden,
            Self::Hidden => Self::Visible,
            Self::Inherited => Self::Inherited,
        }
    }
}

pub struct HelpPlugin;
impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_help)
            .add_systems(Update, toggle_help);
    }
}

fn create_help(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut show_help = TextBundle::from_section(
        SHOW_HELP_TEXT,
        TextStyle {
            font: asset_server.load(DEFAULT_FONT),
            font_size: DEFAULT_FONT_SIZE,
            color: DEFAULT_FONT_COLOR,
        },
    );
    show_help.visibility = Visibility::Visible;

    let mut full_help = TextBundle::from_section(
        FULL_HELP_TEXT,
        TextStyle {
            font: asset_server.load(DEFAULT_FONT),
            font_size: DEFAULT_FONT_SIZE,
            color: DEFAULT_FONT_COLOR,
        },
    );
    full_help.visibility = Visibility::Hidden;
    commands.spawn((show_help, HelpTag));
    commands.spawn((full_help, HelpTag));
}

fn toggle_help(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, (With<Text>, With<HelpTag>)>,
) {
    if !keyboard_input.just_released(HELP_HOTKEY) {
        return;
    }
    query.for_each_mut(|mut visibility| {
        visibility.toggle();
    });
}
