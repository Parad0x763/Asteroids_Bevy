use bevy::{color::palettes::tailwind, prelude::*};

mod components;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window{
                    title: "Asteroids".into(),
                    name: Some("bevy.app".into()),
                    resolution: (2560, 1440).into(), // older versions of Bevy used f32, now u32 is used
                    ..default()
                }),
                ..default()}),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2d);
}
