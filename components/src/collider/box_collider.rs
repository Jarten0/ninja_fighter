<<<<<<< Updated upstream
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use engine::space::{self, Vector2};
use ggez::graphics::{self, Color, DrawParam, Rect};
=======
use bevy_ecs::bundle::Bundle;
use engine::space;
use engine::space::Vector2;
use ggez::graphics;
>>>>>>> Stashed changes
use log::trace;
use mint::Point2;

use crate::collider::convex_mesh::ConvexMesh;
<<<<<<< Updated upstream
=======
use crate::collider::mesh_renderer::MeshRenderer;
>>>>>>> Stashed changes

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BoxCollider;

impl BoxCollider {
<<<<<<< Updated upstream
    pub fn new(scale: Vector2) -> (BoxCollider, ConvexMesh) {
        // let mut builder = ggez::graphics::MeshBuilder::new();

        let bounds = Rect {
=======
    pub fn new(scale: Vector2) -> Self {
        let bounds = graphics::Rect {
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
        ];

        trace!("Created new BoxCollider");

        let mut mesh = ConvexMesh::new(vertices);

        mesh.debug_draw_param = Some(DrawParam {
=======
        ]);

        let collider = Collider::new(vec![Box::new(mesh)]);

        let mesh_renderer = MeshRenderer::new_with_param(graphics::DrawParam {
>>>>>>> Stashed changes
            src: bounds,
            color: graphics::Color::YELLOW,
            transform: graphics::Transform::Values {
                dest: mint::Point2 { x: 500.0, y: 5.0 },
                rotation: 0.0,
                scale: mint::Vector2 { x: 10.0, y: 10.0 },
                offset: Point2 { x: 0.0, y: 0.0 },
            },
            z: 0,
        });

<<<<<<< Updated upstream
        mesh.build_indices()
            .map_err(|err| format!("Invalid indices build: {}", err))
            .unwrap();
=======
        trace!("Created new BoxCollider");
>>>>>>> Stashed changes

        (Self, mesh)
    }
}
