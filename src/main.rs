use bevy::prelude::*;
use bevy_ascii_terminal::{prelude::*, TiledCameraBundle};

#[derive(Component)]
pub struct GameTerminal;

pub const VIEWPORT_SIZE: [u32;2] = [80,40];
pub const UI_SIZE: [u32;2] = [VIEWPORT_SIZE[0],8];
pub const GAME_SIZE: [u32;2] = [VIEWPORT_SIZE[0], VIEWPORT_SIZE[1] - UI_SIZE[1]];

fn setup(mut commands: Commands) {
    // Create the terminal
    let term_y = VIEWPORT_SIZE[1] as u32 / 2 - GAME_SIZE[1] as u32 / 2; 
    let term = Terminal::new([20, term_y]).with_border(Border::single_line());
 
    let term_bundle = TerminalBundle::from(term).with_size([GAME_SIZE[0], GAME_SIZE[1] + 2]);

    commands.spawn(term_bundle).insert(GameTerminal);

    let totalx = GAME_SIZE[0];
    let totaly = GAME_SIZE[1] + UI_SIZE[1];

    commands.spawn(TiledCameraBundle::new().with_tile_count([totalx, totaly]));

}

fn main () {
    App::new()
    .add_plugins((DefaultPlugins, TerminalPlugin))
    .add_systems(Startup, setup)
    .insert_resource(ClearColor(Color::BLACK))
    .run();
}