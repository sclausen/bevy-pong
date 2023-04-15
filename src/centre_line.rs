use bevy::prelude::*;

use crate::{reset::Reset, wall::Wall, GameSet, WINDOW_HEIGHT};

#[derive(Component)]
pub struct CentreLine;

pub struct CentreLinePlugin;

impl Plugin for CentreLinePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(Self::setup)
			.add_system(Self::handle_reset.in_set(GameSet::Reset));
	}
}

impl CentreLinePlugin {
	pub fn setup(mut commands: Commands) {
		commands
			.spawn((CentreLine, Name::new("Centre Line"), SpatialBundle::default()))
			.with_children(|commands| {
				for i in 0..(WINDOW_HEIGHT / Wall::WIDTH) as i32 {
					commands.spawn((SpriteBundle {
						transform: Transform {
							translation: Vec3::new(0.0, i as f32 * Wall::WIDTH * 2.0 - WINDOW_HEIGHT / 2.0, 0.0),
							..default()
						},
						sprite: Sprite {
							custom_size: Some(Vec2::new(Wall::WIDTH, Wall::WIDTH)),
							..default()
						},
						..default()
					},));
				}
			});
	}

	pub fn handle_reset(mut query: Query<&mut Transform, With<CentreLine>>, mut reset_reader: EventReader<Reset>) {
		if reset_reader.iter().last().is_none() {
			return;
		}

		for mut transform in query.iter_mut() {
			transform.translation = Vec3::new(0.0, transform.translation.y, 0.0);
		}
	}
}
