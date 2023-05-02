#[cfg(feature = "debug")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use bevy_turborand::RngPlugin;

mod ai_paddle;
mod ball;
mod centre_line;
mod collider;
mod game_menu;
mod paddle;
mod pause;
mod player;
mod reset;
mod score;
mod sfxr_audio;
mod splash_screen;
mod wall;

use ai_paddle::AiPaddlePlugin;
use ball::BallPlugin;
use centre_line::CentreLinePlugin;
use paddle::PaddlePlugin;
use pause::PausePlugin;
use reset::ResetPlugin;
use score::ScorePlugin;
use sfxr_audio::SfxrAudioPlugin;
use splash_screen::SplashScreenPlugin;
use wall::WallPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
	#[default]
	Menu,
	Playing,
	Paused,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum GameSet {
	Input,
	Movement,
	CollisionDetection,
	Reset,
}

pub const WINDOW_HEIGHT: f32 = 720.;
pub const WINDOW_WIDTH: f32 = 1280.;
const TIME_STEP: f32 = 1.0 / 60.0;

pub struct PongPlugin;
impl Plugin for PongPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(Self::setup)
			.add_state::<GameState>()
			.add_plugin(RngPlugin::default())
			.add_plugin(ResetPlugin)
			//.add_plugin(SfxrAudioPlugin)
			.add_plugin(CentreLinePlugin)
			.add_plugin(BallPlugin)
			.add_plugin(PaddlePlugin)
			//.add_plugin(AiPaddlePlugin)
			.add_plugin(PausePlugin)
			.add_plugin(ScorePlugin)
			.add_plugin(SplashScreenPlugin)
			.add_plugin(WallPlugin)
			.configure_set(GameSet::Input.before(GameSet::Movement))
			.configure_set(GameSet::CollisionDetection.run_if(Self::in_menu_or_playing))
			.configure_set(GameSet::Movement.after(GameSet::CollisionDetection).after(GameSet::Input))
			.configure_set(GameSet::Reset.after(GameSet::CollisionDetection))
			.insert_resource(FixedTime::new_from_secs(TIME_STEP))
			.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));

		#[cfg(feature = "debug")]
		app.add_plugin(FrameTimeDiagnosticsPlugin::default())
			.add_plugin(LogDiagnosticsPlugin::default())
	}
}

impl PongPlugin {
	fn setup(mut commands: Commands) {
		commands.spawn(Camera2dBundle::default());
	}

	pub fn is_playing(state: Res<State<GameState>>) -> bool {
		state.0 == GameState::Playing
	}

	pub fn in_menu(state: Res<State<GameState>>) -> bool {
		state.0 == GameState::Menu
	}

	pub fn in_menu_or_playing(state: Res<State<GameState>>) -> bool {
		state.0 == GameState::Menu || state.0 == GameState::Playing
	}
}
