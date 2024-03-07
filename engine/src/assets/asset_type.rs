use super::super::RenderType;

#[derive(Debug)]
pub enum AssetType {
    Render(RenderType),
    Entity,
}
