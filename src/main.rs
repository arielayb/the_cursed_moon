use bevy::prelude::*;

#[derive(Component)]
struct Person;

fn hello_world(){
    println!("The cursed moon");
}

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
