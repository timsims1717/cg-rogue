use amethyst::core::ecs::{Join, System, ReadExpect, ReadStorage, WriteStorage, Read, Entities};
use amethyst::core::{Transform, Time};
use amethyst::renderer::Camera;
use amethyst::window::ScreenDimensions;
use amethyst::input::{StringBindings, InputHandler};
use crate::resources::{Floor};
use crate::components::Character;
use crate::util::{world_to_map_hex, map_to_world_hex, closest_point_in_map};

pub struct CameraMovementSystem;

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Character>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, Floor>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        screen_dimensions,
        cameras,
        characters,
        mut transforms,
        input_handler,
        floor,
        time,
    ): Self::SystemData) {
        // todo: scroll_sensitivity will be an option
        let scroll_sensitivity = 480.;
        let delta_time = time.delta_real_seconds();
        let move_factor = scroll_sensitivity * delta_time;
        // for the only camera
        for (_, transform) in (&cameras, &mut transforms).join() {
            // move camera if keys are pressed
            if input_handler.action_is_down("CameraMoveUp").unwrap() {
                transform.move_up(move_factor);
            }
            if input_handler.action_is_down("CameraMoveDown").unwrap() {
                transform.move_down(move_factor);
            }
            if input_handler.action_is_down("CameraMoveLeft").unwrap() {
                transform.move_left(move_factor);
            }
            if input_handler.action_is_down("CameraMoveRight").unwrap() {
                transform.move_right(move_factor);
            }
            // snap camera to edge of map
            let (x, y) = closest_point_in_map(transform.translation().x, -transform.translation().y, floor.dimensions.width, floor.dimensions.height);
            transform.set_translation_x(x);
            transform.set_translation_y(-y);
        }
    }
}

pub struct WindowResizeSystem {
    last_dimensions: ScreenDimensions,
}

impl WindowResizeSystem {
    pub fn new() -> Self {
        Self {
            last_dimensions: ScreenDimensions::new(0, 0, 0.0),
        }
    }
}

impl<'s> System<'s> for WindowResizeSystem {
    type SystemData = (ReadExpect<'s, ScreenDimensions>, WriteStorage<'s, Camera>);

    fn run(&mut self, (screen_dimensions, mut cameras): Self::SystemData) {
        // prevents the contents of the window from scaling with screen size
        if self.last_dimensions.width() == 0. || self.last_dimensions.height() == 0. {
            self.last_dimensions = screen_dimensions.clone();
        } else if self.last_dimensions != *screen_dimensions {
            for camera in (&mut cameras).join() {
                if let Some(ortho) = camera.projection_mut().as_orthographic_mut() {
                    ortho.set_bottom_and_top(screen_dimensions.height() * -0.5, screen_dimensions.height() * 0.5);
                    ortho.set_left_and_right(screen_dimensions.width() * -0.5, screen_dimensions.width() * 0.5);
                }
            }

            self.last_dimensions = screen_dimensions.clone();
        }
    }
}