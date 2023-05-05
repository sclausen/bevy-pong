use bevy::{
	prelude::*,
	asset::{AssetLoader, LoadContext, LoadedAsset},
	utils::BoxedFuture,
};

use super::{SfxrAudio, serde::SampleDef};

#[derive(Default)]
pub struct SfxrAudioAssetLoader;

impl AssetLoader for SfxrAudioAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
		Box::pin(async move {
			let mut de = serde_json::Deserializer::from_slice(bytes);
			let schnorp = SampleDef::deserialize(&mut de)?;
			let custom_asset = SfxrAudio {
				sample: schnorp,
			};
			debug!("Loaded SfxrAudio: {:?}", schnorp);
			load_context.set_default_asset(LoadedAsset::new(custom_asset));
			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["json"]
	}
}
