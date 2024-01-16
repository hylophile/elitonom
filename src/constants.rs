use bevy::prelude::Color;

pub static STROKE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);

pub static BG_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

pub const STROKE_WIDTH: f32 = 0.1;

pub const CAP: usize = 200_000;

#[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
pub const LEVELS: usize = 6;

#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub const LEVELS: usize = 3;

#[cfg(target_arch = "wasm32")]
pub const LEVELS: usize = 5;
