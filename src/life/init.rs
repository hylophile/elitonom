// use bevy::ecs::schedule::ShouldRun;
use bevy::{math::Affine2, prelude::*};

use kiddo::distance::squared_euclidean;
use kiddo::float::kdtree::KdTree;

// #[cfg(not(target_arch = "wasm32"))]
// pub type Kdt = kiddo::float::kdtree::KdTree<f32, usize, 2, 64, u32>;
// #[cfg(target_arch = "wasm32")]
pub type Kdt = kiddo::float::kdtree::KdTree<f32, u32, 2, 64, u16>;

use std::ops::Mul;

use crate::constants::CAP;
use crate::{
    meta_tiles::{MetaTile, TileType, HAT_OUTLINE},
    tree::MetaTileTree,
};

use super::noise::AddNoiseEvent;

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

fn touching(a: Affine2, b: Affine2) -> bool {
    let eps = 0.01;
    let pa = HAT_OUTLINE.iter().map(|p| a.transform_point2(*p));
    let pb = HAT_OUTLINE.iter().map(|p| b.transform_point2(*p));
    for p in pa {
        let t = pb
            .clone()
            .any(|q| (p.x - q.x).abs() < eps && (p.y - q.y).abs() < eps);
        // dbg!(p);
        // dbg!(pb.clone().map(|q| (q - p).abs()));
        if t {
            return true;
        }
    }
    false
}

fn neighbors(kdtree: &Kdt, affines: &[Affine2], idx: usize) -> Vec<usize> {
    let n = 20;
    let mut ns: Vec<_> = Vec::with_capacity(n);

    // let _t: Vec<_> = kdtree
    //     .nearest_n(&[0.0, 0.0], n, &squared_euclidean)
    //     .into_iter()
    //     .collect();
    let oa = affines[idx];
    // let _origin = HAT_OUTLINE.iter().map(|p| oa.transform_point2(*p));

    kdtree
        .nearest_n(oa.translation.as_ref(), n, &squared_euclidean)
        .into_iter()
        .filter(|n| touching(oa, affines[n.item as usize]))
        // .filter(|n| n.distance < 80.0)
        .filter(|n| n.item as usize != idx)
        .for_each(|neighbor| {
            ns.push(neighbor.item as usize);
            // dbg!(neighbor.distance);
        });
    ns
}

fn make_affines(affines: &mut Vec<Affine2>, t: Affine2, tree: &MetaTile) {
    let new_transform = t.mul(tree.transform);
    for child in &tree.children {
        make_affines(affines, new_transform, child)
    }

    match tree.shape {
        TileType::H1Hat | TileType::HHat | TileType::THat | TileType::PHat | TileType::FHat => {
            affines.push(new_transform)
        }
        _ => {
            // let level = (tree.width as f32).log2() as usize;
            // polys.meta[level].push(poly);
        }
    }
}

pub fn init_life(mut evt: EventWriter<AddNoiseEvent>) {
    // evt.send(AddNoiseEvent);
}

/// .
pub fn gen_neighbors(mut commands: Commands, mtt: Option<Res<MetaTileTree>>) {
    if let Some(mtt) = mtt {
        if mtt.is_added() || mtt.is_changed() {
            let mut affines = Vec::with_capacity(CAP);
            make_affines(
                &mut affines,
                // Affine2::from_scale(Vec2 { x: 5.0, y: 5.0 }),
                Affine2::IDENTITY,
                &mtt.0,
            );
            let mut kdtree: Kdt = KdTree::with_capacity(affines.len());

            affines
                .iter()
                .enumerate()
                .for_each(|(idx, a)| kdtree.add(a.translation.as_ref(), idx as u32));

            let life_state = LifeState {
                // old: Vec::with_capacity(CAP),
                // new: Vec::with_capacity(CAP),
                old: vec![false; affines.len()],
                new: vec![false; affines.len()],
            };

            let neighbors: Vec<_> = (0..(affines.len()))
                .map(|a| neighbors(&kdtree, &affines, a))
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
