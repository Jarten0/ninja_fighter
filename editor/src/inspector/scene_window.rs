use bevy_ecs::world::{Mut, World};
use engine::scene::{Scene, SceneManager};

use super::{TabResponse, WindowState};

pub fn draw_scene_window(
    state: &mut WindowState,
    ui: &mut egui::Ui,
    tab: &mut String,
) -> Option<TabResponse> {
    let world = state.world_mut();
    world.resource_scope(|world: &mut World, res: Mut<SceneManager>| {
        match res.get_target_scene_component(world) {
            Ok(ok) => ui.label(ok.name.to_string()),
            Err(err) => ui.label("No target scene available"),
        };

        ui.separator();

        if ui.selectable_label(true, "Save scene").clicked() {
            if let Err(err) = res.save_scene(world) {
                log::error!("Could not save scene! [{}]", err.to_string());
                ui.label(format!("Save failed! [{}] (Check logs)", err.to_string()));
            }
        }
    });

    None
}
