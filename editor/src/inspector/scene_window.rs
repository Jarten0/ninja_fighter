use bevy_ecs::world::{Mut, World};
use engine::scene::{Scene, SceneManager};
use log::trace;

use super::{EditorTab, TabResponse, WindowState};

#[derive(Debug, Default)]
pub struct SceneEditorTab {
    recent_err: Option<String>,
    creating_scene_name: String,
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
        world.resource_scope(
            |world: &mut World, mut res: Mut<SceneManager>| -> Option<TabResponse> {
                match res.get_target_scene_component(world) {
                    Ok(ok) => ui.label(ok.name.to_string()),
                    Err(err) => ui.label("No target scene available"),
                };

                ui.separator();

                if ui.selectable_label(false, "Save scene").clicked() {
                    if let Err(err) = res.save_scene(world) {
                        log::error!("Could not save scene! [{}]", err.to_string());
                        self.recent_err = Some(err.to_string())
                    } else {
                        self.recent_err = None;
                    }
                }

                if ui.selectable_label(false, "Reload scene").clicked() {
                    let path = res
                        .get_target_scene_component(world)
                        .ok()?
                        .save_data_path()
                        .unwrap();

                    if let Err(err) = res.load_scene(world, path.to_path_buf()) {
                        log::error!("Could not load scene! [{}]", err.to_string());
                        self.recent_err = Some(err.to_string())
                    } else {
                        self.recent_err = None;
                    }
                }

                ui.horizontal(|ui| -> Option<()> {
                    if ui.selectable_label(false, "New scene").clicked() {
                        trace!("Adding new scene: {}", &self.creating_scene_name);

                        if let Err(err) = res.new_scene(world, self.creating_scene_name.clone()) {
                            log::error!("Could not create scene! [{}]", err.to_string());
                            self.recent_err = Some(err.to_string())
                        } else {
                            self.recent_err = None;
                        };
                    }

                    ui.text_edit_singleline(&mut self.creating_scene_name);
                    None
                });

                if let Some(err) = &self.recent_err {
                    ui.label(format!("Save failed! [{}] (Check logs)", err));
                }

                for (scene_name, entity) in res.current_scenes.clone().into_iter() {
                    if ui.small_button(scene_name).clicked() {
                        res.target_scene = Some(entity);
                    }
                }

                None
            },
        )
    }
}
