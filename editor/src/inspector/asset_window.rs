use std::collections::HashMap;

use engine::{
    assets::{AssetStorage, SceneAssetID},
    scene::{Scene, SceneError, SceneManager},
};

#[derive(Debug, Default)]
pub struct AssetViewTab {
    scene_assets_list: HashMap<SceneAssetID, String>,
    file_assets_list: HashMap<String, String>,
}

impl engine::editor::EditorTab for AssetViewTab {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "AssetViewTab"
    }

    fn display_name(&self) -> String {
        "AssetViewTab".to_string()
    }

    fn ui(
        &mut self,
        window_state: &mut engine::editor::WindowState,
        ui: &mut egui::Ui,
    ) -> Option<engine::editor::TabResponse> {
        let Some(ok_or) = window_state
            .world_ref()
            .resource::<SceneManager>()
            .target_scene
        else {
            ui.label("No target scene found.");
            return None;
        };

        let Some(scene) = window_state.world_mut().get_mut::<Scene>(ok_or) else {
            ui.label("No scene component found on target scene entity.");
            return None;
        };

        for (id, asset) in scene.iter_assets() {
            if self.scene_assets_list.get(id).is_none() {
                self.scene_assets_list.insert(*id, asset.asset_name.clone());
            }
        }

        for (id, name) in &self.scene_assets_list {
            ui.selectable_label(false, name)
                .on_hover_text(format!("{:?}", id));
        }

        for (path, name) in &self.file_assets_list {
            ui.selectable_label(false, name).on_hover_text(path);
        }

        None
    }
}
