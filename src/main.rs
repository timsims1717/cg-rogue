extern crate pathfinding;
extern crate rand;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use crate::systems::{MouseInputSystem, CharMovementSystem, CameraMovementSystem, WindowResizeSystem, AITurnSystem, PlayerTurnSystem, CharacterActionSystem, DebugSystem};
use amethyst::input::{InputBundle, StringBindings};

mod components;
mod entities;
mod resources;
mod states;
mod systems;
mod util;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let input_config = config_dir.clone().join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&input_config)?,
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0., 0., 0., 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(MouseInputSystem::new(), "mouse_input", &[])
        .with(CharMovementSystem, "char_move", &[])
        .with(CameraMovementSystem, "camera_move", &[])
        .with(WindowResizeSystem::new(), "window_resize", &[])
        .with(PlayerTurnSystem, "player_turn_system", &[])
        .with(CharacterActionSystem, "character_action_system", &[])
        .with(AITurnSystem, "ai_turn_system", &[])
        .with(DebugSystem, "debug_system", &[]);

    let mut game = Application::new(assets_dir, states::GamePlayState, game_data)?;
    game.run();

    Ok(())
}
