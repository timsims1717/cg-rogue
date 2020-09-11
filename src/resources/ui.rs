use amethyst::{
    ecs::Entity,
    renderer::SpriteRender,
};
use amethyst::ui::FontHandle;

pub struct UISprites {
    pub set: Vec<SpriteRender>
}

pub struct CameraHandle {
    pub camera: Entity
}

pub struct TypeFaces {
    pub debug: FontHandle
}