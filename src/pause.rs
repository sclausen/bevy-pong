use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct Pause;

pub struct PausePlugin;
impl Plugin for PausePlugin {
	fn build(&self, app: &mut App) {
		app.add_system(Self::setup.in_schedule(OnEnter(GameState::Paused)))
			.add_system(Self::cleanup.in_schedule(OnExit(GameState::Paused)))
			.add_system(Self::toggle_pause);
	}
}

impl PausePlugin {
	pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
		commands
			.spawn((
				Name::new("Pause Screen"),
				Pause,
				NodeBundle {
					background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.9)),
					style: Style {
						align_items: AlignItems::Center,
						justify_content: JustifyContent::Center,
						position_type: PositionType::Absolute,
						size: Size::new(Val::Percent(100.), Val::Percent(100.)),
						..default()
					},
					..default()
				},
			))
			.with_children(|commands| {
				commands.spawn((
					Name::new("Pause Screen Text"),
					TextBundle::from_section(
						"Pause",
						TextStyle {
							color: Color::WHITE,
							font_size: 100.0,
							font: asset_server.load("fonts/Pixelzim 3x5.ttf"),
						},
					),
				));
			});
	}

	pub fn cleanup(mut commands: Commands, mut query: Query<Entity, With<Pause>>) {
		for entity in query.iter_mut() {
			commands.entity(entity).despawn_recursive();
		}
	}

	pub fn toggle_pause(
		state: Res<State<GameState>>,
		mut next_state: ResMut<NextState<GameState>>,
		keyboard_input: Res<Input<KeyCode>>,
	) {
		if keyboard_input.just_pressed(KeyCode::P) {
			match state.0 {
				GameState::Playing => {
					next_state.set(GameState::Paused);
				}
				GameState::Paused => {
					next_state.set(GameState::Playing);
				}
				_ => {
					debug!("Cannot pause in state {:?}", state.0);
				}
			}
		}
	}
}
