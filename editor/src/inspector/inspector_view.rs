use super::InspectorWindow;
use super::Response;
use bevy_ecs::world::Mut;
use bevy_ecs::world::World;
use egui::Ui;
use engine::scene::ReflectTestSuperTrait;
use engine::scene::SceneManager;

#[derive(Debug, Default)]
pub struct InspectorViewState {
    adding_component: bool,
}

pub fn draw_inspector(
    state: &mut InspectorWindow,
    ui: &mut egui::Ui,
    tab: &mut <InspectorWindow as egui_dock::TabViewer>::Tab,
) -> Option<Response> {
    if state.focused_entity.is_none() {
        ui.label("No entity in focus");
        return None;
    }

    ui.label("Inspecting ".to_owned() + &state.focused_entity.as_ref().unwrap().1);
    for (id, component) in state
        .components
        .get(&state.focused_entity.as_ref().unwrap().0)
        .unwrap()
    {
        ui.add(egui::widgets::Button::new(component));
    }
    if ui.button("Add component").clicked() {
        state.inspector.adding_component = true;
    }
    if state.inspector.adding_component {
        state
            .world_mut()
            .unwrap()
            .resource_scope(|world: &mut World, res: Mut<SceneManager>| {
                let types = res
                    .type_registry
                    .iter()
                    .filter(|i| i.data::<ReflectTestSuperTrait>().is_some());

                for type_ in types {
                    if ui.button(type_.type_info().type_path()).clicked() {
                        println!("Clicked!")
                    }
                }
            });

        // state.world().
    }
    None
}
