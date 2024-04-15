use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::reflect::ReflectFromWorld;
use bevy_ecs::world::Mut;
use bevy_ecs::world::World;
use bevy_reflect::DynamicStruct;
use bevy_reflect::DynamicTupleStruct;
use bevy_reflect::Struct;
use bevy_utils::tracing::trace;
use bevy_utils::Duration;
use engine::scene::ReflectTestSuperTrait;
use engine::scene::SceneManager;
use log::error;

use super::field_view::InspectableAsField;
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
    match state.components.get(&entity.0).cloned() {
        Some(some) => {
            state
                .world()
                .resource_scope(|world, res: Mut<SceneManager>| {
                    for component in some {
                        ui.collapsing(component.clone(), |ui: &mut egui::Ui| {
                            let reg = res.type_registry.get_with_type_path(&component).unwrap();
                            match reg.type_info() {
                                bevy_reflect::TypeInfo::Struct(s) => {
                                    for field in s.iter() {
                                        let field_widget = res
                                            .type_registry
                                            .get(field.type_id())
                                            .unwrap()
                                            .data::<InspectableAsField>()
                                            .expect(
                                                format!(
                                                    "InspectableAsField to be implemented into {}",
                                                    field.type_path()
                                                )
                                                .as_str(),
                                            )
                                            .create_widget();

                                        ui.add(field_widget);
                                    }
                                }
                                bevy_reflect::TypeInfo::TupleStruct(ts) => todo!(),
                                bevy_reflect::TypeInfo::Enum(_) => todo!(),
                                _ => panic!("Incorrect type registration"),
                            }
                        });
                    }
                });
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
        let mut close_inspector: bool = false;
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
                        close_inspector = true;

                        trace!("Clicked on component button");

                        let reflect_component = match type_.data::<ReflectComponent>() {
                            Some(some) => some,
                            None => {
                                error!("Couldnt find ReflectComponent type data for {}", path);
                                continue;
                            }
                        };

                        let mut entity_mut = world.entity_mut(entity.0);

                        trace!("Setting fields");

                        match type_.type_info() {
                            bevy_reflect::TypeInfo::Struct(s) => {
                                let mut bundle = DynamicStruct::default();

                                trace!("Setting represented type");

                                bundle.set_represented_type(Some(type_.type_info()));

                                log::trace!("Set. Iterating fields");
                                for field in s.iter() {
                                    let value = res.type_registry.get_type_info(field.type_id());

                                    if value.is_none() {
                                        error!(
                                            "{} has no obtainable type info in the registry!",
                                            field.name()
                                        );
                                        return;
                                    }
                                }

                                for field in bundle.iter_fields() {
                                    println!("{:#?}", field);
                                }
                                log::info!("itered fields");

                                reflect_component.apply_or_insert(
                                    &mut entity_mut,
                                    &bundle,
                                    &res.type_registry,
                                );
                            }
                            bevy_reflect::TypeInfo::TupleStruct(ts) => {
                                let mut bundle = DynamicTupleStruct::default();

                                bundle.set_represented_type(Some(type_.type_info()));

                                reflect_component.apply_or_insert(
                                    &mut entity_mut,
                                    &bundle,
                                    &res.type_registry,
                                );
                            }
                            bevy_reflect::TypeInfo::Enum(e) => todo!(),
                            _ => unreachable!(),
                        }

                        log::trace!(
                            "Added {} to {}",
                            type_.type_info().type_path_table().short_path(),
                            entity.1
                        );
                    }
                }
            });

        if close_inspector {
            state.inspector.adding_component = false;
        }
    }
    None
}
