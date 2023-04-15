use std::time::Duration;

use bevy::prelude::*;

use crate::{ball::Ball, paddle::Paddle, player::Player, GameSet, PongPlugin, WINDOW_WIDTH};

pub struct AiPaddlePlugin;
impl Plugin for AiPaddlePlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.add_startup_system(Self::setup)
			.add_system(Self::debug_print.run_if(PongPlugin::in_menu))
			.add_system(
				Self::process_player
					.in_set(GameSet::Movement)
					.run_if(PongPlugin::in_menu),
			);
	}
}

#[derive(Resource)]
struct DebugPrintConfig {
	timer: Timer,
}

impl AiPaddlePlugin {
	fn debug_print(query: Query<(&Paddle, &Player)>, time: Res<Time>, mut config: ResMut<DebugPrintConfig>) {
		config.timer.tick(time.delta());

		if config.timer.finished() {
			for (paddle, player) in query.iter() {
				if player == &Player::Right {
					continue;
				}
				debug!("Paddle.velocity = {:?} for Player({:?})", paddle.velocity, player);
			}
		}
	}

	fn setup(mut commands: Commands) {
		commands.insert_resource(DebugPrintConfig {
			timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
		})
	}

	fn process_player(
		mut paddle_query: Query<(&Transform, &mut Paddle, &Player), With<Paddle>>,
		mut ball_query: Query<(&Transform, &Ball)>,
	) {
		for (ball_transform, ball) in ball_query.iter_mut() {
			for (paddle_transform, mut paddle, player) in paddle_query.iter_mut() {
				let ball_x = ball_transform.translation.x;
				let ball_y = ball_transform.translation.y;
				let paddle_y = paddle_transform.translation.y;
				let ball_vx = ball.velocity().x;
				let ball_vy = ball.velocity().y;

				let ball_is_moving_towards_player = match player {
					&Player::Left => ball_vx < 0.0 && ball_x < WINDOW_WIDTH * 0.25,
					&Player::Right => ball_vx > 0.0 && ball_x > -WINDOW_WIDTH * 0.25,
				};

				if ball_is_moving_towards_player {
					if ball_y != paddle_y {
						let time_til_collision = match player {
							&Player::Left => (Paddle::MARGIN + Paddle::WIDTH + ball_x) / ball_vx,
							&Player::Right => (WINDOW_WIDTH - Paddle::MARGIN - Paddle::WIDTH - ball_x) / ball_vx,
						};
						let desired_distance = paddle_y - ball_y;
						let desired_velocity = -desired_distance;

						paddle.velocity.y = ball.velocity().y.clamp(-paddle.speed, paddle.speed);
					} else {
						debug!("Paddle is already centered at the ball");
						paddle.velocity.y = 0.0;
					}
				} else {
					// debug!("Ball is moving away from the paddle ({:?})", player);
					paddle.velocity.y = 0.0;
				}
			}
		}
	}
}
