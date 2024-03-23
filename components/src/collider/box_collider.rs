use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use engine::space::{self, Vector2};
use ggez::graphics::{self, Color, DrawParam, Rect};
use log::trace;
use mint::Point2;

use crate::collider::convex_mesh::ConvexMesh;

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BoxCollider;

impl BoxCollider {
    pub fn new(scale: Vector2) -> (BoxCollider, ConvexMesh) {
        // let mut builder = ggez::graphics::MeshBuilder::new();

        let bounds = Rect {
            x: 0.0,
            y: 0.0,
            w: 1.1 * scale.x,
            h: 1.1 * scale.y,
        };

        let vertices: Vec<space::Vertex> = vec![
            engine::space::ZERO.into(),
            (engine::space::RIGHT * scale.x).into(),
            Vector2 {
                x: 1.0 * scale.x,
                y: 1.0 * scale.y,
            }
            .into(),
            space::Vertex::from(Vector2 {
                x: 0.0,
                y: 1.0 * scale.y,
            }),
        ];

        trace!("Created new BoxCollider");

        let mut mesh = ConvexMesh::new(vertices);

        mesh.debug_draw_param = Some(DrawParam {
            src: bounds,
            color: Color::YELLOW,
            transform: graphics::Transform::Values {
                dest: mint::Point2 { x: 500.0, y: 5.0 },
                rotation: 0.0,
                scale: mint::Vector2 { x: 10.0, y: 10.0 },
                offset: Point2 { x: 0.0, y: 0.0 },
            },
            z: 0,
        });

        mesh.build_indices()
            .map_err(|err| format!("Invalid indices build: {}", err))
            .unwrap();

        (Self, mesh)
    }
}
