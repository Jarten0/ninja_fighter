use std::any::Any;
use std::fmt::Debug;
use std::ops::RangeInclusive;

use super::EditorTab;
use super::TabResponse;
use super::WindowState;
use bevy_ecs::component::ComponentId;
use bevy_ecs::entity::Entity;
use bevy_reflect::FromType;
use bevy_reflect::NamedField;
use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use egui::Ui;
use egui::Widget;
use engine::scene::SceneData;
use engine::space::Vector2;
use log::trace;

#[derive(Debug, Default, Reflect)]
pub struct FieldInspectTab {
    focused_item: Option<(Entity, ComponentId, Vec<String>)>,
    /// Used when selecting the field from a list of components
    selecting_item: (Option<Entity>, Option<ComponentId>),
}

impl super::EditorTab for FieldInspectTab {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Field View"
    }

    fn display_name(&self) -> String {
        Self::name().to_string()
    }

    fn ui(&mut self, window_state: &mut WindowState, ui: &mut egui::Ui) -> Option<TabResponse> {
        let Some((focused_entity, focused_component_, focused_field_path)) = &mut self.focused_item
        else {
            if let Some(entity) = self.selecting_item.0 {
            } else {
                ui.label("No focused entity");

                for entity in &window_state.entities_in_current_scene {
                    let name = &window_state
                        .world_ref()
                        .get::<SceneData>(*entity)
                        .expect("The entity must have a scene data component")
                        .entity_name;

                    if ui.small_button(name).clicked() {
                        self.selecting_item.0 = Some(*entity);
                    }
                }
            }
            return None;
        };

        None
    }
}
