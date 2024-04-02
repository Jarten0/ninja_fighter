use crate::scene::Counter;
use crate::scene::IDCounter;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

mod asset_type;
mod id;

#[derive(Resource, Debug)]
pub struct AssetManager {
    pub render_stuff: HashMap<AssetID, Asset>,
}

impl AssetManager {
    pub(super) fn new() -> Self {
        Self {
            render_stuff: HashMap::new(),
        }
    }

    /// Temporary, until load_asset can be finished
    pub fn insert_asset(&mut self, asset: Asset) {
        self.render_stuff.insert(asset.id, asset);
    }

    /// Not done
    pub fn load_asset(&mut self) {
        todo!()
    }

    pub fn unload_asset(&mut self, _asset_id: &AssetID) {
        todo!()
    }

    pub fn get_asset(&self, asset_id: &AssetID) -> Result<&Asset, &'static str> {
        match self.render_stuff.get(&asset_id) {
            Some(ok) => Ok(ok),
            None => Err("This asset was either never loaded or it has already been unloaded."),
        }
    }
}

/// A collection of data that can be serialized and written to a file.
#[derive(Debug)]
pub struct Asset {
    asset_data: HashMap<String, Box<dyn Reflect>>,
    id: AssetID,
}

impl Asset {
    pub fn empty() -> Self {
        Self {
            asset_data: HashMap::new(),
            id: AssetID::get_new(),
        }
    }

    pub fn into_builder(&self) -> AssetBuilder {
        AssetBuilder {
            // because you can't clone a hashmap containing a trait object, you just have to do it manually.
            // but hopefully this shows why you shouldn't rely on transmuting assets back into builders.
            asset_data: self
                .asset_data
                .iter()
                .map(|(key, value)| (key.to_owned(), value.clone_value()))
                .collect::<HashMap<String, Box<dyn Reflect>>>(),
        }
    }
}

#[derive(Debug, Default)]
pub struct AssetBuilder {
    asset_data: HashMap<String, Box<dyn Reflect>>,
}

impl AssetBuilder {
    pub fn new() -> Self {
        Self {
            asset_data: HashMap::new(),
        }
    }

    /// Adds an external file asset using it's path.
    pub fn add_external_file(&mut self, path: PathBuf) -> Option<Box<dyn Reflect>> {
        self.add_data("__external_files", Box::new(path))
    }

    pub fn add_data(&mut self, key: &str, data: Box<dyn Reflect>) -> Option<Box<dyn Reflect>> {
        self.asset_data.insert(key.to_owned(), data)
    }

    pub fn build(self) -> Asset {
        Asset {
            asset_data: self.asset_data,
            id: AssetID::get_new(),
        }
    }
}

impl From<AssetBuilder> for Asset {
    fn from(value: AssetBuilder) -> Self {
        value.build()
    }
}

impl From<Asset> for AssetBuilder {
    fn from(value: Asset) -> Self {
        value.into_builder()
    }
}

pub struct ExternalFileData {
    file_path: PathBuf,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct AssetID(usize);

impl IDCounter for AssetID {
    fn get_new() -> AssetID {
        pub static ASSET_ID_COUNTER: Counter = Counter::new();
        AssetID(ASSET_ID_COUNTER.get())
    }
}

/// A list of assets that are grouped together.
///
/// Useful when managing assets with scenes, to load and unload assets with ease.
pub struct AssetBatch {
    pub assets: Vec<AssetID>,
}
