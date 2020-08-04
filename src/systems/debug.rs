use amethyst::core::ecs::{System, WriteStorage};
use amethyst::shred::ReadExpect;
use crate::resources::{DebugText, Game};
use amethyst::ui::UiText;

pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        ReadExpect<'s, DebugText>,
        ReadExpect<'s, Game>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (
        debug,
        game,
        mut ui_text,
    ): Self::SystemData) {
        if let Some(text) = ui_text.get_mut(debug.phase) {
            text.text = format!("Phase: {}", game.phase.to_string());
        }
    }
}