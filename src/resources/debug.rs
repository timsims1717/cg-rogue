use amethyst::core::ecs::Entity;

/// DebugText contains the ui text components that display debug information
pub struct DebugText {
    pub phase: Entity,
    pub hover: Entity,
}