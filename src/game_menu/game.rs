use bevy::prelude::*;

use super::{despawn_screen, DisplayQuality, GameState, Volume, TEXT_COLOR};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems((
			Self::game_setup.in_schedule(OnEnter(GameState::Game)),
			Self::game.in_set(OnUpdate(GameState::Game)),
			despawn_screen::<OnGameScreen>.in_schedule(OnExit(GameState::Game)),
		));
	}
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(Timer);

impl GamePlugin {
	pub fn game_setup(
		mut commands: Commands,
		asset_server: Res<AssetServer>,
		display_quality: Res<DisplayQuality>,
		volume: Res<Volume>,
	) {
		let font = asset_server.load("fonts/Pixelzim 3x5.ttf");

		commands
			.spawn((
				NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
						// center children
						align_items: AlignItems::Center,
						justify_content: JustifyContent::Center,
						..default()
					},
					..default()
				},
				OnGameScreen,
			))
			.with_children(|parent| {
				// First create a `NodeBundle` for centering what we want to display
				parent
					.spawn(NodeBundle {
						style: Style {
							// This will display its children in a column, from top to bottom
							flex_direction: FlexDirection::Column,
							// `align_items` will align children on the cross axis. Here the main axis is
							// vertical (column), so the cross axis is horizontal. This will center the
							// children
							align_items: AlignItems::Center,
							..default()
						},
						background_color: Color::BLACK.into(),
						..default()
					})
					.with_children(|parent| {
						// Display two lines of text, the second one with the current settings
						parent.spawn(
							TextBundle::from_section(
								"Will be back to the menu shortly...",
								TextStyle {
									font: font.clone(),
									font_size: 80.0,
									color: TEXT_COLOR,
								},
							)
							.with_style(Style {
								margin: UiRect::all(Val::Px(50.0)),
								..default()
							}),
						);
						parent.spawn(
							TextBundle::from_sections([
								TextSection::new(
									format!("quality: {:?}", *display_quality),
									TextStyle {
										font: font.clone(),
										font_size: 60.0,
										color: Color::BLUE,
									},
								),
								TextSection::new(
									" - ",
									TextStyle {
										font: font.clone(),
										font_size: 60.0,
										color: TEXT_COLOR,
									},
								),
								TextSection::new(
									format!("volume: {:?}", *volume),
									TextStyle {
										font: font.clone(),
										font_size: 60.0,
										color: Color::GREEN,
									},
								),
							])
							.with_style(Style {
								margin: UiRect::all(Val::Px(50.0)),
								..default()
							}),
						);
					});
			});
		// Spawn a 5 seconds timer to trigger going back to the menu
		commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
	}

	// Tick the timer, and change state when finished
	pub fn game(time: Res<Time>, mut game_state: ResMut<NextState<GameState>>, mut timer: ResMut<GameTimer>) {
		if timer.tick(time.delta()).finished() {
			game_state.set(GameState::Menu);
		}
	}
}
