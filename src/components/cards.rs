use amethyst::core::ecs::{Component, DenseVecStorage};
use amethyst::ui::UiTransform;

pub const HAND_SCALE: f32 = 0.5;
pub const HAND_Y: f32 = 200.;
pub const HAND_CEN: f32 = 200.;

pub const HAND_HOVER_SCALE: f32 = 1.;
pub const HAND_HOVER_Y: f32 = 400.;

#[derive(Debug, Clone)]
pub struct Card {

}

impl Card {
    pub fn new() -> Card {
        Card{

        }
    }

    pub fn hover(transform: UiTransform) {
        
    }

    pub fn unhover(transform: UiTransform) {

    }
}

impl Component for Card {
    type Storage = DenseVecStorage<Self>;
}