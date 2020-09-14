
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
use crate::resources::{CameraHandle, UISprites, PreFloor, Game, DebugText, TypeFaces};
use crate::util::{CHAR_Z, TILE_Z, map_to_world_hex, UI_Z};
use crate::components::{Character, AI, MovementOptions, ActionOption, AIActionOption, AIActionOptionSeq, AITree, AITargetDecision, AttackOptions, AIRequire, AITargetChoice, Diplomacy, HexCoords, Player, Health, DamageType, ID, Name};
use crate::components::ActionOption::Interact;
use amethyst::ui::{TtfFormat, UiTransform, Anchor, UiText, UiImage};
use std::collections::HashMap;

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
        let card_sprites = load_card_textures(world);

        // UI Textures
        let ui_sprites = load_ui_textures(world);
        world.insert(UISprites { set: ui_sprites });

        let floor = init_floor(world, &mut pre_floor, &tile_sprites, &char_sprites);
        world.insert(floor);
        world.insert(Game::new());
        init_debug(world);
        init_cards(world, &card_sprites);
    }

    fn handle_event(
        &mut self,
        mut data: StateData<'_, GameData<'_, '_>>,
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

/* loading test textures */

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

    (0..5)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

pub fn load_card_textures(world: &mut World) -> Vec<SpriteRender> {
    // ui textures
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "cards/testcard.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "cards/testcard.ron",
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

fn init_floor(world: &mut World, pre_floor: &mut PreFloor, tile_sprites: &[SpriteRender], char_sprites: &[SpriteRender]) -> Floor {
    let mut floor = Floor::new(pre_floor.dimensions.clone());
    for (y, row) in pre_floor.tiles.iter_mut().enumerate() {
        for (x, t) in row.iter_mut().enumerate() {
            let (world_x, world_y) = map_to_world_hex(x as f32, y as f32);

            let mut transform = Transform::default();
            transform.set_scale(Vector3::new(SCALAR, SCALAR, 0.));
            transform.set_translation_xyz(world_x, -world_y, TILE_Z);

            if x == 4 && y == 7 {
                t.character = Some(init_player(world, char_sprites))
            } else if x == 9 && y == 6 {
                t.character = Some(init_enemy(world, char_sprites))
            }

            floor.append(
                world.create_entity()
                    .with(tile_sprites[t.sprite_index].clone())
                    .with(t.clone())
                    .with(transform)
                    .build()
            )
        }
    }
    floor
}

fn init_player(world: &mut World, char_sprites: &[SpriteRender]) -> Entity {
    let (world_x, world_y) = map_to_world_hex(4 as f32, 7 as f32);
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALAR, SCALAR, 0.0));
    transform.set_translation_xyz(world_x, -world_y, CHAR_Z);

    world.create_entity()
        .with(char_sprites[0].clone())
        .with(Character::new())
        .with(ID::new())
        .with(HexCoords{ x: 4, y: 7 })
        .with(transform)
        .with(Diplomacy::Player)
        .with(Health::new(10))
        .with(Player::new())
        .build()
}

fn init_enemy(world: &mut World, char_sprites: &[SpriteRender]) -> Entity {
    let (world_x, world_y) = map_to_world_hex(9 as f32, 6 as f32);
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALAR, SCALAR, 0.0));
    transform.set_translation_xyz(world_x, -world_y, CHAR_Z);

    world.create_entity()
        .with(char_sprites[0].clone())
        .with(Character::new())
        .with(ID::new())
        .with(HexCoords{ x: 9, y: 6 })
        .with(Diplomacy::Enemy)
        .with(Health::new(10))
        .with(AI{
            action_choices: vec![
                AIActionOptionSeq {
                    sequence: vec![
                        AIActionOption {
                            option: ActionOption::Move(MovementOptions::basic(2)),
                            target: AITargetChoice::ClosestAlly(3, 30),
                        },
                    ]
                },
                AIActionOptionSeq {
                    sequence: vec![
                        AIActionOption {
                            option: ActionOption::Move(MovementOptions::basic(1)),
                            target: AITargetChoice::ClosestAlly(2, 3),
                        },
                        AIActionOption {
                            option: ActionOption::Attack(AttackOptions::basic(1, 6, DamageType::Blunt)),
                            target: AITargetChoice::ClosestAlly(1, 2),
                        }
                    ]
                },
                AIActionOptionSeq {
                    sequence: vec![
                        AIActionOption {
                            option: ActionOption::Attack(AttackOptions::basic(1, 6, DamageType::Blunt)),
                            target: AITargetChoice::ClosestAlly(1, 2),
                        }
                    ]
                }
            ],
            tree: Some(vec![AITree{
                require: AIRequire::Target(AITargetDecision::AnyAlly(3, 30)),
                decision: Some(0),
                tree: None,
            }, AITree {
                require: AIRequire::Target(AITargetDecision::AnyAlly(2, 3)),
                decision: Some(1),
                tree: None,
            }, AITree {
                require: AIRequire::Target(AITargetDecision::AnyAlly(1, 2)),
                decision: Some(2),
                tree: None,
            }])
        })
        .with(transform)
        .build()
}

fn init_cards(world: &mut World, card_sprites: &[SpriteRender]) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let text = UiText::new(
        font.clone(),
        "this is a test".to_string(),
        [0.02, 0.02, 0.02, 1.],
        20.,
    );
    let card_transform = UiTransform::new(
        "CARD_test".to_string(),
        Anchor::BottomLeft,
        Anchor::BottomLeft,
        100.0, 100.0, UI_Z, 250.0, 350.0,
    );
    world.create_entity()
        .with(UiImage::Sprite(card_sprites[0].clone()))
        .with(Name::new("card_test".to_string()))
        .with(text)
        .with(card_transform)
        .build();
}

fn init_debug(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let phase_transform = UiTransform::new(
        "DEBUG_phase".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        20.0, -20.0, UI_Z, 200.0, 20.0,
    );

    let phase = world
        .create_entity()
        .with(phase_transform)
        .with(UiText::new(
            font.clone(),
            "init".to_string(),
            [1., 1., 1., 1.],
            20.,
        ))
        .build();

    let hover_transform = UiTransform::new(
        "DEBUG_hover".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        20.0, -40.0, UI_Z, 200.0, 20.0,
    );

    let hover = world
        .create_entity()
        .with(hover_transform)
        .with(UiText::new(
            font.clone(),
            "init".to_string(),
            [1., 1., 1., 1.],
            20.,
        ))
        .build();

    let hover_hp_transform = UiTransform::new(
        "DEBUG_hover".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        20.0, -60.0, UI_Z, 200.0, 20.0,
    );

    let hover_hp = world
        .create_entity()
        .with(hover_hp_transform)
        .with(UiText::new(
            font.clone(),
            "init".to_string(),
            [1., 1., 1., 1.],
            20.,
        ))
        .build();

    let ui_hover_transform = UiTransform::new(
        "DEBUG_ui_hover".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        20.0, -80.0, UI_Z, 200.0, 20.0,
    );

    let ui_hover = world
        .create_entity()
        .with(ui_hover_transform)
        .with(UiText::new(
            font.clone(),
            "init".to_string(),
            [1., 1., 1., 1.],
            20.,
        ))
        .build();

    world.insert(DebugText{ phase, ui_hover, hover, hover_hp });
    world.insert(TypeFaces{ debug: font })
}