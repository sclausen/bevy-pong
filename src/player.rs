use bevy::prelude::*;

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub enum Player {
	Left,
	Right,
}

impl Player {
	pub fn movement_keys(&self) -> (KeyCode, KeyCode) {
		match self {
			Player::Left => (KeyCode::W, KeyCode::S),
			Player::Right => (KeyCode::Up, KeyCode::Down),
		}
	}
}
