use bevy::prelude::*;

use crate::{
	collider::Collider, player::Player, reset::Reset, wall::Wall, GameSet, PongPlugin, WINDOW_HEIGHT, WINDOW_WIDTH,
};

#[derive(Debug, Default, Component)]
pub struct Paddle {
	pub speed: f32,
	pub velocity: Vec2,
}

impl Paddle {
	pub const WIDTH: f32 = Wall::WIDTH;
	pub const HEIGHT: f32 = WINDOW_HEIGHT * 0.2;
	pub const MARGIN: f32 = Wall::WIDTH * 2.;

	pub fn new(speed: f32, velocity: Vec2) -> Self {
		Self { speed, velocity }
	}
}

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(Self::setup)
			.add_system(Self::handle_reset.in_set(GameSet::Reset))
			.add_system(
				Self::handle_input
					.in_set(GameSet::Movement)
					.run_if(PongPlugin::is_playing),
			)
			.add_system(Self::update_position.in_set(GameSet::Movement));
	}
}

impl PaddlePlugin {
	pub fn setup(mut commands: Commands) {
		commands.spawn((
			Name::new("Paddle Left"),
			Collider,
			Paddle::new(WINDOW_HEIGHT * 0.5, Vec2::default()),
			Player::Left,
			SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::new(Paddle::WIDTH, Paddle::HEIGHT)),
					..default()
				},
				transform: Transform::from_translation(Vec3::new(
					Paddle::MARGIN - (WINDOW_WIDTH / 2.) + Wall::WIDTH / 2.,
					0.,
					0.,
				)),
				..default()
			},
		));
		commands.spawn((
			Name::new("Paddle Right"),
			Collider,
			Paddle::new(WINDOW_HEIGHT * 0.5, Vec2::default()),
			Player::Right,
			SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::new(Paddle::WIDTH, Paddle::HEIGHT)),
					..default()
				},
				transform: Transform::from_translation(Vec3::new(
					(WINDOW_WIDTH / 2.) - Paddle::MARGIN - Wall::WIDTH / 2.,
					0.,
					0.,
				)),
				..default()
			},
		));
	}

	pub fn handle_reset(
		mut paddle_query: Query<(&mut Sprite, &mut Transform, &mut Paddle, &Player)>,
		mut reset_reader: EventReader<Reset>,
	) {
		if let Some(Reset::Hard) = reset_reader.iter().last() {
			for (mut sprite, mut transform, mut paddle, &player) in paddle_query.iter_mut() {
				paddle.speed = WINDOW_HEIGHT / 3.;

				sprite.custom_size = Some(Vec2::new(Paddle::WIDTH, Paddle::HEIGHT));

				let translation_x = match player {
					Player::Left => Paddle::MARGIN - (WINDOW_WIDTH / 2.) + Wall::WIDTH / 2.,
					Player::Right => (WINDOW_WIDTH / 2.) - Paddle::MARGIN - Wall::WIDTH / 2.,
				};

				transform.translation = Vec3::new(translation_x, 0., 0.);
			}
		}
	}

	pub fn handle_input(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Paddle, &Player)>) {
		debug!("Handling input for paddles...");
		for (mut paddle, player) in query.iter_mut() {
			let (up_keycode, down_keycode) = player.movement_keys();

			if keys.pressed(up_keycode) {
				paddle.velocity.y = paddle.speed;
			} else if keys.pressed(down_keycode) {
				paddle.velocity.y = -paddle.speed;
			} else {
				paddle.velocity.y = 0.;
			}
		}
	}

	pub fn update_position(mut query: Query<(&Paddle, &mut Transform)>, time: Res<Time>) {
		//  debug!("Updating paddle positions...");
		let delta_seconds = time.delta_seconds();

		for (paddle, mut transform) in query.iter_mut() {
			// debug!(
			//     "Setting paddle ({:?}) y position to {},{}",
			//     paddle, transform.translation.x, transform.translation.y
			// );

			transform.translation.y = (transform.translation.y + delta_seconds * paddle.velocity.y)
				.max(-WINDOW_HEIGHT * 0.5 + WINDOW_HEIGHT * 0.1 + Wall::WIDTH)
				.min(WINDOW_HEIGHT * 0.5 - WINDOW_HEIGHT * 0.1 - Wall::WIDTH);
		}
	}
}
