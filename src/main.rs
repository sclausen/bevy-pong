use bevy::{
	log::LogPlugin,
	prelude::*,
	window::{WindowLevel, WindowResolution},
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_pong::{PongPlugin, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
	let mut app = App::new();

	app.add_plugins(
		DefaultPlugins
			.set(LogPlugin {
				filter: "bevy_pong=debug,bevy=debug".into(),
				level: bevy::log::Level::WARN,
			})
			.set(AssetPlugin {
				watch_for_changes: true,
				..default()
			})
			.set(WindowPlugin {
				primary_window: Some(Window {
					canvas: Some("#bevy".to_owned()),
					resizable: false,
					resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
					title: "pong".to_string(),
					present_mode: bevy::window::PresentMode::Fifo,
					window_level: WindowLevel::AlwaysOnTop,
					..default()
				}),
				..default()
			}),
	)
	.add_plugin(PongPlugin)
	.add_system(bevy::window::close_on_esc);

	#[cfg(feature = "debug")]
	app.add_plugin(WorldInspectorPlugin::new());

	app.run();
}
