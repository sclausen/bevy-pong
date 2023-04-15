use bevy::{audio::AddAudioSource, prelude::*, reflect::TypeUuid};

use crate::{ball::CollisionEvent, GameSet};

use self::{sfxr_audio_asset_loader::SfxrAudioAssetLoader, sfxr_decoder::SfxrDecoder};

mod sfxr_audio_asset_loader;
mod sfxr_decoder;

#[derive(TypeUuid, Clone, Copy)]
#[uuid = "3f377deb-f29d-4b81-ab40-b4a7cdaa3036"]
pub struct SfxrAudio {
	pub sample: sfxr::Sample,
}

impl Decodable for SfxrAudio {
	type Decoder = SfxrDecoder;

	type DecoderItem = <sfxr::Generator as Iterator>::Item;

	fn decoder(&self) -> Self::Decoder {
		SfxrDecoder::new(self.sample)
	}
}

#[derive(Default, Resource)]
pub struct AudioHandles {
	pub ping: Handle<SfxrAudio>,
	pub pong: Handle<SfxrAudio>,
	pub goal: Handle<SfxrAudio>,
}

pub struct SfxrAudioPlugin;
impl Plugin for SfxrAudioPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<AudioHandles>()
			.add_audio_source::<SfxrAudio>()
			.add_asset::<SfxrAudio>()
			.init_asset_loader::<SfxrAudioAssetLoader>()
			.add_system(Self::play_collision_sound.in_set(GameSet::CollisionDetection))
			.add_startup_system(Self::setup);
	}
}

impl SfxrAudioPlugin {
	fn setup(mut audio_handles: ResMut<AudioHandles>, asset_server: Res<AssetServer>) {
		audio_handles.ping = asset_server.load("sounds/ping.json");
		audio_handles.pong = asset_server.load("sounds/pong.json");
		audio_handles.goal = asset_server.load("sounds/goal.json");
	}

	fn play_collision_sound(
		mut collision_events: EventReader<CollisionEvent>,
		audio: Res<Audio<SfxrAudio>>,
		audio_handles: Res<AudioHandles>,
	) {
		if !collision_events.is_empty() {
			for collision_event in collision_events.iter() {
				match collision_event {
					CollisionEvent::Paddle => {
						audio.play(audio_handles.pong.clone());
					}
					CollisionEvent::Wall => {
						audio.play(audio_handles.ping.clone());
					}
					CollisionEvent::Goal => {
						audio.play(audio_handles.goal.clone());
					}
				};
			}
		}
	}
}
