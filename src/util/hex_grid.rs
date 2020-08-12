use crate::util::SCALED_TILE_SIZE;
use amethyst::window::ScreenDimensions;
use amethyst::core::{
    math::{Point3,Vector2},
    Transform,
};
use amethyst::renderer::Camera;
use pathfinding::prelude::astar;
use amethyst::core::ecs::{WriteStorage, Entities};
use crate::components::FloorTile;
use crate::resources::{Floor, Pos};
use pathfinding::directed::astar::astar_bag_collect;

/* Basic Grid Functions */

pub fn map_to_world_hex(map_x: f32, map_y: f32) -> (f32, f32) {
    if map_x.floor() as usize % 2 != 0 {
        ((map_x + 0.5) * SCALED_TILE_SIZE, (map_y + 1.0) * SCALED_TILE_SIZE)
    } else {
        ((map_x + 0.5) * SCALED_TILE_SIZE, (map_y + 0.5) * SCALED_TILE_SIZE)
    }
}

pub fn world_to_map_hex(world_x: f32, world_y: f32) -> (f32, f32) {
    let map_x = (world_x / SCALED_TILE_SIZE).floor();
    if map_x.floor() as usize % 2 != 0 {
        ((world_x / SCALED_TILE_SIZE).floor(), (world_y / SCALED_TILE_SIZE - 0.5).floor())
    } else {
        ((world_x / SCALED_TILE_SIZE).floor(), (world_y / SCALED_TILE_SIZE).floor())
    }
}

pub fn mouse_to_world(mouse_x: f32, mouse_y: f32, screen_dim: &ScreenDimensions, camera: &Camera, camera_transform: &Transform) -> (f32, f32) {
    let diagonal = Vector2::new(screen_dim.width(), screen_dim.height());
    let world_point = camera.projection().screen_to_world_point(
        Point3::new(mouse_x, mouse_y, 0.0),
        diagonal,
        camera_transform,
    );
    (world_point.x, -world_point.y)
}

pub fn mouse_to_map_hex(mouse_x: f32, mouse_y: f32, screen_dim: &ScreenDimensions, camera: &Camera, camera_transform: &Transform) -> (f32, f32) {
    let (world_x, world_y) = mouse_to_world(mouse_x, mouse_y, screen_dim, camera, camera_transform);
    world_to_map_hex(world_x, world_y)
}

pub fn closest_point_in_map(x: f32, y: f32, width: usize, height: usize) -> (f32, f32) {
    let (w, h) = (width as f32 * SCALED_TILE_SIZE, height as f32 * SCALED_TILE_SIZE);
    let new_x = if x < 0.0 {
        0.0
    } else if x > w {
        w
    } else {
        x
    };
    let new_y = if y < 0.0 {
        0.0
    } else if y > h {
        h
    } else {
        y
    };
    (new_x, new_y)
}

/* Path and Shape Functions */

#[derive(Clone, Debug)]
pub struct PathEnds {
    pub a_x: usize,
    pub a_y: usize,
    pub b_x: usize,
    pub b_y: usize,
}

pub fn distance(ends: &PathEnds) -> u32 {
    let mut dist: u32 = 0;
    let (mut x, mut y) = (ends.a_x, ends.a_y);
    while x != ends.b_x {
        if x % 2 == 0 && y > ends.b_y {
            y -= 1;
        } else if x % 2 != 0 && y < ends.b_y {
            y += 1;
        }
        if x > ends.b_x {
            x -= 1;
        } else {
            x += 1;
        }
        dist += 1;
    }
    dist + (y as i32 - ends.b_y as i32).abs() as u32
}

pub fn distance_world(ends: &PathEnds) -> u32 {
    let (a_x, a_y) = map_to_world_hex(ends.a_x as f32, ends.a_y as f32);
    let (b_x, b_y) = map_to_world_hex(ends.b_x as f32, ends.b_y as f32);
    let x = (a_x - b_x);
    let y = (a_y - b_y);
    distance(ends) + ((x*x + y*y).sqrt() * 100.0) as u32
}

// find the shortest path between two tiles
pub fn shortest_path(ends: PathEnds, floor: &Floor/*, entities: &Entities*/) -> Option<Vec<(usize, usize)>> {
    // let start = Pos::new(ends.a_x, ends.a_y);
    // let end = Pos::just_point(ends.b_x, ends.b_y);
    // let results = astar_bag_collect(
    //     &start,
    //     |p| p.successors(floor),
    //     |p| p.distance(&end),
    //     |p| *p == end
    // );
    // if let Some((mut v, _)) = results {
    //     return Some(v[v.len() / 2].clone().into_iter().map(|p| (p.x, p.y)).collect());
    // }
    // None

    let start = Pos::new(ends.a_x, ends.a_y);
    let end = Pos::just_point(ends.b_x, ends.b_y);
    let result = astar(
        &start,
        |p| p.successors(floor),
        |p| p.distance_world(&end),
        |p| *p == end
    );
    if let Some((v, _)) = result {
        Some(v.into_iter().map(|p| (p.x, p.y)).collect())
    } else {
        None
    }
}

// returns true if the two tiles are within range
pub fn in_range(ends: &PathEnds, range: usize, floor: &Floor/*, entities: &Entities*/) -> bool {
    if range >= distance(ends) as usize {
        // todo: calculate floor, los, etc.
        true
    } else {
        false
    }
}