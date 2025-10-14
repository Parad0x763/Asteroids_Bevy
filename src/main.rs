use bevy::app::{App, Plugin, Startup, Update};
use bevy::ecs::{
    query::With,
    schedule::IntoScheduleConfigs as _,
    system::{Commands, Query, Res, ResMut},
    component::Component,
};
use bevy::time::{Time, Timer, TimerMode};
use bevy::DefaultPlugins;
use bevy::prelude::Resource;


pub struct HelloPlugin;


impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}


fn hello_world() {
    println!("hello world");
}


#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);


#[derive(Resource)]
struct GreetTimer(Timer);


fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Dennis Ritchey".to_string())));
    commands.spawn((Person, Name("Ken Thomson".to_string())));
    commands.spawn((Person, Name("Graydon Hoare".to_string())));
}


fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that casued the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}", name.0)
        }
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
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
