# Asteroids Built in Rust Using Bevy
TODO: Need to add health the entities


- The goal of this project is to create the classic Asteroids game using Rust and to learn the Bevy Game Engine

## Bevy

- [bevy-engine](https://bevy.org/)
- `cargo add bevy # Adds bevy as a package`
- Uses SI units

### Examples Used

- [plugin](https://bevy.org/examples/application/plugin/)
- [physics-in-fixed-timestep](https://bevy.org/examples/movement/physics-in-fixed-timestep/)
- [2d-rendering/rotation](https://bevy.org/examples/2d-rendering/rotation/)
- [bevy_rand/Tutorial](https://docs.rs/bevy_rand/latest/bevy_rand/)
- [bevy/games/breakout](https://bevy.org/examples/games/breakout/)
- [bevy/multi_window_text](https://github.com/bevyengine/bevy/blob/main/examples/window/multi_window_text.rs)
- [bevy/multiple_windows](https://github.com/bevyengine/bevy/blob/main/examples/window/multiple_windows.rs)
- [bevy/text_input](https://github.com/bevyengine/bevy/blob/main/examples/input/text_input.rs)

### ECS Quick Start

- [Quick-Start/ECS](https://bevy.org/learn/quick-start/getting-started/ecs/)

## Bevy's Default Plugins

- `DefaultPlugins` are a `PluginGroup` containing core engine features like 2D / 3D renderer, asset loading, UI System, Windows, and input
  - Using this adds an "event loop" that runs once per frame

## Handling Movement In Bevy

### Transform

- contains translation, rotation, and scale
- `Translation::default()` uses the Identity matrix values

## Rotation

- use `rotation.normalize()` to normalize the angle
