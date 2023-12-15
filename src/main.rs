extern crate fps_clock;

pub use crate::renderer::render; // Import the `renderer` module
mod renderer;

pub use crate::player::Player; // Import the `player` module
mod player;

pub use crate::camera::get_offset; // Import the `camera` module
mod camera;

pub use crate::menu::menu; // Import the `menu` module
mod menu;

pub use crate::car::Car; // Import the `car` module
mod car;

pub use crate::game::run;
mod game;

fn main() {
    menu();
}
