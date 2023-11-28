// hehe

use coffee::graphics::Image;
use std::{collections::HashMap, path::Path};

type AssetModule = HashMap<Box<Path>, Image>;

#[allow(dead_code)]
pub struct Assets {
    /// Image Structure goes as follows:
    ///
    /// {
    ///
    ///     [`AssetModule`] Name: Dict {
    ///         FileName: Image,
    ///     },
    ///
    /// }
    pub internal_assets: HashMap<String, AssetModule>,
    // /// Any assets by third parties however are organized as follows:
    // /// OverallDict {
    // ///     AuthorName {
    // ///         ModuleName: Dict {
    // ///             FileName: Image,
    // ///         },
    // ///     },
    // /// }
    // pub externalAssets: Dict<Dict<Dict<Image>>>,
}

#[allow(dead_code)]
impl Assets {
    /// Inserts a new asset inside of the specified module at the specified path, and returns the old value
    /// if there is one.
    ///
    /// Returns [`None`] if the [`AssetModule`] doesn't exist, or if there was no previous
    /// [`Image`] stored.
    pub fn new_asset(&mut self, module_name: &String, path: &Path, val: Image) -> Option<Image> {
        let module_dict = self.internal_assets.get_mut(module_name);
        let module = module_dict?;
        Some(module.insert(path.to_owned().into(), val)?)
    }

    /// Returns the asset from the given [`AssetModule`] at the given path.
    ///
    /// Returns [`None`] if no
    /// [`Image`] was found.
    pub fn get_asset(&self, module_name: &String, path: &Path) -> Option<&Image> {
        Some(self.internal_assets.get(module_name)?.get(path)?)
    }

    /// Inserts a new module. If no module exists, this returns [`None`]. If one already exists, this
    /// returns the old [`AssetModule`].
    pub fn new_module(&mut self, module_name: &String) -> Option<&AssetModule> {
        self.internal_assets
            .insert(String::clone(module_name), HashMap::new())?;
        self.internal_assets.get(module_name)
    }
}
