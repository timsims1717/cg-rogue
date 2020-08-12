use amethyst::core::ecs::{Join, System, WriteExpect, ReadExpect, Entities, ReadStorage, LazyUpdate, WriteStorage};
use crate::resources::Phase::{PlayerDecisionPhase, PlayerActionPhase, AIActionPhase};
use crate::resources::{Game, Player, UISprites, Floor};
use crate::entities::{create_tile_ui, TileUI};
use crate::util::{shortest_path, PathEnds, in_range};
use crate::components::ActionOption::{Move, Interact, Attack};
use crate::components::{MovementAction, Character, AttackAction};

pub struct TileSelectSystem;

impl<'s> System<'s> for TileSelectSystem {
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, Game>,
        WriteExpect<'s, Player>,
        ReadStorage<'s, Character>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, UISprites>,
        ReadExpect<'s, Floor>,
        WriteStorage<'s, MovementAction>,
        WriteStorage<'s, AttackAction>,
    );

    fn run(&mut self, (
        entities,
        mut game,
        mut player,
        characters,
        lazy_update,
        ui_sprites,
        floor,
        mut movements,
        mut attacks,
    ): Self::SystemData) {
        if let Some((tile_x, tile_y)) = player.input.tile {
            if game.phase == PlayerDecisionPhase {
                if let Some(character) = characters.get(player.character) {
                    let mut new_mode = player.mode.clone();
                    let path_ends = PathEnds {
                        a_x: character.x,
                        a_y: character.y,
                        b_x: tile_x,
                        b_y: tile_y
                    };
                    match &player.mode {
                        Interact => {
                            create_tile_ui(&entities, &ui_sprites.set, tile_x, tile_y, true, TileUI::Normal, &lazy_update);
                        },
                        Move(m) => {
                            if in_range(&path_ends, m.range, &floor) {
                                if let Some(path) = shortest_path(path_ends, &floor, ) {
                                    for (x, y) in path.iter() {
                                        create_tile_ui(&entities, &ui_sprites.set, *x, *y, true, TileUI::Move, &lazy_update);
                                    }
                                    if player.input.l_click {
                                        movements.insert(player.character, MovementAction::new(path, (*m).clone()));
                                        new_mode = Interact;
                                        game.phase = PlayerActionPhase;
                                    }
                                }
                            } else {
                                create_tile_ui(&entities, &ui_sprites.set, tile_x, tile_y, true, TileUI::Nope, &lazy_update);
                            }
                        },
                        Attack(a) => {
                            if in_range(&path_ends, a.range, &floor) {
                                let mut tile_group = vec![(tile_x, tile_y)];
                                tile_group.append(&mut a.area.clone());
                                if a.path {
                                    if let Some(mut path) = shortest_path(path_ends, &floor, ) {
                                        tile_group.append(&mut path);
                                    }
                                }
                                for (x, y) in tile_group.iter() {
                                    create_tile_ui(&entities, &ui_sprites.set, *x, *y, true, TileUI::Attack, &lazy_update);
                                }
                                if player.input.l_click {
                                    attacks.insert(player.character, AttackAction::new(tile_group, (*a).clone()));
                                    new_mode = Interact;
                                    game.phase = PlayerActionPhase;
                                }
                            } else {
                                create_tile_ui(&entities, &ui_sprites.set, tile_x, tile_y, true, TileUI::Nope, &lazy_update);
                            }
                        }
                    }
                    player.mode = new_mode;
                }
            } else {
                create_tile_ui(&entities, &ui_sprites.set, tile_x, tile_y, true, TileUI::Normal, &lazy_update);
            }
        }
    }
}