use bevy::{
	prelude::*,
	sprite::collide_aabb::{collide, Collision},
};
use bevy_turborand::*;
use std::{f32::consts::PI, ops::DerefMut};

use crate::{collider::Collider, paddle::Paddle, reset::Reset, score::Score, wall::Wall, GameSet};

const MAX_BOUNCE_ANGLE: f32 = 5.0 * PI / 12.0;
const BALL_SPEED: f32 = 400.0;

#[derive(Component, Debug)]
pub struct Ball {
	direction: Vec2,
	speed: f32,
}

pub enum CollisionEvent {
	Paddle,
	Wall,
	Goal,
}

impl Ball {
	pub fn velocity(&self) -> Vec2 {
		self.direction.normalize() * self.speed
	}
}

#[derive(Default)]
pub struct LastCollision(pub Option<Collision>);

pub struct BallPlugin;
impl Plugin for BallPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(Self::setup)
			.add_event::<CollisionEvent>()
			.add_systems(
				(
					Self::handle_reset.in_set(GameSet::Reset),
					Self::check_collision.in_set(GameSet::CollisionDetection),
					Self::update_position.in_set(GameSet::Movement),
				)
					.chain()
					.in_schedule(CoreSchedule::FixedUpdate),
			);
	}
}

impl BallPlugin {
	pub fn setup(mut commands: Commands, mut rng: ResMut<GlobalRng>) {
		let direction = Vec2::new(if rng.bool() { 1. } else { -1. }, if rng.bool() { 0.5 } else { -0.5 });
		commands.spawn((
			Name::new("Ball"),
			Ball {
				speed: BALL_SPEED,
				direction,
			},
			SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::new(Wall::WIDTH, Wall::WIDTH)),
					..default()
				},
				..default()
			},
		));
	}

	pub fn handle_reset(
		mut query: Query<(&mut Transform, &mut Ball)>,
		mut reset_reader: EventReader<Reset>,
		mut rng: ResMut<GlobalRng>,
	) {
		if reset_reader.iter().last().is_none() {
			return;
		}

		let speed = BALL_SPEED;

		for (mut transform, mut ball) in query.iter_mut() {
			ball.direction =
				Vec2::new(if rng.bool() { 1. } else { -1. }, if rng.bool() { 0.5 } else { -0.5 }).normalize();
			ball.speed = speed;
			transform.translation = Vec3::default();
		}
	}

	pub fn check_collision(
		mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
		collider_query: Query<(&Transform, &Sprite, Option<&Wall>, Option<&Paddle>), With<Collider>>,
		mut reset_writer: EventWriter<Reset>,
		mut score: ResMut<Score>,
		mut last_collision: Local<LastCollision>,
		mut collision_events: EventWriter<CollisionEvent>,
	) {
		let (mut ball, ball_transform, ball_sprite) = ball_query.single_mut();

		for (collider_transform, collider_sprite, wall, paddle) in &collider_query {
			if ball_sprite.custom_size.is_some() && collider_sprite.custom_size.is_some() {
				let collision = collide(
					ball_transform.translation,
					ball_sprite.custom_size.unwrap(),
					collider_transform.translation,
					collider_sprite.custom_size.unwrap(),
				);

				if collision.is_none() {
					continue;
				}

				if last_collision.0 == collision {
					debug!("Duplicate Collision: {:?}", collision);
					continue;
				} else {
					debug!("Normal Collision: {:?}", collision);
					last_collision.0 = match collision {
						Some(Collision::Top) => Some(Collision::Top),
						Some(Collision::Bottom) => Some(Collision::Bottom),
						Some(Collision::Left) => Some(Collision::Left),
						Some(Collision::Right) => Some(Collision::Right),
						Some(Collision::Inside) => Some(Collision::Inside),
						None => None,
					};
				}

				if paddle.is_some() {
					debug!("Paddle collision: {:?}", collision);

					let ball_position = ball_transform.translation;
					let paddle_position = collider_transform.translation;

					let ball_angle = Self::novel_reflect(&paddle_position, &ball_position);
					debug!("Ball angle: {:?}", ball_angle);

					ball.direction = match collision {
						Some(Collision::Right) => Vec2::new(ball_angle.x, -ball_angle.y),
						Some(Collision::Left) => Vec2::new(-ball_angle.x, -ball_angle.y),
						_ => Vec2::new(-ball.direction.x, -ball.direction.y),
					};

					collision_events.send(CollisionEvent::Paddle);
				} else {
					let (reflect_x, reflect_y) = match collision.unwrap() {
						Collision::Top => (false, ball.direction.y < 0.),
						Collision::Right => (ball.direction.x < 0., false),
						Collision::Left => (ball.direction.x > 0., false),
						Collision::Bottom => (false, ball.direction.y > 0.),
						Collision::Inside => (false, false),
					};

					if reflect_x {
						ball.direction.x = -ball.direction.x;
					}

					if reflect_y {
						ball.direction.y = -ball.direction.y;
					}
				}

				if let Some(wall) = wall {
					match wall {
						Wall::Top | Wall::Bottom => collision_events.send(CollisionEvent::Wall),
						Wall::Right => {
							score.deref_mut().left += 1;
							ball.speed = 0.;
							collision_events.send(CollisionEvent::Goal);
							reset_writer.send(Reset::Soft);
							continue;
						}
						Wall::Left => {
							score.deref_mut().right += 1;
							ball.speed = 0.;
							collision_events.send(CollisionEvent::Goal);
							reset_writer.send(Reset::Soft);
							continue;
						}
					}
				}
			}
		}
	}

	pub fn update_position(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
		for (ball, mut transform) in query.iter_mut() {
			transform.translation += time.delta_seconds() * ball.velocity().extend(0.);
		}
	}

	pub fn novel_reflect(paddle_pos: &Vec3, hit_pos: &Vec3) -> Vec2 {
		let relative_ball_pos = *hit_pos - *paddle_pos;
		debug!("Relative hit pos: {:?}", relative_ball_pos);

		let normalized_relative_ball_pos = relative_ball_pos / Paddle::HEIGHT;

		debug!("Normalized relative hit pos: {:?}", normalized_relative_ball_pos);

		let bounce_angle = normalized_relative_ball_pos * MAX_BOUNCE_ANGLE;
		debug!("Bounce angle: {:?} (max: {:?}", bounce_angle, MAX_BOUNCE_ANGLE);

		let ball_angle = Vec2::new(f32::cos(bounce_angle.x), -f32::sin(bounce_angle.y)).normalize();

		debug!("Ball angle: {:?}", ball_angle);

		ball_angle
	}
}
