use bevy_ecs::component::ComponentId;
use bevy_ecs::identifier::error;
use bevy_ecs::reflect::{ReflectComponent, ReflectFromWorld};
use bevy_ecs::world::{EntityMut, EntityWorldMut, Mut, World};
use bevy_reflect::{Array, DynamicTypePath, Enum, List, Struct, Tuple, TupleStruct};
use bevy_reflect::{DynamicStruct, DynamicTupleStruct};
use bevy_reflect::{Reflect, ReflectKind, ReflectMut};
use bevy_reflect::{TypeInfo, TypeRegistration, TypeRegistry};
use bevy_utils::Duration;
use convert_case::Casing as _;
use egui::{ComboBox, Label};
use engine::editor::InspectableAsField;
use engine::editor::*;
use engine::scene::SceneManager;
use log::*;
use std::any::Any;

#[derive(Debug, Default)]
pub struct InspectorTab {
    adding_component: bool,
}

impl super::EditorTab for InspectorTab {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Inspector"
    }

    fn display_name(&self) -> String {
        "Inspector".to_string()
    }

    fn ui(&mut self, window_state: &mut WindowState, ui: &mut egui::Ui) -> Option<TabResponse> {
        draw_inspector(self, window_state, ui)
    }
}

pub(super) fn draw_inspector(
    tab: &mut InspectorTab,
    state: &mut WindowState,
    ui: &mut egui::Ui,
) -> Option<TabResponse> {
    if state.focused_entity.is_none() {
        ui.label("No entity in focus");
        return None;
    }

    let entity = state.focused_entity.clone().unwrap();

    let mut current_response = None;

    let debug_mode = state.debug_mode;

    ui.label("Inspecting ".to_owned() + &entity.1);

    let Some(component_ids) = state.components.get(&entity.0).cloned() else {
        ui.label("(Components Unavailable)".to_owned());

        error!("Could not find component data for {}", &entity.1);
        return None;
    };

    state
        .world_mut()
        .resource_scope(|world, res: Mut<SceneManager>| {
            // qualified since bevy_reflect::List import is auto implemented for Vec
            for component_id in <[ComponentId]>::iter(&component_ids) {
                let Some(component_info) = world.components().get_info(*component_id) else {
                    error!(
                        "Could not find component {:?} on {}",
                        component_id, entity.1
                    );
                    return;
                };

                let component_type_id = component_info.clone().type_id().unwrap();

                let type_registration = res
                    .type_registry
                    .get(component_type_id)
                    .expect("expected a type registration for component");

                let Some(mut get_entity_mut) = world.get_entity_mut(entity.0) else {
                    error!("Could not find entity in world");
                    return;
                };

                let Some(mut ptr) = get_entity_mut.get_mut_by_id(*component_id) else {
                    error!("Could not get untyped component from ComponentID.");
                    return;
                };

                let reflected = unsafe {
                    let val = ptr.as_mut();

                    type_registration
                        .data::<bevy_reflect::ReflectFromPtr>()
                        .unwrap()
                        .as_reflect_mut(val)
                };

                assert_eq!(reflected.as_reflect().type_id(), component_type_id);
                // SAFETY: ptr is of type type_id as required in safety contract, type_id was checked above
                // also, I'm totally taking "inspiration" from `bevy-inspector-egui`. Go check it out, it's awesome :D

                let type_path = reflected.reflect_type_path().to_owned();

                let display_path = match debug_mode {
                    true => type_path,
                    false => type_path
                        .split("::")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .to_string()
                        .to_case(convert_case::Case::Title),
                };

                if let ReflectMut::Struct(s) = reflected.reflect_mut() {
                    if s.field_len() == 0 {
                        if let Some(inspectable) = type_registration.data::<InspectableAsField>() {
                            inspectable.show(ui, s.as_reflect_mut());
                            if !debug_mode {
                                continue;
                            }
                        };
                        ui.label(display_path.clone());
                    } else {
                        ui.collapsing(display_path.clone(), |ui| {
                            if let Some(inspectable) =
                                type_registration.data::<InspectableAsField>()
                            {
                                inspectable.show(ui, s.as_reflect_mut());
                                if !debug_mode {
                                    return;
                                }
                            };
                            reflect_struct_ui(s, ui, &res.type_registry, debug_mode);
                        })
                        .header_response
                        .context_menu(|ui| {
                            if ui.selectable_label(true, "Remove component").clicked() {
                                current_response =
                                    Some(TabResponse::RemoveComponent(entity.0, *component_id))
                            }
                        });
                    }
                };
                if let ReflectMut::TupleStruct(ts) = reflected.reflect_mut() {
                    ui.collapsing(display_path.clone(), |ui| {
                        if let Some(inspectable) = type_registration.data::<InspectableAsField>() {
                            inspectable.show(ui, ts.as_reflect_mut());
                            if !debug_mode {
                                return;
                            }
                        };
                        reflect_tuple_struct_ui(ts, ui, &res.type_registry, debug_mode);
                    });
                };
                if let ReflectMut::Enum(e) = reflected.reflect_mut() {
                    if let Some(inspectable) = type_registration.data::<InspectableAsField>() {
                        inspectable.show(ui, e.as_reflect_mut());
                        if !debug_mode {
                            continue;
                        }
                    };
                    reflect_enum_ui(e, ui, &res.type_registry, debug_mode);
                };
            }
        });

    ui.add(egui::Separator::default());

    ui.collapsing("Add components", |ui| {
        let modules = &state.component_modules.clone();
        let world = state.world_mut(); // pulled out into variable so that formatter wouldnt add an extra tab of indentation for no reason like why is it so much
        world.resource_scope(|world: &mut World, res: Mut<SceneManager>| {
            for module in modules {
                let response = ui.collapsing(module.0.clone(), |ui| {
                    for (component, type_) in module.1 {
                        if let Some(c_id) = world.components().get_id(type_.type_id()) {
                            if component_ids.contains(&c_id) {
                                continue;
                            }
                        }

                        let display_path = match debug_mode {
                            true => {
                                component.1.clone()
                            },
                            false => {
                                component.1.split("::").collect::<Vec<&str>>().pop().unwrap().to_string()
                            },
                        };

                        let mut button = ui.button(display_path);
                        if let Some(component_docs) = type_.type_info().docs() {
                            button = button.on_hover_text(component_docs);
                        }

                        if button.clicked() {
                            trace!("Clicked on component add button");

                            let reflect_component = match type_.data::<ReflectComponent>() {
                                Some(some) => some,
                                None => {
                                    log::error!(
                                        "Couldnt find ReflectComponent type data for {}.
                                        (Perhaps you're missing the #[reflect(Component) macro attribute)",
                                        component.1
                                    );

                                    continue;
                                }
                            };

                            let mut entity_mut = world.entity_mut(entity.0);

                            trace!("Setting values for fields in component");

                            match type_.type_info() {
                                TypeInfo::Struct(s) => {
                                    let mut bundle = DynamicStruct::default();

                                    trace!("Setting represented type to iterate over expected fields for type info");

                                    bundle.set_represented_type(Some(type_.type_info()));
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

                            log::info!(
                                "Added {} to {}",
                                type_.type_info().type_path_table().short_path(),
                                entity.1
                            );
                        }
                    }
                });
            }
        });
    });
    current_response
}

fn reflect_field_ui(
    ui: &mut egui::Ui,
    field: &mut dyn Reflect,
    type_registry: &TypeRegistry,
    debug_mode: bool,
) {
    match field.reflect_mut() {
        ReflectMut::Struct(structure) => {
            reflect_struct_ui(structure, ui, type_registry, debug_mode)
        }
        ReflectMut::TupleStruct(tuplestruct) => {
            reflect_tuple_struct_ui(tuplestruct, ui, type_registry, debug_mode)
        }
        ReflectMut::Tuple(tuple) => reflect_tuple_ui(ui, tuple, type_registry, debug_mode),
        ReflectMut::List(list) => reflect_list_ui(ui, list, type_registry, debug_mode),
        ReflectMut::Array(array) => reflect_array_ui(ui, array, type_registry, debug_mode),
        ReflectMut::Map(map) => reflect_map_ui(ui, map, type_registry, debug_mode),
        ReflectMut::Enum(enumerator) => reflect_enum_ui(enumerator, ui, type_registry, debug_mode),
        ReflectMut::Value(value) => {
            reflect_value_ui(type_registry, value, ui, debug_mode);
        }
    }
}

fn reflect_struct_ui(
    structure: &mut dyn Struct,
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    debug_mode: bool,
) {
    let Some(some) = structure.get_represented_type_info().cloned() else {
        error!(
            "Could not get type info for {}",
            structure.reflect_type_path()
        );
        return;
    };
    let TypeInfo::Struct(struct_info) = some else {
        error!(
            "Invalid type info for {}, this is not a struct",
            structure.reflect_type_path()
        );
        return;
    };

    let grid = egui::Grid::new(structure.reflect_type_path())
        .num_columns(2)
        .striped(true);

    grid.show(ui, |ui: &mut egui::Ui| {
        for field_name in struct_info.field_names() {
            let name = field_name.to_owned();

            let field_mut = Struct::field_mut(structure, name);

            let Some(field) = field_mut else {
                // this error log is breaking rustfmt >:(
                // log::error!("Invalid field data: Field {} exists in type data, but not in the reflected struct.", field_name);
                continue;
            };

            let display_name = match debug_mode {
                true => field_name.to_string(),
                false => field_name.to_case(convert_case::Case::Title),
            };
            if let Some(type_data) =
                type_registry.get_type_data::<InspectableAsField>(field.as_reflect().type_id())
            {
                let response = ui.add(egui::Label::new(display_name));
                if let Some(field_docs) = struct_info.field(&field_name).unwrap().docs() {
                    response.on_hover_text(field_docs);
                }

                ui.scope(|ui: &mut egui::Ui| {
                    type_data.show(ui, field);
                });

                ui.end_row();

                continue;
            }

            reflect_field_ui(ui, field, type_registry, debug_mode);

            ui.end_row();
        }
    });
}

fn reflect_tuple_struct_ui(
    tuplestruct: &mut dyn TupleStruct,
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    debug_mode: bool,
) {
    let Some(some) = tuplestruct.get_represented_type_info().cloned() else {
        error!(
            "Could not get type info for {}",
            tuplestruct.reflect_type_path()
        );
        return;
    };
    let TypeInfo::TupleStruct(tuple_struct_info) = some else {
        error!(
            "Invalid type info for {}, this is not a struct",
            tuplestruct.reflect_type_path()
        );
        return;
    };

    for unnamed_field in tuple_struct_info.iter() {
        let field_mut = tuplestruct.field_mut(unnamed_field.index());

        let Some(field) = field_mut else {
            error!("Invalid field data: Field at {} exists in type data, but not in the reflected struct.", unnamed_field.index());
            continue;
        };

        let Some(type_data) =
            type_registry.get_type_data::<InspectableAsField>(field.as_reflect().type_id())
        else {
            error!(
                "Could not find inspector data for type [{}] for field at index [{}]",
                field.reflect_type_path(),
                unnamed_field.index()
            );
            continue;
        };

        reflect_field_ui(ui, field, type_registry, debug_mode)
    }
}

fn reflect_tuple_ui(
    ui: &mut egui::Ui,
    tuple: &mut dyn Tuple,
    type_registry: &TypeRegistry,
    debug_mode: bool,
) {
    todo!()
}
fn reflect_list_ui(
    ui: &mut egui::Ui,
    list: &mut dyn List,
    type_registry: &TypeRegistry,
    debug_mode: bool,
) {
    todo!()
}

fn reflect_array_ui(
    ui: &mut egui::Ui,
    array: &mut dyn Array,
    type_registry: &TypeRegistry,
    debug_mode: bool,
) {
    todo!()
}

fn reflect_map_ui(
    ui: &mut egui::Ui,
    m: &mut dyn bevy_reflect::Map,
    type_registry: &TypeRegistry,
    debug_mode: bool,
) {
    // let grid = egui::Grid::new("hashmap").num_columns(2).striped(true);

    // grid.show(ui, |ui| {
    let mut keys = vec![];

    for (key, _) in m.iter() {
        keys.push(key.clone_value());
    }

    let grid = egui::Grid::new("testhashmap").num_columns(2).striped(true);

    let Some(key) = keys.get(0) else {
        ui.label("No items in map");
        return;
    };

    let type_registration = type_registry
        .get_with_type_path(key.reflect_type_path())
        .unwrap();
    let value_type_registration = type_registry
        .get_with_type_path(m.get(&**key).unwrap().reflect_type_path())
        .unwrap();

    let Some(inspectable_as_field) = type_registration.data::<InspectableAsField>() else {
        ui.label("Key has no inspection implementation (InspectableAsField)");
        return;
    };

    grid.show(ui, |ui| {
        for (index, mut key) in keys.into_iter().enumerate() {
            let value = m.get_mut(&*key).unwrap();

            inspectable_as_field.show(ui, &mut *key);

            reflect_field_ui(ui, value, type_registry, debug_mode);

            ui.end_row()
        }
    });
    let selectable_label = ui.selectable_label(false, "Add element").on_hover_text("");
    if selectable_label.clicked() {
        m.insert_boxed(
            type_registration
                .data::<bevy_reflect::std_traits::ReflectDefault>()
                .unwrap()
                .default(),
            value_type_registration
                .data::<bevy_reflect::std_traits::ReflectDefault>()
                .unwrap()
                .default(),
        );
    }
    // });
}

fn reflect_enum_ui(
    structure: &mut dyn Enum,
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    debug_mode: bool,
) {
    ComboBox::new("Enum", "Not supported").show_ui(ui, |ui| {});
}

fn reflect_value_ui(
    type_registry: &TypeRegistry,
    value: &mut dyn Reflect,
    ui: &mut egui::Ui,
    debug_mode: bool,
) {
    let Some(type_data) = type_registry
        .get_with_type_path(value.reflect_type_path())
        .unwrap()
        .data::<InspectableAsField>()
    else {
        ui.label("not implemented");
        return;
    };

    type_data.show(ui, value);
}
