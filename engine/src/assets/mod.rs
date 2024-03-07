use self::asset_type::AssetType;
use crate::assets::id::AssetID;
use bevy_ecs::{system::Resource, world::World};
use std::collections::HashMap;

mod asset_type;
mod id;

#[derive(Resource, Debug)]
pub struct Assets {
    pub render_stuff: HashMap<AssetID, AssetType>,
    pub queue: AssetCommandQueue,
}

impl Assets {
    pub(super) fn new() -> Self {
        Self {
            render_stuff: HashMap::new(),
            queue: AssetCommandQueue { queue: Vec::new() },
        }
    }

    /// Temporary, until load_asset can be finished
    pub fn insert_asset(&mut self, asset_id: AssetID, asset_type: AssetType) {
        self.render_stuff.insert(asset_id, asset_type);
    }

    /// Not
    pub fn load_asset(&mut self) {
        todo!()
    }

    pub fn unload_asset(&mut self, _asset_id: &AssetID) {
        todo!()
    }

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
                self.queue.unload_asset(asset_id.to_owned());
            }
        }
    }
}

#[derive(Debug)]
pub struct AssetCommandQueue {
    queue: Vec<AssetCommand>,
}

impl AssetCommandQueue {
    fn unload_asset(&mut self, asset_id: AssetID) {
        self.queue.push(AssetCommand::Unload(asset_id))
    }
}

#[allow(unused)]
#[derive(Debug)]
enum AssetCommand {
    Load(AssetID),
    Unload(AssetID),
}
