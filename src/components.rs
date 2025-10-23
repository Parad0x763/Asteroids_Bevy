use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Player;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Asteroid;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Turret;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Collider;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}
