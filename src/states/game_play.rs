
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::base::Vector3, transform::{Transform}},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use log::info;

pub struct GamePlayState;

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // load test map (will be generated randomly in loading state I think)
        let mut floor = load_test_floor();
        world.insert(floor.clone());

        // Camera
        init_camera(world, &dimensions, &floor.dimensions);
        // world.insert(CameraHandle{camera});

        // Tilemap
        let tile_sprites = load_test_tiles(world);

        init_floor(world, &mut floor, &tile_sprites)
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
    let (center_x, center_y) = (floor_size.width as f32 * 24. * 4. / 2., floor_size.height as f32 * 24. * 4. / 2.);
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

/* loading floor */

fn init_floor(world: &mut World, floor: &mut Floor, tile_sprites: &[SpriteRender]) {
    for (y, row) in floor.tiles.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            let scalar = 4.;
            let (world_x, world_y) = (x as f32 * 24. * scalar, y as f32 * 24. * scalar);

            let mut transform = Transform::default();
            transform.set_scale(Vector3::new(scalar, scalar, 0.));
            transform.set_translation_xyz(world_x, -world_y, 0.1);

            world.create_entity()
                .with(tile_sprites[t.tile_index].clone())
                .with(transform)
                .build();
        }
    }
}

/* loading test floor */

fn load_test_floor() -> Floor {
    return Floor{
        dimensions: FloorSize{
            width: 10,
            height: 10,
        },
        tiles: vec![vec![FloorTile{
            tile_index: 0,
        }; 10]; 10]
    }
}

#[derive(Debug, Clone)]
pub struct Floor {
    pub dimensions: FloorSize,
    pub tiles: Vec<Vec<FloorTile>>,
}

#[derive(Default, Debug, Clone)]
pub struct FloorSize {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub struct FloorTile {
    pub tile_index: usize,
}

impl Component for FloorTile {
    type Storage = DenseVecStorage<Self>;
}