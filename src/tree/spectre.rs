use super::DeadCells;
use super::MetaTileNode;
use super::MetaTileTree;
use crate::constants::STROKE_COLOR;
use crate::constants::STROKE_WIDTH;
use bevy::math::Affine2;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle, Stroke},
    shapes,
};
use std::f32::consts::PI;
use std::ops::Mul;
use std::ops::{Index, IndexMut};
use std::sync::Arc;
type Quad = [Vec2; 4];

#[derive(Debug, Clone)]
pub struct SpectreShape {
    pub transform: Affine2,
}

impl SpectreShape {
    pub fn new() -> Self {
        Self {
            transform: Affine2::IDENTITY,
        }
    }
    pub fn transformed(transform: Affine2) -> Self {
        Self { transform }
    }
}

#[derive(Debug, Clone)]
pub struct SpectreMetaTile {
    pub transform: Affine2,
    pub quad: Quad,
    pub children: Vec<Arc<SpectreNode>>,
}

#[derive(Debug, Clone)]
pub enum SpectreNode {
    Meta(SpectreMetaTile),
    Shape(SpectreShape),
}

impl SpectreMetaTile {
    pub fn new() -> Self {
        Self {
            transform: Affine2::IDENTITY,
            quad: *SPECTRE_KEYS,
            children: Vec::new(),
        }
    }
    pub fn transformed(transform: Affine2) -> Self {
        Self {
            transform,
            quad: *SPECTRE_KEYS,
            children: Vec::new(),
        }
    }
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
struct Sys {
    gamma: SpectreNode,
    delta: SpectreNode,
    theta: SpectreNode,
    lambda: SpectreNode,
    xi: SpectreNode,
    pi: SpectreNode,
    sigma: SpectreNode,
    phi: SpectreNode,
    psi: SpectreNode,
}

impl Sys {
    pub fn new() -> Self {
        Self {
            gamma: SpectreNode::Shape(SpectreShape::new()),
            delta: SpectreNode::Shape(SpectreShape::new()),
            theta: SpectreNode::Shape(SpectreShape::new()),
            lambda: SpectreNode::Shape(SpectreShape::new()),
            xi: SpectreNode::Shape(SpectreShape::new()),
            pi: SpectreNode::Shape(SpectreShape::new()),
            sigma: SpectreNode::Shape(SpectreShape::new()),
            phi: SpectreNode::Shape(SpectreShape::new()),
            psi: SpectreNode::Shape(SpectreShape::new()),
        }
    }
}

impl Index<SpectreCategory> for Sys {
    type Output = SpectreNode;

    fn index(&self, index: SpectreCategory) -> &Self::Output {
        match index {
            Gamma => &self.gamma,
            Delta => &self.delta,
            Theta => &self.theta,
            Lambda => &self.lambda,
            Xi => &self.xi,
            Pi => &self.pi,
            Sigma => &self.sigma,
            Phi => &self.phi,
            Psi => &self.psi,
        }
    }
}

impl IndexMut<SpectreCategory> for Sys {
    fn index_mut(&mut self, index: SpectreCategory) -> &mut Self::Output {
        match index {
            Gamma => &mut self.gamma,
            Delta => &mut self.delta,
            Theta => &mut self.theta,
            Lambda => &mut self.lambda,
            Xi => &mut self.xi,
            Pi => &mut self.pi,
            Sigma => &mut self.sigma,
            Phi => &mut self.phi,
            Psi => &mut self.psi,
        }
    }
}

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

const T_RULES: &[&(f32, usize, usize); 7] = &[
    &(60.0, 3, 1),
    &(0.0, 2, 0),
    &(60.0, 3, 1),
    &(60.0, 3, 1),
    &(0.0, 2, 0),
    &(60.0, 3, 1),
    &(-120.0, 3, 3),
];

pub fn spectre_background_polygons(mut commands: Commands, levels: usize) {
    //
    //
    // let smt = SpectreMetaTile {
    //     transform: Affine2::IDENTITY,
    //     quad: *SPECTRE_KEYS,
    //     children: Vec::new(),
    // };
    // let sys = build_spectre_base();
    // let smt = sys.gamma;
    let sys = build_spectre_tree(levels);
    let smt = sys.delta;

    let mut polys = Vec::new();
    make_spectre_polygons(&mut polys, Affine2::IDENTITY, &smt);

    let mut g = GeometryBuilder::new();
    for poly in polys {
        g = g.add(&poly);
    }

    commands.insert_resource(MetaTileTree(MetaTileNode::Spectre(smt)));
    commands.spawn((
        ShapeBundle {
            path: g.build(),
            ..default()
        },
        Stroke::new(STROKE_COLOR, STROKE_WIDTH),
        DeadCells,
    ));
}

fn make_spectre_polygons(polys: &mut Vec<shapes::Polygon>, t: Affine2, tree: &SpectreNode) {
    match tree {
        SpectreNode::Meta(tree) => {
            for child in &tree.children {
                make_spectre_polygons(polys, t.mul(tree.transform), child);
            }
        }
        SpectreNode::Shape(s) => {
            let tt = t.mul(s.transform);
            let points = SPECTRE_OUTLINE
                .iter()
                .map(|p| tt.transform_point2(*p))
                .collect();
            let poly = shapes::Polygon {
                points,
                closed: true,
            };
            polys.push(poly)
        }
    }
}

fn build_spectre_tree(levels: usize) -> Sys {
    let mut a = build_spectre_base();

    for _ in 0..levels {
        a = build_spectre_super_tiles(a);
    }
    a
}

fn build_spectre_base() -> Sys {
    let mystic = SpectreNode::Meta(SpectreMetaTile {
        transform: Affine2::IDENTITY,
        quad: *SPECTRE_KEYS,
        children: vec![
            Arc::new(SpectreNode::Shape(SpectreShape::new())),
            Arc::new(SpectreNode::Shape(SpectreShape::transformed(
                Affine2::from_angle_translation(PI / 6.0, SPECTRE_OUTLINE[8]),
            ))),
        ],
    });

    Sys {
        gamma: mystic,
        delta: SpectreNode::Shape(SpectreShape::new()),
        theta: SpectreNode::Shape(SpectreShape::new()),
        lambda: SpectreNode::Shape(SpectreShape::new()),
        xi: SpectreNode::Shape(SpectreShape::new()),
        pi: SpectreNode::Shape(SpectreShape::new()),
        sigma: SpectreNode::Shape(SpectreShape::new()),
        phi: SpectreNode::Shape(SpectreShape::new()),
        psi: SpectreNode::Shape(SpectreShape::new()),
    }
}

fn build_spectre_super_tiles(sys: Sys) -> Sys {
    let r = Affine2::from_scale(Vec2::new(-1.0, 1.0));
    let quad = match sys.delta {
        SpectreNode::Meta(ref m) => m.quad,
        SpectreNode::Shape(_) => *SPECTRE_KEYS,
    };
    let mut ts = vec![Affine2::IDENTITY];
    let mut total_angle = 0.0;
    let mut tquad = quad;
    let mut rot = Affine2::IDENTITY;

    for (angle, from, to) in T_RULES {
        total_angle += angle;
        if *angle != 0.0 {
            rot = Affine2::from_angle(total_angle.to_radians());
            tquad = quad
                .iter()
                .map(|p| rot.transform_point2(*p))
                .collect::<Vec<Vec2>>()
                .as_slice()
                .try_into()
                .unwrap();
        }
        let ttt1 = tquad[*to];
        let ttt2 = ts[ts.len() - 1].transform_point2(quad[*from]);

        let ttt = Affine2::from_translation(ttt2 - ttt1);

        ts.push(ttt * rot);
    }

    for t in ts.iter_mut() {
        *t = r.mul(*t);
    }

    let super_quad: Quad = [
        ts[6].transform_point2(quad[2]),
        ts[5].transform_point2(quad[1]),
        ts[3].transform_point2(quad[2]),
        ts[0].transform_point2(quad[1]),
    ];

    let mut ret = Sys::new();

    for (category, subs) in SUPER_RULES {
        // let
        // let children = subs.iter().enumerate(|i, sub| )
        let mut children = vec![];
        for i in 0..8 {
            if let Some(sub) = subs[i] {
                let mut a = sys[sub].clone();
                match a {
                    SpectreNode::Meta(ref mut m) => m.transform = ts[i],
                    SpectreNode::Shape(ref mut s) => s.transform = ts[i],
                };
                children.push(a.into());
            }
        }
        ret[*category] = SpectreNode::Meta(SpectreMetaTile {
            transform: Affine2::IDENTITY,
            quad: super_quad,
            children,
        })
    }

    ret

    // ts.push(tquad);
}
