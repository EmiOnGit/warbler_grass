use bevy::{asset::{AssetLoader, LoadContext, LoadedAsset}, utils::BoxedFuture, reflect::{TypeUuid, Reflect}};
/// Each entry contains a rect internally defined by [x,z,width,height]
#[derive(Debug, TypeUuid, serde::Deserialize, Reflect)]
#[uuid = "39a3dc56-aa9c-4543-8640-a018b74b5052"]
pub struct GrassDataAsset(pub Vec<[u16; 5]>);

#[derive(Default)]
pub struct GrassDataAssetLoader;
impl AssetLoader for GrassDataAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<GrassDataAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["ron", "grass"]
    }
}