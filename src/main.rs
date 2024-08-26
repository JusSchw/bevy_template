mod components;
mod events;
mod plugins;
mod resources;
mod states;
mod systems;
mod utils;

use bevy::prelude::*;
use plugins::MainPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();
}
