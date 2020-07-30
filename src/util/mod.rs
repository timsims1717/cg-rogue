mod hex_grid;

use amethyst::{
    core::{
        math::{Point3,Vector2},
        Transform,
    },
    renderer::camera::Camera,
    window::ScreenDimensions,
};

pub use self::hex_grid::*;

/* Constants */

pub const TILE_SIZE: f32 = 24.;
pub const SCALAR: f32 = 4.;
pub const SCALED_TILE_SIZE: f32 = TILE_SIZE * SCALAR;

/* Z Constants */

pub const CAMERA_Z: f32 = 1.0;
pub const CHAR_Z: f32 = 0.5;
pub const TILE_UI_Z: f32 = 0.3;
pub const TILE_OBJ_Z: f32 = 0.2;
pub const TILE_Z: f32 = 0.1;
pub const BASE_Z: f32 = 0.0;

// returns true if a tile's coordinates are within the size of the map
pub fn tile_exists(x: isize, y: isize, width: isize, height: isize) -> bool {
    x >= 0 && y >= 0 && x < width && y < height
}