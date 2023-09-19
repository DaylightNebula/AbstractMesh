use abstracted_mesh_rust::structs::shapes::{AMShape, self};
use bevy::{prelude::*, asset::{AssetLoader, LoadedAsset}, reflect::{TypePath, TypeUuid}};

pub struct AMLoader;
impl Plugin for AMLoader {
    fn build(&self, app: &mut App) {
        app
            .add_asset_loader(AMBLoader::default())
            .add_asset_loader(AMJLoader::default());
    }
}

#[derive(Default)]
pub struct AMBLoader;
#[derive(Default)]
pub struct AMJLoader;

#[derive(Component, TypePath, TypeUuid)]
#[uuid = "4b82490b-f2a0-407c-9419-2babf2b6481f"]
pub struct LoadedAbstractMesh(Vec<AMShape>);

// bevy loader for amb files
impl AssetLoader for AMBLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // deserialize
            let content = shapes::load_shapes_bin(bytes);
            let content: Vec<AMShape> = if content.is_ok() { content.unwrap() } else { return Err(bevy::asset::Error::msg("Failed to load binary file!")); };

            // return asset
            load_context.set_default_asset(LoadedAsset::new(LoadedAbstractMesh(content)));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["amb"]
    }
}

// bevy loader for amj files
impl AssetLoader for AMJLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // deserialize
            let content = std::str::from_utf8(bytes)?;
            let content = shapes::load_shapes_json(content);
            let content = if content.is_ok() { content.unwrap() } else { return Err(bevy::asset::Error::msg("Failed to load amj file!")) };

            // return asset
            load_context.set_default_asset(LoadedAsset::new(LoadedAbstractMesh(content)));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["amj"]
    }
}

// pub fn finalize_load
