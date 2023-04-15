//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

use bevy::prelude::*;

use self::{game::GamePlugin, menu::MenuPlugin, splash::SplashPlugin};

mod game;
mod menu;
mod splash;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
	#[default]
	Splash,
	Menu,
	Game,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
	Low,
	Medium,
	High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(u32);

pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(DisplayQuality::Medium)
			.insert_resource(Volume(7))
			.add_startup_system(setup)
			.add_state::<GameState>()
			.add_plugin(SplashPlugin)
			.add_plugin(MenuPlugin)
			.add_plugin(GamePlugin);
	}
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}
