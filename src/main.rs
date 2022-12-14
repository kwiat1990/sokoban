use ggez::{conf, event, Context, GameResult};
use specs::{Component, VecStorage, World, WorldExt};

use std::path;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {}

struct Game {}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
}

fn main() {
    let context_builder = ggez::ContextBuilder::new("sokoban", "kwiat1990")
        .window_setup(conf::WindowSetup::default().title("Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 800.0))
        .add_resource_path(path::PathBuf::from("./ressources"));

    let (context, event_loop) = context_builder
        .build()
        .expect("context_builder responded with an error");

    let game = Game {};
    event::run(context, event_loop, game)
}
