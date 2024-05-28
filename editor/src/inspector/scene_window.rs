use bevy_ecs::world::{Mut, World};
use engine::scene::{Scene, SceneManager};
use engine::GgezInterface;
use log::trace;

use super::{EditorTab, TabResponse, WindowState};

#[derive(Debug, Default)]
pub struct SceneEditorTab {
    recent_err: Option<String>,
    message: Option<String>,
    creating_scene_name: String,
}

impl SceneEditorTab {
    /// Resets the recent err and current message. Should be called on every significant user interaction (Clicked a button that can cause a fallible interaction).
    fn reset(&mut self) {
        self.recent_err = None;
        self.message = None;
    }

    fn message(&mut self, message: String) {
        self.message = Some(message);
        self.recent_err = None;
    }

    fn error(&mut self, error: String) {
        self.message = None;
        self.recent_err = Some(error);
    }
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

                if ui.separator().clicked() {
                    self.reset();
                }

                ui.horizontal(|ui| -> Option<()> {
                    if ui.selectable_label(false, "New scene").clicked() {
                        self.reset();

                        if self.creating_scene_name == "" {
                            log::error!("No scene name specified");
                            self.error("No scene name specified".to_owned());
                            return None;
                        }

                        trace!("Adding new scene: {}", &self.creating_scene_name);

                        if let Err(err) = res.new_scene(world, self.creating_scene_name.clone()) {
                            log::error!("Could not create scene! [{}]", err.to_string());
                            self.error(err.to_string());
                        } else {
                            self.message("Scene created successfully".to_string());
                        };
                    }

                    ui.text_edit_singleline(&mut self.creating_scene_name);
                    None
                });

                if ui.selectable_label(false, "Save scene").clicked() {
                    self.reset();

                    if let Err(err) = res.save_scene(world) {
                        log::error!("Could not save scene! [{}]", err.to_string());
                        self.error(err.to_string());
                    } else {
                        self.message("Saved successfully".to_owned());
                    }
                }

                if ui.selectable_label(false, "Load scene").clicked() {
                    self.reset();

                    let scenes_folder = world
                        .resource::<GgezInterface>()
                        .get_engine_config()
                        .scenes_folder
                        .clone();

                    let path = if let Some(folder) = scenes_folder {
                        folder.into()
                    } else {
                        std::env::current_dir().expect("no errors from currentdir")
                    };

                    let file_dialog = rfd::FileDialog::new()
                        .add_filter("json", &["json"])
                        .set_directory(&path)
                        .pick_file();

                    if let Some(path) = file_dialog {
                        if let Err(err) = res.load_scene(world, path) {
                            log::error!("Could not load scene: {}", err);
                            self.error(err.to_string());
                        }
                    } else {
                        log::error!("File selection operation cancelled");
                        self.error("File selection operation cancelled".to_owned());
                    }
                }

                if ui.selectable_label(false, "Reload scene").clicked() {
                    self.reset();

                    let Some(path) =
                        (match res.get_target_scene_component(world).ok()?.save_data_path() {
                            Some(some) => Some(some.to_owned()),
                            None => rfd::FileDialog::new()
                                .set_directory(std::env::current_dir().unwrap())
                                .add_filter("JSON", &["json"])
                                .pick_file(),
                        })
                    else {
                        log::error!("Could not find file to reload");
                        self.error("Scene file not found".to_owned());
                        return None;
                    };

                    if let Err(err) = res.unload_scene(world) {
                        log::error!("Could not unload scene! [{}]", err);
                        self.error(err.to_string())
                    }

                    if let Err(err) = res.load_scene(world, path.to_path_buf()) {
                        log::error!("Could not load scene! [{}]", err.to_string());
                        self.error(err.to_string());
                    } else {
                        self.message("Reloaded scene".to_owned());
                    }
                }

                if let Some(msg) = &self.message {
                    ui.label(format!("{}", msg));
                }

                if let Some(err) = &self.recent_err {
                    ui.label(format!("Error: {} (Check logs)", err));
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
