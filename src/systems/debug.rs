use amethyst::core::ecs::{Join, System, WriteStorage, ReadStorage, Entities, LazyUpdate, WriteExpect};
use amethyst::shred::ReadExpect;
use crate::resources::{DebugText, Game, Floor, TypeFaces};
use amethyst::ui::{UiText, UiTransform, Anchor};
use crate::components::{Player, FloorTile, AI, Character, Health, Hovered, Name};
use amethyst::core::{math::base::Vector3, Transform, Parent};
use crate::util::{SCALAR, UI_Z};

pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        WriteExpect<'s, DebugText>,
        ReadExpect<'s, Game>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, Floor>,
        ReadStorage<'s, FloorTile>,
        ReadStorage<'s, AI>,
        ReadStorage<'s, Health>,
        ReadStorage<'s, Hovered>,
        ReadStorage<'s, Name>,
        ReadStorage<'s, UiTransform>,
    );

    fn run(&mut self, (
        mut debug,
        game,
        players,
        mut ui_text,
        floor,
        tiles,
        ai,
        health,
        hovereds,
        names,
        ui_transforms,
    ): Self::SystemData) {
        if let Some(text) = ui_text.get_mut(debug.phase) {
            text.text = format!("Phase: {}", game.phase.to_string());
        }
        let mut ui_hover = "None".to_string();
        for (name, _, _) in (&names, &hovereds, &ui_transforms).join() {
            ui_hover = name.get();
        }
        if let Some(text) = ui_text.get_mut(debug.ui_hover) {
            text.text = format!("UI Hover: {}", ui_hover);
        }
        for (player) in (&players).join() {
            let mut hover_entity = "Empty";
            let mut hover_hp = "-".to_string();
            if let Some(hex) = &player.input.tile {
                if let Some(tile) = floor.get(&hex) {
                    if let Some(f_tile) = tiles.get(tile) {
                        if let Some(entity) = f_tile.character {
                            if players.get(entity).is_some() {
                                hover_entity = "Player";
                            } else if ai.get(entity).is_some() {
                                hover_entity = "AI";
                            }
                            if let Some(hp) = health.get(entity) {
                                if hp.temp > 0 {
                                    hover_hp = format!("{}/{}/{}", hp.temp, hp.current, hp.max);
                                } else {
                                    hover_hp = format!("{}/{}", hp.current, hp.max);
                                }
                            }
                        }
                    }
                }
                if let Some(text) = ui_text.get_mut(debug.hover) {
                    text.text = format!("({},{}): {}", hex.x, hex.y, hover_entity);
                }
                if let Some(text) = ui_text.get_mut(debug.hover_hp) {
                    text.text = format!("Health: {}", hover_hp);
                }
            }
        }
    }
}