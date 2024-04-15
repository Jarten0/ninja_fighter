use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::world::Mut;
use bevy_ecs::world::World;
use bevy_reflect::DynamicStruct;
use bevy_utils::tracing::trace;
use engine::scene::ReflectTestSuperTrait;
use engine::scene::SceneManager;
use log::error;

use super::InspectorWindow;
use super::Response;

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

    let entity = state.focused_entity.clone().unwrap();

    ui.label("Inspecting ".to_owned() + &entity.1);
    match state.components.get(&entity.0) {
        Some(some) => {
            for component in some {
                ui.add(egui::widgets::Button::new(component));
            }
        }
        None => {
            ui.label("(Components Unavailable) ".to_owned());
        }
    }

    ui.add(egui::Separator::default());

    if ui.button("Add component").clicked() {
        state.inspector.adding_component = true;
    }

    if state.inspector.adding_component {
        state
            .world()
            .resource_scope(|world: &mut World, res: Mut<SceneManager>| {
                let types = res
                    .type_registry
                    .iter()
                    .filter(|i| i.data::<ReflectTestSuperTrait>().is_some());

                for type_ in types {
                    let path = type_.type_info().type_path();

                    if ui.button(path).clicked() {
                        let reflect_component = match type_.data::<ReflectComponent>() {
                            Some(some) => some,
                            None => {
                                error!("Couldnt find ReflectComponent type data for {}", path);
                                continue;
                            }
                        };

                        let bundle = DynamicStruct::default();

                        let mut entity_mut = world.entity_mut(entity.0);

                        reflect_component.apply_or_insert(
                            &mut entity_mut,
                            &bundle,
                            &res.type_registry,
                        );

                        log::trace!(
                            "Added {} to {}",
                            type_.type_info().type_path_table().short_path(),
                            entity.1
                        );
                    }
                }
            });
    }
    None
}
