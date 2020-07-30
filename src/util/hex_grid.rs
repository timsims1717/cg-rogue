use crate::util::SCALED_TILE_SIZE;
use amethyst::window::ScreenDimensions;
use amethyst::core::{
    math::{Point3,Vector2},
    Transform,
};
use amethyst::renderer::Camera;

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