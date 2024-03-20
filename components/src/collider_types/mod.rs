use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use engine::space::{self, Vector2};
use ggez::{
    context::Has,
    graphics::{self, Color, DrawParam, FillOptions, GraphicsContext, Rect, StrokeOptions},
};
use log::trace;
use mint::Point2;

use crate::collider::collider_mesh::ColliderMesh;

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BoxCollider;

impl BoxCollider {
    pub fn new(scale: Vector2) -> (BoxCollider, ColliderMesh) {
        // let mut builder = ggez::graphics::MeshBuilder::new();

        let bounds = Rect {
            x: 0.0,
            y: 0.0,
            w: 1.1 * scale.x,
            h: 1.1 * scale.y,
        };

        let vertices: Vec<space::Vertex> = vec![
            space::Vertex::from(Vector2 { x: 0.0, y: 0.0 }),
            space::Vertex::from(Vector2 {
                x: 1.0 * scale.x,
                y: 0.0,
            }),
            space::Vertex::from(Vector2 {
                x: 1.0 * scale.x,
                y: 1.0 * scale.y,
            }),
            space::Vertex::from(Vector2 {
                x: 0.0,
                y: 1.0 * scale.y,
            }),
        ];

        // let fill = FillOptions::DEFAULT.with_fill_rule(graphics::FillRule::NonZero);
        // let mode = graphics::DrawMode::Fill(fill);

        // builder
        //     .rectangle(mode, bounds, Color::from_rgb(224, 224, 224))
        //     .unwrap();

        // builder
        //     .line(
        //         &[
        //             Point2 { x: 0.0, y: 0.0 },
        //             Point2 {
        //                 x: 1000.0,
        //                 y: 800.0,
        //             },
        //             // Point2 { x: 80.0, y: 100.0 },
        //             // Point2 { x: 40.0, y: 200.0 },
        //         ],
        //         20.0,
        //         Color::BLUE,
        //     )
        //     .unwrap();

        // let mesh_data = builder.build();

        trace!("Created new BoxCollider");

        let mut mesh = ColliderMesh::new(vertices);

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

        (Self, dbg!(mesh))
    }
}
