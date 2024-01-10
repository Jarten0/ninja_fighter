#![allow(unused)]
use crate::engine::space;

use crate::engine::space::Vertex;

#[derive(Debug, Default, Clone)]
pub struct MetaVertex {
    pub(crate) next_vertex: Option<Box<MetaVertex>>,
    pub(crate) vertex: Vertex,
}

impl MetaVertex {
    pub fn collect(&mut self) -> Vec<space::Vertex> {
        let mut vec = Vec::new();

        match &mut self.next_vertex {
            Some(v) => vec.append(&mut v.collect()),
            None => (),
        };

        vec
    }
}
