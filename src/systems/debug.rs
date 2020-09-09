use amethyst::core::ecs::{Join, System, WriteStorage, ReadStorage};
use amethyst::shred::ReadExpect;
use crate::resources::{DebugText, Game, Floor};
use amethyst::ui::UiText;
use crate::components::{Player, FloorTile, AI};

pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        ReadExpect<'s, DebugText>,
        ReadExpect<'s, Game>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, Floor>,
        ReadStorage<'s, FloorTile>,
        ReadStorage<'s, AI>,
    );

    fn run(&mut self, (
        debug,
        game,
        players,
        mut ui_text,
        floor,
        tiles,
        ai,
    ): Self::SystemData) {
        if let Some(text) = ui_text.get_mut(debug.phase) {
            text.text = format!("Phase: {}", game.phase.to_string());
        }
        for (player) in (&players).join() {
            let mut hover_entity = "Empty";
            if let Some(hex) = &player.input.tile {
                if let Some(tile) = tiles.get(floor.get(&hex)) {
                    if let Some(entity) = tile.character {
                        if players.get(entity).is_some() {
                            hover_entity = "Player";
                        } else if ai.get(entity).is_some() {
                            hover_entity = "AI";
                        }
                    }
                }
                if let Some(text) = ui_text.get_mut(debug.hover) {
                    text.text = format!("({},{}): {}", hex.x, hex.y, hover_entity);
                }
            }
        }
    }
}