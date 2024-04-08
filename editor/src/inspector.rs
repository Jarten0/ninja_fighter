mod button;
use bevy_ecs::{prelude::*, query};
use bevy_reflect::{Reflect, TypeInfo, TypeRegistration, TypeRegistry};
use engine::{
    scene::{self, ObjectID, SceneData, SceneManager},
    GgezInterface, Input,
};
use ggez::{
    graphics::{self, Canvas, Color, DrawParam, Drawable, FillOptions, TextFragment},
    mint::Point2,
};
use log::*;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Resource)]
pub struct Inspector {
    enabled: bool,
    width: f32,
    view: InspectorView,
    elements: InspectorElementContainer,
}

/// The container for all of the various inspector tabs
#[derive(Debug, Default)]
pub struct InspectorElementContainer {
    entities: Vec<String>,
    components_list: HashMap<Entity, HashMap<String, Box<dyn InspectorElement>>>,
}

#[derive(Debug, Resource)]
pub struct InspectorDrawInfo {
    pub next_y_position: f32,
    pub width: f32,
    pub enabled: bool,
}

impl Default for InspectorDrawInfo {
    fn default() -> Self {
        Self {
            next_y_position: Default::default(),
            width: 600.0,
            enabled: Default::default(),
        }
    }
}

impl Default for Inspector {
    fn default() -> Self {
        Self {
            enabled: false,
            width: 600.0,
            view: InspectorView::default(),
            elements: InspectorElementContainer::default(),
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
    fn draw(&self, canvas: &mut Canvas, inspector_draw_info: &mut InspectorDrawInfo);
}

pub fn update_inspector(
    query: Query<(Entity, &InspectorData)>,
    entities_without_data: Query<(Entity, &SceneData), Without<InspectorData>>,
    mut commands: Commands,
    mut inspector: ResMut<Inspector>,
    scene: Res<SceneManager>,
    input: Res<Input>,
) {
    // trace!("Updating inspector");
    if input
        .get_action("nextInspectorTab")
        .unwrap()
        .is_just_pressed()
    {
        inspector.view.next_tab();
        trace!("Now inspecting tab {:?}", &inspector.view);
    } else if input
        .get_action("previousInspectorTab")
        .unwrap()
        .is_just_pressed()
    {
        inspector.view.previous_tab();
        trace!("Now inspecting tab {:?}", &inspector.view);
    }

    for (entity, scene_data) in entities_without_data.iter() {
        info!("Found new entity: {}", scene_data.object_name);
        trace!("Debug info: {:?}", scene_data);
        commands.entity(entity).insert(InspectorData {});
        inspector
            .elements
            .entities
            .push(scene_data.object_name.clone());

        for (_, component_path) in &scene_data.component_paths {
            let mut component_data = HashMap::new();

            let type_info = scene
                .type_registry
                .get_with_type_path(&component_path)
                .unwrap()
                .type_info();

            component_data.insert(
                component_path.to_string(),
                Box::new(ComponentInspectorElement::new(
                    entity,
                    type_info.to_owned(),
                    component_path.to_string(),
                )) as Box<dyn InspectorElement>,
            );
            inspector
                .elements
                .components_list
                .insert(entity, component_data);
            trace!("Inserted {}", component_path);
        }
    }

    for (entity, scene_data) in query.iter() {}
}

pub fn draw_inspector(
    mut inspector: ResMut<Inspector>,
    mut engine: ResMut<GgezInterface>,
    input: Res<Input>,
) {
    // trace!("Drawing inspector");

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

#[inline]
fn inspector_draw_components(
    mut inspector: ResMut<Inspector>,
    mut engine: ResMut<GgezInterface>,
    input: Res<Input>,
) {
    let mut inspector_draw_info = InspectorDrawInfo::default();

    for (_entity, element_list) in &inspector.elements.components_list {
        let text = graphics::Text::new(TextFragment::new("Inspecting components of entity"));

        text.draw(
            engine.get_canvas_mut().unwrap(),
            DrawParam::new().dest(Point2 {
                x: 1320.0,
                y: inspector_draw_info.next_y_position,
            }),
        );
        inspector_draw_info.next_y_position += 20.0;

        for (path, element) in element_list {
            element.draw(engine.get_canvas_mut().unwrap(), &mut inspector_draw_info);

            inspector_draw_info.next_y_position += 20.0;
        }
    }
}

#[inline]
fn inspector_draw_entities(
    mut inspector: ResMut<Inspector>,
    mut engine: ResMut<GgezInterface>,
    input: Res<Input>,
) {
    let mut inspector_draw_info = InspectorDrawInfo::default();
    for id in &inspector.elements.entities {
        let text = graphics::Text::new(TextFragment::new(id));

        text.draw(
            engine.get_canvas_mut().unwrap(),
            DrawParam::new().dest(Point2 {
                x: 1320.0,
                y: inspector_draw_info.next_y_position,
            }),
        );
        inspector_draw_info.next_y_position += 20.0;
    }
}

#[derive(Debug)]
pub struct ComponentInspectorElement {
    pub entity: Entity,
    type_info: TypeInfo,
    component_path: String,
}

impl ComponentInspectorElement {
    pub fn new(entity: Entity, type_info: TypeInfo, path: String) -> Self {
        Self {
            entity,
            type_info,
            component_path: path,
        }
    }
}

impl InspectorElement for ComponentInspectorElement {
    fn get_height(&self) -> f32 {
        let mut height = 30.0;
        match &self.type_info {
            TypeInfo::Struct(s) => {
                for field in s.iter() {
                    if field.is::<&dyn InspectorValue>() {
                        height += 20.0;
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

    fn draw(&self, canvas: &mut Canvas, inspector: &mut InspectorDrawInfo) {
        trace!("Drawing {:?}", self.component_path);
        match &self.type_info {
            TypeInfo::Struct(s) => {
                for field in s.field_names() {
                    let text = (*field).to_owned() + ":" + "  todo!()";
                    let fragment = TextFragment::new(text);

                    let drawable = graphics::Text::new(fragment);

                    canvas.draw(&drawable, DrawParam::new())
                }
            }
            TypeInfo::TupleStruct(_) => todo!(),
            TypeInfo::Tuple(_) => todo!(),
            TypeInfo::Enum(_) => todo!(),
            _ => panic!("The given type info is not a type you can implement `Component` onto!"),
        }
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
