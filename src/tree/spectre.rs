use bevy::math::Affine2;
use bevy::math::Vec2;
use bevy::prelude::*;
use std::ops::Mul;
use std::sync::Arc;

use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle, Stroke},
    shapes,
};
type Quad = [Vec2; 4];

#[derive(Debug)]
pub struct SpectreMetaTile {
    pub transform: Affine2,
    pub quad: Quad,
    pub children: Vec<Arc<SpectreMetaTile>>,
}

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

pub const SPECTRE_KEYS: &Quad = &[
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

use crate::constants::STROKE_COLOR;
use crate::constants::STROKE_WIDTH;

use super::DeadCells;

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
    //
    let smt = SpectreMetaTile {
        transform: Affine2::IDENTITY,
        quad: *SPECTRE_KEYS,
        children: Vec::new(),
    };
    let mut polys = Vec::new();
    make_spectre_polygons(&mut polys, Affine2::IDENTITY, &smt);

    let mut g = GeometryBuilder::new();
    for poly in polys {
        dbg!(&poly);
        g = g.add(&poly);
    }
    commands.spawn((
        ShapeBundle {
            path: g.build(),
            ..default()
        },
        Stroke::new(STROKE_COLOR, STROKE_WIDTH),
        DeadCells,
    ));
}

fn make_spectre_polygons(polys: &mut Vec<shapes::Polygon>, t: Affine2, tree: &SpectreMetaTile) {
    for child in &tree.children {
        make_spectre_polygons(polys, t.mul(tree.transform), child);
    }

    let tt = t.mul(tree.transform);
    let points = SPECTRE_OUTLINE
        .iter()
        .map(|p| tt.transform_point2(*p))
        // .map(|p| (Affine2::from_scale(Vec2::new(100.0, 100.0))).transform_point2(*p))
        .collect();
    let poly = shapes::Polygon {
        points,
        closed: true,
    };

    polys.push(poly)
}

fn build_spectre_base() {
    // [Delta, Theta, Lambda, Xi, Pi, Sigma, Phi, Psi]
}

fn build_spectre_super_tiles() {
    //
}
