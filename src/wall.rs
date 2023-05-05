use bevy::prelude::*;

use crate::{collider::Collider, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Component, Debug, Eq, PartialEq)]
pub enum Wall {
	Top,
	Bottom,
	Right,
	Left,
}

impl Wall {
	pub const WIDTH: f32 = 20.;
}

pub struct WallPlugin;
impl Plugin for WallPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(Self::setup);
	}
}

impl WallPlugin {
	pub fn setup(mut commands: Commands) {
		Self::spawn_wall(Wall::Top, &mut commands);
		Self::spawn_wall(Wall::Right, &mut commands);
		Self::spawn_wall(Wall::Bottom, &mut commands);
		Self::spawn_wall(Wall::Left, &mut commands);
	}

	fn spawn_wall(wall: Wall, commands: &mut Commands) {
		let wall_y = (WINDOW_HEIGHT - Wall::WIDTH) / 2.;
		let wall_x = (WINDOW_WIDTH - Wall::WIDTH) / 2.;
		let translation = match wall {
			Wall::Top => Vec3::new(0., wall_y, 0.),
			Wall::Bottom => Vec3::new(0., -wall_y, 0.),
			Wall::Right => Vec3::new(wall_x, 0., 0.),
			Wall::Left => Vec3::new(-wall_x, 0., 0.),
		};

		let custom_size = match wall {
			Wall::Top | Wall::Bottom => Vec2::new(WINDOW_WIDTH, Wall::WIDTH),
			Wall::Right | Wall::Left => Vec2::new(Wall::WIDTH, WINDOW_HEIGHT),
		};

		commands.spawn((
			Name::new(format!("Wall {:?}", wall)),
			Collider,
			SpriteBundle {
				sprite: Sprite {
					custom_size: Some(custom_size),
					..default()
				},
				transform: Transform::from_translation(translation),
				..default()
			},
			wall,
		));
	}
}
