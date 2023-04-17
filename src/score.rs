use bevy::prelude::*;

use std::fmt::{Display, Formatter};
use std::ops::DerefMut;

use crate::GameSet;
use crate::{reset::Reset, wall::Wall};

#[derive(Default, Resource)]
pub struct Score {
	pub left: usize,
	pub right: usize,
}

impl Display for Score {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		write!(formatter, "{:0>2}   {:0>2}", self.left, self.right)
	}
}

#[derive(Default, Component)]
pub struct ScoreBoard;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(Self::setup)
			.add_system(Self::handle_reset.in_set(GameSet::Reset))
			.add_system(Self::update.in_set(GameSet::Movement))
			.insert_resource(Score::default());
	}
}

impl ScorePlugin {
	pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
		commands
			.spawn((
				Name::new("Scoreboard"),
				NodeBundle {
					style: Style {
						align_items: AlignItems::Center,
						flex_direction: FlexDirection::Column,
						justify_content: JustifyContent::FlexStart,
						padding: UiRect {
							top: Val::Px(Wall::WIDTH * 2.0),
							left: Val::Px(5.),
							right: Val::Px(5.),
							bottom: Val::Px(1.),
						},
						size: Size::new(Val::Percent(100.), Val::Px(120.)),
						..default()
					},
					..default()
				},
			))
			.with_children(|commands| {
				commands.spawn((
					ScoreBoard,
					TextBundle::from_sections([TextSection::new(
						Score::default().to_string(),
						TextStyle {
							color: Color::WHITE,
							font_size: 100.,
							font: asset_server.load("fonts/Pixelzim 3x5.ttf"),
						},
					)]),
				));
			});
	}

	pub fn update(
		mut reset_reader: EventReader<Reset>,
		score: Res<Score>,
		mut query: Query<&mut Text, With<ScoreBoard>>,
	) {
		if reset_reader.iter().last().is_none() {
			return;
		}

		for mut text in query.iter_mut() {
			if let Some(section) = text.sections.get_mut(0) {
				section.value = score.to_string();
			}
		}
	}

	pub fn handle_reset(
		mut reset_reader: EventReader<Reset>,
		mut score: ResMut<Score>,
		mut query: Query<&mut Text, With<ScoreBoard>>,
	) {
		if let Some(reset_event) = reset_reader.iter().last() {
			if reset_event == &Reset::Hard {
				score.deref_mut().left = 0;
				score.deref_mut().right = 0;
			}

			for mut text in query.iter_mut() {
				if let Some(section) = text.sections.get_mut(0) {
					section.value = score.to_string();
				}
			}
		}
	}
}
