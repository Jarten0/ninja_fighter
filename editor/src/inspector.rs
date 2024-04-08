mod button;
use bevy_ecs::{prelude::*, query};
use bevy_reflect::{Reflect, TypeInfo, TypeRegistration, TypeRegistry};
use engine::{
    scene::{self, ObjectID, SceneData},
    GgezInterface, Input,
};
use ggez::{
    graphics::{self, Canvas, Color, DrawParam, Drawable, FillOptions, TextFragment},
    mint::Point2,
};
use log::*;
use std::{collections::HashMap, default, fmt::Debug};

#[derive(Debug, Resource)]
pub struct Inspector {
    enabled: bool,
    width: f32,
    view: InspectorView,
    pub next_y_position: f32,
    elements: InspectorElementContainer,
}

/// The container for all of the various inspector tabs
#[derive(Debug, Default)]
pub struct InspectorElementContainer {
    entities: Vec<String>,
    components_list: HashMap<ObjectID, Box<dyn InspectorElement>>,
}

impl Default for Inspector {
    fn default() -> Self {
        Self {
            enabled: false,
            width: 600.0,
            view: InspectorView::default(),
            elements: InspectorElementContainer::default(),
            next_y_position: 0.0,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum InspectorView {
    #[default]
    Entities,
    Components,
    Field,
    DebugInfo,
    Misc,
}

impl InspectorView {
    pub fn next_tab(&mut self) -> InspectorView {
        let tab = match self {
            InspectorView::Entities => InspectorView::Components,
            InspectorView::Components => InspectorView::Field,
            InspectorView::Field => InspectorView::DebugInfo,
            InspectorView::DebugInfo => InspectorView::Misc,
            InspectorView::Misc => InspectorView::Misc,
        };

        *self = tab.clone();

        tab
    }

    pub fn previous_tab(&mut self) -> InspectorView {
        let tab = match self {
            InspectorView::Entities => InspectorView::Entities,
            InspectorView::Components => InspectorView::Entities,
            InspectorView::Field => InspectorView::Components,
            InspectorView::DebugInfo => InspectorView::Field,
            InspectorView::Misc => InspectorView::DebugInfo,
        };

        *self = tab.clone();

        tab
    }
}

pub trait InspectorElement
where
    Self: Send + Sync + Debug,
{
    /// How tall is this element in the inspector, at this current frame
    fn get_height(&self) -> f32;

    /// Which inspector view does this element belong to
    fn view(&self) -> InspectorView;

    /// Draws to the canvas based on the current state stored in the element's state, updated by it's own systems.
    ///
    /// That means you should handle state managment outside of this function, only using it to draw to the canvas.
    /// You can still manage the state in your own draw functions. This will be called after those functions are called.
    ///
    /// Make use of the inspector to
    fn draw(&self, canvas: &mut Canvas, inspector: &mut ResMut<Inspector>);
}

pub fn update_inspector(
    query: Query<(Entity, &InspectorData)>,
    entities_without_data: Query<(Entity, &SceneData), Without<InspectorData>>,
    mut commands: Commands,
    mut inspector: ResMut<Inspector>,
    input: Res<Input>,
) {
    if input
        .get_action("nextInspectorTab")
        .unwrap()
        .is_just_pressed()
    {
        inspector.view.next_tab();
    } else if input
        .get_action("nextInspectorTab")
        .unwrap()
        .is_just_pressed()
    {
        inspector.view.previous_tab();
    }

    for (entity, scene_data) in entities_without_data.iter() {
        commands.entity(entity).insert(InspectorData {});
        inspector
            .elements
            .entities
            .push(scene_data.object_name.clone());
    }

    for (entity, scene_data) in query.iter() {}
}

pub fn draw_inspector(
    mut inspector: ResMut<Inspector>,
    mut engine: ResMut<GgezInterface>,
    input: Res<Input>,
) {
    if input
        .get_action("enableinspector")
        .unwrap()
        .is_just_pressed()
    {
        match inspector.enabled {
            false => {
                inspector.enabled = true;
                debug!("Enabled inspector");
            }
            true => {
                inspector.enabled = false;
                debug!("Disabled inspector");
            }
        }
    }

    if !inspector.enabled {
        return;
    }

    engine.get_context().gfx.window().set_maximized(true);

    let inspector_rect = graphics::Rect::new(1920.0 - inspector.width, 0.0, 600.0, 1060.0);
    let quad = graphics::Mesh::new_rectangle(
        &engine.get_context().gfx,
        graphics::DrawMode::Fill(FillOptions::DEFAULT),
        inspector_rect,
        Color::from_rgba(40, 40, 40, 230),
    )
    .unwrap();

    quad.draw(engine.get_canvas_mut().unwrap(), DrawParam::new());

    match inspector.view {
        InspectorView::Entities => inspector_draw_entities(inspector, engine, input),
        InspectorView::Components => inspector_draw_components(inspector, engine, input),
        InspectorView::DebugInfo => todo!(),
        InspectorView::Misc => todo!(),
        _ => todo!(),
    }
}

fn inspector_draw_components(
    mut inspector: ResMut<Inspector>,
    mut engine: ResMut<GgezInterface>,
    input: Res<Input>,
) {
    let mut y = 0.0;
    for id in &inspector.elements.entities {
        let text = graphics::Text::new(TextFragment::new(id));

        text.draw(
            engine.get_canvas_mut().unwrap(),
            DrawParam::new().dest(Point2 { x: 1320.0, y }),
        );
        y += 20.0;
    }
}

fn inspector_draw_entities(
    mut inspector: ResMut<Inspector>,
    mut engine: ResMut<GgezInterface>,
    input: Res<Input>,
) {
    let mut y = 0.0;
    for id in &inspector.elements.entities {
        let text = graphics::Text::new(TextFragment::new(id));

        text.draw(
            engine.get_canvas_mut().unwrap(),
            DrawParam::new().dest(Point2 { x: 1320.0, y }),
        );
        y += 20.0;
    }
}

#[derive(Debug)]
pub struct ComponentInspectorElement {
    pub entity: Entity,
    type_info: TypeInfo,
}

impl ComponentInspectorElement {
    pub fn new(entity: Entity, component: &dyn Reflect, registry: &TypeRegistry) -> Self {
        let type_info = registry
            .get(component.type_id())
            .expect("Expected a type registration of the given component")
            .type_info()
            .clone();

        Self { entity, type_info }
    }
}

impl InspectorElement for ComponentInspectorElement {
    fn get_height(&self) -> f32 {
        let mut height = 30;
        match &self.type_info {
            TypeInfo::Struct(s) => {
                for field in s.iter() {
                    if field.is::<&dyn InspectorValue>() {
                        height += (field. as &dyn InspectorValue).height();
                    }
                }
            }
            TypeInfo::TupleStruct(ts) => todo!(),
            TypeInfo::Tuple(t) => todo!(),
            TypeInfo::Enum(e) => todo!(),
            _ => panic!("The given type info is not a type you can implement `Component` onto!"),
        }
        height
    }

    fn view(&self) -> InspectorView {
        InspectorView::Components
    }

    fn draw(&self, canvas: &mut Canvas, inspector: &mut ResMut<Inspector>) {
        todo!()
    }
}

#[derive(Debug, Default, Component)]
pub struct InspectorData {}

pub trait InspectorValue {
    fn height(&self) -> i32 {
        20
    }

    fn height_static() -> i32
    where
        Self: Sized,
    {
        20
    }
}

impl InspectorValue for u8 {}
impl InspectorValue for u16 {}
impl InspectorValue for u32 {}
impl InspectorValue for u64 {}
impl InspectorValue for u128 {}
impl InspectorValue for usize {}
impl InspectorValue for i8 {}
impl InspectorValue for i16 {}
impl InspectorValue for i32 {}
impl InspectorValue for i64 {}
impl InspectorValue for i128 {}
impl InspectorValue for isize {}
impl InspectorValue for f32 {}
impl InspectorValue for f64 {}
impl InspectorValue for bool {}
impl InspectorValue for String {}
impl InspectorValue for &str {}
