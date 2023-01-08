use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
use serde::Deserialize;

pub struct BinaryAssetPlugin;

impl Plugin for BinaryAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<BinaryAsset>()
            .init_asset_loader::<BinaryAssetLoader>();
    }
}

#[derive(Deserialize, TypeUuid, Default, Clone)]
#[uuid = "6eff28b0-1f79-4a3c-bfa9-6099de804489"]
pub struct BinaryAsset {
    pub data: Vec<u8>,
}

#[derive(Default)]
pub struct BinaryAssetLoader;

impl AssetLoader for BinaryAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let asset = BinaryAsset {
                data: bytes.to_vec(),
            };
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["f08"]
    }
}
