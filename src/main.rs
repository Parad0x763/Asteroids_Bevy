use bevy::app::{App, Startup, Update};
use bevy::ecs::query::With;
use bevy::ecs::schedule::IntoScheduleConfigs as _;
use bevy::ecs::system::{Commands, Query};
use bevy::ecs::{component::Component};


fn hello_world() {
    println!("hello world");
}


#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);


fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Dennis Ritchey".to_string())));
    commands.spawn((Person, Name("Ken Thomson".to_string())));
    commands.spawn((Person, Name("Graydon Hoare".to_string())));
}


fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}", name.0);
    }
}


fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Graydon Hoare" {
            name.0 = "Graydon Hoare: Rust Creator".to_string();
            break; // just updating this one name
        }
    }
}


fn main() {
    App::new()
        .add_systems(Startup, add_people)
        // using .chain() forces the systems to run in exactly the order listed
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
