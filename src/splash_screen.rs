use bevy::{
	input::{keyboard::KeyboardInput, ButtonState},
	prelude::*,
};

use crate::GameState;

#[derive(Component)]
pub struct SplashScreen;

pub struct SplashScreenPlugin;
impl Plugin for SplashScreenPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(Self::cleanup.in_schedule(OnExit(GameState::Menu)))
			.add_system(Self::setup.in_schedule(OnEnter(GameState::Menu)))
			.add_system(Self::toggle_splash);
	}
}

impl SplashScreenPlugin {
	pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
		commands
			.spawn((
				Name::new("Splash Screen"),
				SplashScreen,
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
					Name::new("Splash Screen Text"),
					TextBundle::from_section(
						"Press any key\n  to start",
						TextStyle {
							color: Color::WHITE,
							font_size: 100.0,
							font: asset_server.load("fonts/Pixelzim 3x5.ttf"),
						},
					),
				));
			});
	}

	pub fn cleanup(mut commands: Commands, mut query: Query<Entity, With<SplashScreen>>) {
		for entity in query.iter_mut() {
			commands.entity(entity).despawn_recursive();
		}
	}

	pub fn toggle_splash(
		state: Res<State<GameState>>,
		mut next_state: ResMut<NextState<GameState>>,
		mut keyboard_input_event_reader: EventReader<KeyboardInput>,
	) {
		for event in keyboard_input_event_reader.iter() {
			if state.0 == GameState::Menu && event.state == ButtonState::Pressed {
				next_state.set(GameState::Playing);
			}
		}
	}
}
