use bevy_ecs::world::{Mut, World};
use engine::scene::{Scene, SceneManager};

use super::{EditorTab, TabResponse, WindowState};

#[derive(Debug, Default)]
pub struct SceneEditorTab {
    recent_err: Option<String>,
}

impl EditorTab for SceneEditorTab {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Scene Editor"
    }

    fn display_name(&self) -> String {
        "Scene".to_string()
    }

    fn ui(&mut self, window_state: &mut WindowState, ui: &mut egui::Ui) -> Option<TabResponse> {
        let world = window_state.world_mut();
        world.resource_scope(|world: &mut World, res: Mut<SceneManager>| {
            match res.get_target_scene_component(world) {
                Ok(ok) => ui.label(ok.name.to_string()),
                Err(err) => ui.label("No target scene available"),
            };

            ui.separator();

            if ui.selectable_label(true, "Save scene").clicked() {
                if let Err(err) = res.save_scene(world) {
                    log::error!("Could not save scene! [{}]", err.to_string());
                    self.recent_err = Some(err.to_string())
                } else {
                    self.recent_err = None;
                }
            }

            if let Some(err) = &self.recent_err {
                ui.label(format!("Save failed! [{}] (Check logs)", err));
            }
        });

        None
    }
}
