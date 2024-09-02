mod logic;
mod prelude;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, logic::plugin))
        .run();
}
