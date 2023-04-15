use bevy::{
	asset::{AssetLoader, LoadContext, LoadedAsset},
	utils::BoxedFuture,
};

use super::SfxrAudio;

#[derive(Default)]
pub struct SfxrAudioAssetLoader;

impl AssetLoader for SfxrAudioAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
		Box::pin(async move {
			let custom_asset = SfxrAudio {
				sample: serde_json::from_slice::<sfxr::Sample>(bytes)?,
			};
			load_context.set_default_asset(LoadedAsset::new(custom_asset));
			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["json"]
	}
}
