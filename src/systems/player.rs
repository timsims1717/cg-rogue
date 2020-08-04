use amethyst::core::ecs::{Join, System, WriteExpect, ReadExpect, Entities, ReadStorage, LazyUpdate, WriteStorage};
use crate::resources::Phase::{PlayerTurn, PlayerAction, AIAction};
use crate::resources::{Game, Player, UISprites, Floor};
use crate::entities::{create_tile_ui, TileUI};
use crate::util::{shortest_path, PathEnds};
use crate::components::Mode::{Move, Interact};
use crate::components::{Movement, Character};

pub struct PlayerTurnSystem;

impl<'s> System<'s> for PlayerTurnSystem {
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, Game>,
        WriteExpect<'s, Player>,
        ReadStorage<'s, Character>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, UISprites>,
        ReadExpect<'s, Floor>,
        WriteStorage<'s, Movement>,
    );

    fn run(&mut self, (
        entities,
        mut game,
        mut player,
        characters,
        lazy_update,
        ui_sprites,
        floor,
        mut movements
    ): Self::SystemData) {
        if let Some((tile_x, tile_y)) = player.input.tile {
            if game.phase == PlayerTurn {
                if let Some(character) = characters.get(player.character) {
                    let mut new_mode = player.mode.clone();
                    match &player.mode {
                        Interact => {
                            create_tile_ui(&entities, ui_sprites.set.clone(), tile_x, tile_y, true, TileUI::Normal, &lazy_update);
                        },
                        Move(m) => {
                            if let Some(tile_group) = shortest_path(
                                PathEnds {
                                    a_x: character.x,
                                    a_y: character.y,
                                    b_x: tile_x,
                                    b_y: tile_y
                                }, &floor,
                            ) {
                                for (x, y) in tile_group.iter() {
                                    create_tile_ui(&entities, ui_sprites.set.clone(), *x, *y, true, TileUI::Move, &lazy_update);
                                }
                                if player.input.l_click {
                                    movements.insert(player.character, Movement::new(tile_group, (*m).clone()));
                                    new_mode = Interact;
                                    game.phase = PlayerAction;
                                }
                            }
                        }
                    }
                    player.mode = new_mode;
                }
            } else {
                create_tile_ui(&entities, ui_sprites.set.clone(), tile_x, tile_y, true, TileUI::Normal, &lazy_update);
            }
        }
    }
}