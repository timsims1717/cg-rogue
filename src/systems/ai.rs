use amethyst::core::ecs::{Join, System, ReadStorage, Entities, ReadExpect, LazyUpdate, WriteStorage};
use crate::components::{AI, Character, Movement, FloorTile};
use crate::resources::{Player, Game, Floor};
use amethyst::core::shred::WriteExpect;
use crate::resources::Phase::{AITurn, PlayerTurn, AIAction};
use rand::{thread_rng, Rng};
use crate::components::Mode::Move;
use crate::util::{shortest_path, PathEnds};

pub struct AITurnSystem;

impl<'s> System<'s> for AITurnSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AI>,
        ReadStorage<'s, Character>,
        ReadExpect<'s, Player>,
        WriteExpect<'s, Game>,
        ReadExpect<'s, Floor>,
        ReadStorage<'s, FloorTile>,
        WriteStorage<'s, Movement>,
    );

    fn run(&mut self, (
        entities,
        all_ai,
        characters,
        player,
        mut game,
        floor,
        tiles,
        mut movements,
    ): Self::SystemData) {
        if game.phase == AITurn {
            let pc = characters.get(player.character).unwrap();
            for (entity, ai, character) in (&entities, &all_ai, &characters).join() {
                if !ai.actions.is_empty() {
                    let choice = &ai.actions[thread_rng().gen_range(0, ai.actions.len())];
                    match choice {
                        Move(m) => {
                            if let Some(mut path) = shortest_path(PathEnds{
                                a_x: character.x, a_y: character.y, b_x: pc.x, b_y: pc.y
                            }, &floor) {
                                path.truncate(m.range);
                                movements.insert(entity, Movement::new(path, m.clone()));
                            }
                        },
                        _ => {}
                    }
                }
            }
            game.phase = PlayerTurn;
        }
    }
}