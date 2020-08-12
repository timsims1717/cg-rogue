use amethyst::{
    core::{math::base::Vector3, Transform},
    ecs::{Entities, LazyUpdate, ReadExpect},
    renderer::SpriteRender,
};

use crate::components::{TileUIElement};
use crate::util::{TILE_UI_Z, SCALAR, map_to_world_hex};

// creates a ui element for a tile
pub fn create_tile_ui(
    entities: &Entities,
    sprites: &Vec<SpriteRender>,
    tile_x: usize,
    tile_y: usize,
    hover: bool,
    ui_type: TileUI,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let ui_entity = entities.create();
    let (world_x, world_y) = map_to_world_hex(tile_x as f32, tile_y as f32);
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALAR, SCALAR, 0.));
    transform.set_translation_xyz(world_x, -world_y, TILE_UI_Z);

    let ui_element = TileUIElement {
        x: tile_x,
        y: tile_y,
        hover,
    };

    let i = match ui_type {
        TileUI::Normal => 0,
        TileUI::Move => 1,
        TileUI::Attack => 2,
        TileUI::Nope => 3,
        _ => 4,
    };

    lazy_update.insert(ui_entity, ui_element);
    lazy_update.insert(ui_entity, transform);
    lazy_update.insert(ui_entity, sprites[i].clone());
}

// todo: how to stack UI? ie, move + attack

#[allow(dead_code)]
pub enum TileUI{
    Normal,  // gray
    Move,    // blue
    Move2,   // lighter blue
    Attack,  // red
    Attack2, // orange-red
    Debuff,  // yellow
    Debuff2, // lighter yellow
    Buff,    // green
    Buff2,   // lighter green
    Misc,    // ?
    Nope,    // red w/cross
    Unknown  // ?
}