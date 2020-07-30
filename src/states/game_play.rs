
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::base::Vector3, transform::{Transform}},
    ecs::prelude::Entity,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use log::info;
use crate::{
    resources::{Floor, FloorSize},
    util::{SCALAR, TILE_SIZE}
};
use crate::resources::{CameraHandle, UISprites, PreFloor};
use crate::util::{CHAR_Z, TILE_Z, map_to_world_hex};
use crate::components::Character;

pub struct GamePlayState;

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // load test map (will be generated randomly in loading state I think)
        let mut pre_floor = PreFloor::new();

        // Camera
        let camera = init_camera(world, &dimensions, &pre_floor.dimensions);
        world.insert(CameraHandle{camera});

        // Tilemap
        let tile_sprites = load_test_tiles(world);

        let char_sprites = load_char_sprites(world);

        // UI Textures
        let ui_sprites = load_ui_textures(world);
        world.insert(UISprites { set: ui_sprites });

        let floor = init_floor(world, &mut pre_floor, &tile_sprites);
        world.insert(floor);
        init_player(world, &char_sprites);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }


            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions, floor_size: &FloorSize) -> Entity {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let (center_x, center_y) = (floor_size.width as f32 * TILE_SIZE * SCALAR / 2., floor_size.height as f32 * TILE_SIZE * SCALAR / 2.);
    let mut transform = Transform::default();
    // reverse y to put origin at top of map
    transform.set_translation_xyz(center_x, -center_y, 1.);

    world.create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build()
}

/* loading test tiles */

fn load_test_tiles(world: &mut World) -> Vec<SpriteRender> {
    // tile textures
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "tiles/testfloor.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "tiles/testfloor.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    (0..1)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

fn load_char_sprites(world: &mut World) -> Vec<SpriteRender> {
    // tile textures
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "characters/testman.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "characters/testman.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    (0..1)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

pub fn load_ui_textures(world: &mut World) -> Vec<SpriteRender> {
    // ui textures
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "ui/testui.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "ui/testui.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    (0..1)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

/* loading floor */

fn init_floor(world: &mut World, pre_floor: &mut PreFloor, tile_sprites: &[SpriteRender]) -> Floor {
    let mut floor = Floor::new(pre_floor.dimensions.clone());
    for (x, row) in pre_floor.tiles.iter().enumerate() {
        for (y, t) in row.iter().enumerate() {
            let (world_x, world_y) = map_to_world_hex(x as f32, y as f32);

            let mut transform = Transform::default();
            transform.set_scale(Vector3::new(SCALAR, SCALAR, 0.));
            transform.set_translation_xyz(world_x, -world_y, TILE_Z);

            floor.append(
                world.create_entity()
                    .with(tile_sprites[t.tile_index].clone())
                    .with(t.clone())
                    .with(transform)
                    .build()
            )
        }
    }
    floor
}

fn init_player(world: &mut World, char_sprites: &[SpriteRender]) {
    let (world_x, world_y) = map_to_world_hex(4 as f32, 7 as f32);
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALAR, SCALAR, 0.0));
    transform.set_translation_xyz(world_x, -world_y, CHAR_Z);

    world.create_entity()
        .with(char_sprites[0].clone())
        .with(Character{
            x: 4,
            y: 7,
        })
        .with(transform)
        .build();
}
