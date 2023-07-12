use bevy::math::Affine2;
use bevy::math::Vec2;
use bevy::prelude::*;
use std::sync::Arc;

pub const SPECTRE_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.5, -0.8660254037844386),
    Vec2::new(2.366025403784439, -0.36602540378443865),
    Vec2::new(2.366025403784439, 0.6339745962155614),
    Vec2::new(3.366025403784439, 0.6339745962155614),
    Vec2::new(3.866025403784439, 1.5),
    Vec2::new(3.0, 2.0),
    Vec2::new(2.133974596215561, 1.5),
    Vec2::new(1.6339745962155614, 2.3660254037844393),
    Vec2::new(0.6339745962155614, 2.3660254037844393),
    Vec2::new(-0.3660254037844386, 2.3660254037844393),
    Vec2::new(-0.866025403784439, 1.5),
    Vec2::new(0.0, 1.0),
];

pub const SPECTRE_KEYS: &[Vec2] = &[
    SPECTRE_OUTLINE[3],
    SPECTRE_OUTLINE[5],
    SPECTRE_OUTLINE[7],
    SPECTRE_OUTLINE[11],
];

enum SpectreCategory {
    Gamma,
    Delta,
    Theta,
    Lambda,
    Xi,
    Pi,
    Sigma,
    Phi,
    Psi,
}

use SpectreCategory::*;

#[rustfmt::skip]
const SUPER_RULES: &[(SpectreCategory, [Option<SpectreCategory>; 8])] = &[
    (Gamma,   [Some(Pi),   Some(Delta),  None,       Some(Theta),  Some(Sigma),  Some(Xi),   Some(Phi),     Some(Gamma)]),
    (Delta,   [Some(Xi),   Some(Delta),  Some(Xi),   Some(Phi),    Some(Sigma),  Some(Pi),   Some(Phi),     Some(Gamma)]),
    (Theta,   [Some(Psi),  Some(Delta),  Some(Pi),   Some(Phi),    Some(Sigma),  Some(Pi),   Some(Phi),     Some(Gamma)]),
    (Lambda,  [Some(Psi),  Some(Delta),  Some(Xi),   Some(Phi),    Some(Sigma),  Some(Pi),   Some(Phi),     Some(Gamma)]),
    (Xi,      [Some(Psi),  Some(Delta),  Some(Pi),   Some(Phi),    Some(Sigma),  Some(Psi),  Some(Phi),     Some(Gamma)]),
    (Pi,      [Some(Psi),  Some(Delta),  Some(Xi),   Some(Phi),    Some(Sigma),  Some(Psi),  Some(Phi),     Some(Gamma)]),
    (Sigma,   [Some(Xi),   Some(Delta),  Some(Xi),   Some(Phi),    Some(Sigma),  Some(Pi),   Some(Lambda),  Some(Gamma)]),
    (Phi,     [Some(Psi),  Some(Delta),  Some(Psi),  Some(Phi),    Some(Sigma),  Some(Pi),   Some(Phi),     Some(Gamma)]),
    (Psi,     [Some(Psi),  Some(Delta),  Some(Psi),  Some(Phi),    Some(Sigma),  Some(Psi),  Some(Phi),     Some(Gamma)]),
];

const T_RULES: &[&[isize; 3]; 7] = &[
    &[60, 3, 1],
    &[0, 2, 0],
    &[60, 3, 1],
    &[60, 3, 1],
    &[0, 2, 0],
    &[60, 3, 1],
    &[-120, 3, 3],
];

pub fn spectre_background_polygons(mut commands: Commands, levels: usize) {
    //
}

fn build_spectre_base() {
    //
}

fn build_spectre_super_tiles() {
    //
}
