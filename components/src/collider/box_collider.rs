use crate::collider::convex_mesh::ConvexMesh;
use crate::collider::mesh_renderer::MeshRenderer;
use crate::collider::Collider;
use bevy_ecs::bundle::Bundle;
use engine::space;
use engine::space::Vector2;
use ggez::graphics;
use log::trace;
use mint::Point2;

#[derive(Debug, Bundle)]
pub struct BoxCollider {
    pub collider: Collider,
    pub renderer: MeshRenderer,
}

impl BoxCollider {
    pub fn new(scale: Vector2) -> Self {
        let bounds = graphics::Rect {
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

        let mesh = ConvexMesh::new(vertices);

        let collider = Collider::new(vec![Box::new(mesh)]);

        let mesh_renderer = MeshRenderer::new_with_param(graphics::DrawParam {
            src: bounds,
            color: graphics::Color::YELLOW,
            transform: graphics::Transform::Values {
                dest: mint::Point2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                scale: mint::Vector2 { x: 1.0, y: 1.0 },
                offset: Point2 { x: 0.0, y: 0.0 },
            },
            z: 0,
        });

        trace!("Created new BoxCollider");

        Self {
            collider,
            renderer: mesh_renderer,
        }
    }
}
