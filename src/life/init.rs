use bevy::{math::Affine2, prelude::*};

use kiddo::distance::squared_euclidean;
use kiddo::float::kdtree::KdTree;

pub type Kdt = kiddo::float::kdtree::KdTree<f32, u32, 2, 64, u16>;

use std::ops::Mul;

use crate::constants::CAP;
use crate::tree::spectre::{SpectreNode, SPECTRE_OUTLINE};
use crate::tree::MetaTileNode;
use crate::{
    tree::hat_meta_tiles::{HatMetaTile, HatTileType, HAT_OUTLINE},
    tree::MetaTileTree,
};

#[derive(Resource)]
pub struct MetaTileKdTree(pub Kdt);

#[derive(Resource)]
pub struct Affines(pub Vec<Affine2>);

#[derive(Debug, Resource)]
pub struct LifeState {
    pub old: Vec<bool>,
    pub new: Vec<bool>,
}

impl LifeState {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.old, &mut self.new)
    }
}

#[derive(Component)]
pub struct AliveCells;

fn touching(a: Affine2, b: Affine2, outline: &[Vec2]) -> bool {
    let epsilon = 0.01;
    let points_a = outline.iter().map(|p| a.transform_point2(*p));
    let points_b = outline.iter().map(|p| b.transform_point2(*p));
    for point_a in points_a {
        let t = points_b.clone().any(|point_b| {
            (point_a.x - point_b.x).abs() < epsilon && (point_a.y - point_b.y).abs() < epsilon
        });
        if t {
            return true;
        }
    }
    false
}

fn neighbors(kdtree: &Kdt, affines: &[Affine2], idx: usize, outline: &[Vec2]) -> Vec<usize> {
    let n = 20;
    let shape_affine = affines[idx];

    let mut neighbor_idxs: Vec<_> = Vec::with_capacity(n);

    kdtree
        .nearest_n(shape_affine.translation.as_ref(), n, &squared_euclidean)
        .into_iter()
        .filter(|candidate| touching(shape_affine, affines[candidate.item as usize], outline))
        .filter(|candidate| candidate.item as usize != idx)
        .for_each(|neighbor| {
            neighbor_idxs.push(neighbor.item as usize);
        });
    neighbor_idxs
}

fn make_spectre_affines(affines: &mut Vec<Affine2>, t: Affine2, node: &SpectreNode) {
    // TODO we could just get the affines when we generate the polys in make_spectre_polys
    match node {
        SpectreNode::Meta(tree) => {
            let new_transform = t.mul(tree.transform);
            for child in &tree.children {
                make_spectre_affines(affines, new_transform, child);
            }
        }
        SpectreNode::Shape(leaf) => {
            let new_transform = t.mul(leaf.transform);
            affines.push(new_transform);
        }
    }
}

fn make_hat_affines(affines: &mut Vec<Affine2>, t: Affine2, tree: &HatMetaTile) {
    // TODO we could just get the affines when we generate the polys in make_hat_polys
    let new_transform = t.mul(tree.transform);
    for child in &tree.children {
        make_hat_affines(affines, new_transform, child)
    }

    match tree.shape {
        HatTileType::H1Hat
        | HatTileType::HHat
        | HatTileType::THat
        | HatTileType::PHat
        | HatTileType::FHat => affines.push(new_transform),
        _ => (),
    }
}

pub fn gen_neighbors(mut commands: Commands, mtt: Option<Res<MetaTileTree>>) {
    if let Some(mtt) = mtt {
        if mtt.is_added() || mtt.is_changed() {
            let mut affines = Vec::with_capacity(CAP);
            let mut outline = HAT_OUTLINE;
            match &mtt.0 {
                MetaTileNode::Hat(hmtt) => {
                    make_hat_affines(&mut affines, Affine2::IDENTITY, &hmtt);
                }
                MetaTileNode::Spectre(s) => match s {
                    SpectreNode::Meta(_) => {
                        make_spectre_affines(&mut affines, Affine2::IDENTITY, &s);
                        outline = SPECTRE_OUTLINE;
                    }
                    SpectreNode::Shape(_) => todo!(),
                },
            };
            let mut kdtree: Kdt = KdTree::with_capacity(affines.len());

            affines
                .iter()
                .enumerate()
                .for_each(|(idx, a)| kdtree.add(a.translation.as_ref(), idx as u32));

            let life_state = LifeState {
                old: vec![false; affines.len()],
                new: vec![false; affines.len()],
            };

            let neighbors: Vec<_> = (0..(affines.len()))
                .map(|a| neighbors(&kdtree, &affines, a, outline))
                .collect();

            commands.insert_resource(life_state);
            commands.insert_resource(MetaTileKdTree(kdtree));
            commands.insert_resource(Affines(affines));
            commands.insert_resource(HatNeighbors(neighbors));
        }
    }
}

#[derive(Resource, Debug)]
pub struct HatNeighbors(pub Vec<Vec<usize>>);
