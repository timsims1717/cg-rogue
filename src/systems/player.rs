use amethyst::core::ecs::{Join, System, WriteExpect, ReadExpect, Entities, ReadStorage, LazyUpdate, WriteStorage};
use crate::resources::Phase::{PlayerDecisionPhase, PlayerActionPhase, AIActionPhase};
use crate::resources::{Game, UISprites, Floor};
use crate::entities::{create_tile_ui, TileUI};
use crate::util::{shortest_path, PathEnds, in_range};
use crate::components::ActionOption::{Move, Interact, Attack};
use crate::components::{MovementAction, Character, AttackAction, HexCoords, Player, Action};

pub struct TileSelectSystem;

impl<'s> System<'s> for TileSelectSystem {
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, Game>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Character>,
        ReadStorage<'s, HexCoords>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, UISprites>,
        ReadExpect<'s, Floor>,
    );

    fn run(&mut self, (
        entities,
        mut game,
        mut players,
        mut characters,
        hexes,
        lazy_update,
        ui_sprites,
        floor,
    ): Self::SystemData) {
        for (_, character, hex, player) in (&entities, &mut characters, &hexes, &mut players).join() {
            if let Some(input_hex) = player.input.tile.clone() {
                if game.phase == PlayerDecisionPhase {
                    let mut new_mode = player.mode.clone();
                    let path_ends = PathEnds {
                        a: hex.clone(),
                        b: input_hex.clone(),
                    };
                    match &player.mode {
                        Interact => {
                            create_tile_ui(&entities, &ui_sprites.set, input_hex.x, input_hex.y, true, TileUI::Normal, &lazy_update);
                        },
                        Move(m) => {
                            if in_range(&path_ends, m.range, &floor) {
                                if let Some(path) = shortest_path(path_ends, &floor) {
                                    for h in path.iter() {
                                        create_tile_ui(&entities, &ui_sprites.set, h.x, h.y, true, TileUI::Move, &lazy_update);
                                    }
                                    if player.input.l_click {
                                        if *hex == input_hex {
                                            character.actions.push(Action::NoAction{});
                                        } else {
                                            character.actions.push(Action::Move(MovementAction::new(path, (*m).clone())));
                                        }
                                        new_mode = Interact;
                                        game.phase = PlayerActionPhase;
                                    }
                                }
                            } else {
                                create_tile_ui(&entities, &ui_sprites.set, input_hex.x, input_hex.y, true, TileUI::Nope, &lazy_update);
                            }
                        },
                        Attack(a) => {
                            if in_range(&path_ends, a.range, &floor) {
                                let mut tile_group = vec![HexCoords { x: input_hex.x, y: input_hex.y }];
                                if let Some(area) = &a.area {
                                    tile_group.append(&mut area.area.clone());
                                    if area.path {
                                        if let Some(mut path) = shortest_path(path_ends, &floor, ) {
                                            tile_group.append(&mut path);
                                        }
                                    }
                                }
                                for h in tile_group.iter() {
                                    create_tile_ui(&entities, &ui_sprites.set, h.x, h.y, true, TileUI::Attack, &lazy_update);
                                }
                                if player.input.l_click {
                                    character.actions.push(Action::Attack(AttackAction::new(tile_group, (*a).clone())));
                                    new_mode = Interact;
                                    game.phase = PlayerActionPhase;
                                }
                            } else {
                                create_tile_ui(&entities, &ui_sprites.set, input_hex.x, input_hex.y, true, TileUI::Nope, &lazy_update);
                            }
                        }
                    }
                    player.mode = new_mode;
                } else {
                    create_tile_ui(&entities, &ui_sprites.set, input_hex.x, input_hex.y, true, TileUI::Normal, &lazy_update);
                }
            }
        }
    }
}