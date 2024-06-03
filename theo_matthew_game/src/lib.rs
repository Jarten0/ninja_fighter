use std::fs::File;
use std::path::{Path, PathBuf};

use bevy_ecs::component::Component;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::schedule::{IntoSystemConfigs, Schedule};
use bevy_ecs::world::World;
use bevy_reflect::Reflect;
use components::schedules::{frame, tick};

use serde::{Deserialize, Serialize};

pub fn init_world(world: &mut World) {
    let mut schedule = Schedule::new(engine::schedule::ScheduleTag::Tick);
    schedule.add_systems((
        tick::collider_update,
        tick::protag_update,
        tick::renderer_update,
    ));
    world.add_schedule(schedule);

    let mut schedule = Schedule::new(engine::schedule::ScheduleTag::Frame);
    schedule.add_systems(
        (
            frame::mesh_renderer_draw,
            frame::render_text_renderers,
            frame::renderer_draw,
        )
            .chain(),
    );
    world.add_schedule(schedule);

    components::initialize_component_types(world);

    editor::init_editor_resources(world);

    world.add_schedule(editor::debug_tick_schedule());
    world.add_schedule(editor::debug_frame_schedule());
    world.add_schedule(editor::debug_init_schedule());
    world.add_schedule(editor::debug_gui_schedule());
}

#[derive(Debug, Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TextDocumentFeeder {
    #[reflect(ignore)]
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub document: Option<File>,
    pub sequences: Vec<>
}

impl TextDocumentFeeder {
    pub fn start_parsing_file(&mut self, path: &Path) {
        self.document = File::options().open(path).ok();
    }

    pub fn get_next_sequence(&mut self){
        todo!()
    }
}

pub struct TextSequence {
    name: String,
    lines
}

#[derive(Debug, Serialize, Deserialize, Reflect)]
pub struct Line {
    raw_text: String,
}


