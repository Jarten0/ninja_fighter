use self::asset_type::AssetType;
use super::RenderType;
use crate::engine::assets::id::AssetID;
use bevy_ecs::{
    system::Resource,
    world::{self, World},
};
use std::collections::HashMap;

mod asset_type;
mod id;

#[derive(Resource, Debug)]
pub struct Assets {
    pub render_stuff: HashMap<AssetID, AssetType>,
}

impl Assets {
    pub(super) fn new() -> Self {
        Self {
            render_stuff: HashMap::new(),
        }
    }

    pub fn load_asset(&mut self) {}

    pub fn unload_asset(&mut self, asset_id: &AssetID) {}

    pub fn get_asset(&self, asset_id: &AssetID) -> Result<&AssetType, &'static str> {
        match self.render_stuff.get(&asset_id) {
            Some(ok) => Ok(ok),
            None => Err("This asset was either never loaded or it has already been unloaded."),
        }
    }

    pub fn check_for_unloads(&mut self, world: &World) {
        for asset_id in self.render_stuff.keys() {
            if let Some(condition) = asset_id.unload_condition {
                condition(asset_id, &world);
                self.unload_asset(asset_id);
            }
        }
    }
}

pub struct AssetCommandQueue {
    pub queue: Vec<AssetCommand>,
}

impl AssetCommandQueue {}

enum AssetCommand {
    Load(AssetID),
    Unload(),
}
