use crate::scene::Counter;
use crate::scene::IDCounter;
use crate::scene::Scene;
use bevy_ecs::system::Resource;
use bevy_reflect::serde::ReflectSerializer;
use bevy_reflect::Reflect;
use bevy_reflect::ReflectSerialize;
use bevy_reflect::TypeData;
use bevy_reflect::TypeRegistry;
use serde::Deserialize;
use serde::Serialize;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;

mod asset_type;
mod id;

/// A collection of data that can be serialized and written to a file.
#[derive(Debug, Clone)]
pub struct Asset<T> {
    pub asset_name: String,
    pub asset_data: T,
    pub id: AssetID,
    pub storage: AssetStorage,
}

impl<T> Asset<T> {
    pub(crate) fn new(asset_name: String, asset_data: T, storage: AssetStorage) -> Self {
        Self {
            asset_name,
            asset_data,
            id: AssetID::get_new(),
            storage,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct AssetID(usize);

impl IDCounter for AssetID {
    fn get_new() -> AssetID {
        pub static ASSET_ID_COUNTER: Counter = Counter::new();
        AssetID(ASSET_ID_COUNTER.get())
    }
}

pub enum AssetSerializationType<'asset> {
    Serde(&'asset dyn erased_serde::Serialize),
    Reflect(&'asset dyn Reflect),
}

/// An asset with additional information stored for ease of serialization.
///
/// Note that this is not the serialized form itself, instead it's really just metadata for assets about to be saved or initialized.
///
/// This lives for at least as long as the asset itself does. It dies whenever control of the asset is handed back to the user.
pub struct SerializableAsset<'asset, 'registry> {
    /// Controls how the asset will be saved
    pub storage: AssetStorage,
    /// The name of the asset, which it or a hashed version is used to identify the asset.
    pub asset_name: String,
    /// Information if reinstantiating it using reflection
    pub asset_data_type: Option<String>, //the module path of the asset data type
    /// The actual asset data itself that will be stored, without any type information here. (That's what `asset_data_type` is for!)
    pub asset_data: AssetSerializationType<'asset>,
    /// The actual asset data itself that will be stored, without any type information here. (That's what `asset_data_type` is for!)
    pub type_registry: &'registry TypeRegistry,
}

impl<'asset, 'registry> SerializableAsset<'asset, 'registry> {
    pub fn from_reflect_asset(
        asset: &'asset Asset<Box<dyn Reflect>>,
        type_registry: &'registry TypeRegistry,
    ) -> Self {
        Self {
            asset_name: asset.asset_name.clone(),
            asset_data: AssetSerializationType::Reflect(asset.asset_data.as_ref()),
            asset_data_type: Some(asset.asset_data.reflect_type_path().to_string()),
            storage: asset.storage.clone(),
            type_registry,
        }
    }

    pub fn from_serialize_asset(
        asset: &'asset Asset<Box<dyn erased_serde::Serialize>>,
        type_registry: &'registry TypeRegistry,
    ) -> SerializableAsset<'asset, 'registry> {
        Self {
            asset_name: asset.asset_name.clone(),
            asset_data: AssetSerializationType::Serde(asset.asset_data.as_ref()),
            asset_data_type: None,
            storage: asset.storage.clone(),
            type_registry,
        }
    }
}

impl<'asset, 'registry> Serialize for SerializableAsset<'asset, 'registry> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("SerializableAsset", 3)?;

        match self.asset_data {
            AssetSerializationType::Serde(serialize) => {
                s.serialize_field("asset_data", serialize)?;
            }
            AssetSerializationType::Reflect(reflect) => {
                let type_id = reflect.type_id();

                let serialize_type_data = self
                    .type_registry
                    .get_type_data::<ReflectSerialize>(type_id)
                    .ok_or(<S::Error as serde::ser::Error>::custom(
                        format!(
                            "Could not find ReflectSerialize type data for asset {}",
                            self.asset_name
                        )
                        .as_str(),
                    ))?;

                match serialize_type_data.get_serializable(reflect) {
                    bevy_reflect::serde::Serializable::Owned(owned) => {
                        s.serialize_field("asset_data", &owned)?;
                    }
                    bevy_reflect::serde::Serializable::Borrowed(borrowed) => {
                        s.serialize_field("asset_data", borrowed)?;
                    }
                }
            }
        }

        s.serialize_field("asset_name", &self.asset_name)?;
        s.serialize_field("asset_data_type", &self.asset_data_type)?;
        s.end()
    }
}

/// * `&'static str` = the name of the field relative to SerializableAsset
/// * `String` = the data, serialized and storable in whatever format is given.
pub type SerializedAsset = HashMap<&'static str, String>;

/// This one works different from most other identifiers, since it's assigned by the scene, instead of a global incremental counter.
///
/// The ID is hashed directly from the name, meaning that no two assets may share a name in the same scene.
///
// Developer's note: I intentionally chose to not use any other kind of state when hashing for a unique ID, since I want this to be interoperable with a file system later aswell.
// The SceneID in that case would be hashed from the file's path from the program root, so having two of those at once would be impossible anyways.
// Maybe I'll change it later if I find a good reason to, but for now I think this works just fine.
#[derive(
    Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Reflect,
)]
pub struct SceneAssetID(pub(crate) usize);

impl SceneAssetID {
    pub fn get(asset_name: &str) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        asset_name.hash(&mut hasher);
        SceneAssetID(hasher.finish().try_into().unwrap())
    }
}

/// Where to store the asset, and the identifier type that will be used when retrieving it.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum AssetStorage {
    /// The asset is stored directly on the entity, inside of a component for another component to access whenever.
    Local(usize),
    /// The asset is stored inside of a scene that any component on any entity inside of the scene can access.
    Scene(SceneAssetID),
    /// The asset is stored inside of a file that anything can access whenever the file exists.
    File(String),
}
