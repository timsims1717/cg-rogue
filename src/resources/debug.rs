use amethyst::core::ecs::Entity;
use std::collections::HashMap;
use uuid::Uuid;

/// DebugText contains the ui text components that display debug information
pub struct DebugText {
    pub phase: Entity,
    pub ui_hover: Entity,
    pub hover: Entity,
    pub hover_hp: Entity,
}