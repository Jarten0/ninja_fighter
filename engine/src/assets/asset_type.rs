use super::super::RenderType;
use crate::root::GameRoot;
use crate::GgezInterface;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::World;
use ggez::filesystem::Filesystem;

fn e(root: &mut GameRoot) {
    root.context();
}

#[derive(Debug)]
pub enum AssetType {
    Render(RenderType),
    Entity,
}
