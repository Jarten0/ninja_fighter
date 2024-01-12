use core::fmt;

use engine::Engine;
use engine::Input;

use bevy_ecs::entity::Entity;

use bevy_ecs::system::Commands;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ecs::system::ResMut;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Rect;

use crate::collider::Collider;

use super::collider::collider_mesh::ColliderMesh;

pub fn init(mut commands: Commands, engine: Res<Engine>) {
    if !engine.debug {
        return;
    }
    commands.spawn(DebugComponent::new());
}

#[derive(bevy_ecs::component::Component)]
pub struct DebugComponent {
    current_place_state: PlaceState,
}

impl DebugComponent {
    fn new() -> Self {
        Self {
            current_place_state: PlaceState::Idle,
        }
    }

    fn update_place_state(&mut self, place_state: PlaceState) {
        self.current_place_state = place_state;
    }
}

pub fn update(
    mut query: Query<&mut DebugComponent>,
    mut collider_query: Query<&mut ColliderMesh>,
    engine: ResMut<Engine>,
    mut input: ResMut<Input>,
    mut commands: Commands,
) {
    println!("{:?}", input);

    let is_just_pressed = input
        .get_action("Click")
        .unwrap()
        .action_status()
        .is_just_pressed();

    for mut debug in query.iter_mut() {
        match debug.current_place_state {
            PlaceState::Idle => {
                if is_just_pressed {
                    let spawn = commands.spawn(Collider::new(&engine));

                    debug.update_place_state(PlaceState::Pending(spawn.id()));
                }
            }
            PlaceState::Pending(entity) => {
                if let Ok(_) = collider_query.get(entity) {
                    debug.update_place_state(PlaceState::Placing(0, entity))
                }
            }
            PlaceState::Placing(stage, entity) => {
                if !is_just_pressed {
                    continue;
                }
                // colliders.get(entity).unwrap().vertecies_list;
            }
        }
    }
}

enum PlaceState {
    Idle,
    /// Waiting on the bevy command to go through before changing to Placing.
    Pending(Entity),
    /// * `i32` - current stage of placement
    ///
    /// * `Entity` - the current entity being placed
    Placing(i32, Entity),
}

impl fmt::Display for PlaceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlaceState::Idle => write!(f, "Idle"),
            PlaceState::Pending(entity) => write!(f, "Pending {:#?}", entity),
            PlaceState::Placing(stage, entity) => write!(f, "Placing {}, {:#?}", stage, entity),
        }
    }
}

pub fn draw(query: Query<&ColliderMesh>, mut engine: ResMut<Engine>) {
    if !engine.debug {
        return;
    }

    for mesh in query.iter() {
        draw_vertecies(&mut engine, mesh);
    }
}

pub(super) fn draw_vertecies(engine: &mut ResMut<Engine>, mesh: &ColliderMesh) {
    let param = DrawParam {
        src: Rect::default(),
        color: Color::CYAN,
        transform: ggez::graphics::Transform::default(),
        z: 0,
    };
    engine
        .get_canvas_mut()
        .unwrap()
        .draw(mesh.get_drawable(), param)
}
