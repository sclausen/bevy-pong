use bevy::prelude::*;

use crate::GameState;

#[derive(Debug, PartialEq)]
pub enum Reset {
	Soft,
	Hard,
}

pub struct ResetPlugin;
impl Plugin for ResetPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<Reset>().add_system(Self::hard_reset_on_keypress);
	}
}

impl ResetPlugin {
	fn hard_reset_on_keypress(
		mut writer: EventWriter<Reset>,
		state: Res<State<GameState>>,
		keyboard_input: Res<Input<KeyCode>>,
	) {
		if state.0 == GameState::Playing && keyboard_input.just_pressed(KeyCode::R) {
			writer.send(Reset::Hard);
		}
	}
}
