use std::any::Any;
use std::fmt::Debug;
use std::ops::RangeInclusive;

use super::TabResponse;
use super::WindowState;
use bevy_reflect::FromType;
use bevy_reflect::NamedField;
use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use egui::Ui;
use egui::Widget;
use engine::space::Vector2;
use log::trace;

#[derive(Debug, Default)]
pub struct FieldViewState {}

pub(super) fn draw_field(
    state: &mut WindowState,
    ui: &mut egui::Ui,
    tab: &mut <WindowState as egui_dock::TabViewer>::Tab,
) -> Option<TabResponse> {
    ui.label("No implementation of field editor at the moment");
    None
}
