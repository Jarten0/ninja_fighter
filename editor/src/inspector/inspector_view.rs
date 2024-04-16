use std::any::Any;

use bevy_ecs::identifier::error;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::reflect::ReflectFromWorld;
use bevy_ecs::world::Mut;
use bevy_ecs::world::World;
use bevy_reflect::DynamicStruct;
use bevy_reflect::DynamicTupleStruct;
use bevy_reflect::Reflect;
use bevy_reflect::ReflectKind;
use bevy_reflect::Struct;
use bevy_reflect::TypeInfo;
use bevy_reflect::TypeRegistry;
use bevy_utils::Duration;
use engine::scene::ReflectTestSuperTrait;
use engine::scene::SceneManager;
use log::*;

use super::field_view::InspectableAsField;
use super::InspectorWindow;
use super::Response;

#[derive(Debug, Default)]
pub struct InspectorViewState {
    adding_component: bool,
}

pub(super) fn draw_inspector(
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
                        let component_struct = world
                            .components()
                            .get_info(component)
                            .ok_or_else({
                                error!("Could not find component  on {}", entity.1);
                                return;
                                || {}
                            })
                            .unwrap();

                        let type_registration = res
                            .type_registry
                            .get(
                                component_struct
                                    .type_id()
                                    .expect("dynamic components not implemented"), // theres no unwrap with message.
                            )
                            .expect("expected a type registration for component");

                        let ptr = world
                            .get_entity_mut(entity.0)
                            .unwrap()
                            .get_mut_by_id(component)
                            .expect("component not found on entity")
                            .as_mut();

                        let reflected = unsafe {
                            type_registration
                                .data::<bevy_reflect::ReflectFromPtr>()
                                .unwrap()
                                .as_reflect_mut(ptr)
                        };

                        assert_eq!(reflected.type_id(), component_struct.type_id().unwrap());
                        // SAFETY: ptr is of type type_id as required in safety contract, type_id was checked above
                        // also, I'm totally taking "inspiration" from `bevy-inspector-egui`. Go check it out, it's awesome :D

                        match reflected.reflect_mut() {
                            bevy_reflect::ReflectMut::Struct(s) => draw_struct_in_inspector(s, ui),
                            bevy_reflect::ReflectMut::TupleStruct(ts) => todo!(),
                            bevy_reflect::ReflectMut::Enum(e) => todo!(),
                            _ => unimplemented!(),
                        }
                        // ui.collapsing(ui: &mut egui::Ui| {});
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
                            TypeInfo::Struct(s) => {
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
                            TypeInfo::TupleStruct(ts) => {
                                let mut bundle = DynamicTupleStruct::default();

                                bundle.set_represented_type(Some(type_.type_info()));

                                reflect_component.apply_or_insert(
                                    &mut entity_mut,
                                    &bundle,
                                    &res.type_registry,
                                );
                            }
                            TypeInfo::Enum(e) => todo!(),
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

fn draw_struct_in_inspector(
    structure: &mut dyn Struct,
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
) {
    for field in structure.iter_fields() {
        let type_data = type_registry
            .get_type_data::<InspectableAsField>(field.type_id())
            .unwrap();

        type_data.ui;
    }
}
