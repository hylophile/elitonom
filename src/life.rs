use std::ops::Mul;

use bevy::app::StartupSet::PostStartup;
use bevy::{math::Affine2, prelude::*};
use bevy_prototype_lyon::prelude::*;
use kiddo::{distance::squared_euclidean, KdTree};
use std::collections::HashSet;

use rand::distributions::{Distribution, Uniform};

use crate::constants::CAP;
use crate::{
    constants::*,
    meta_tiles::{MetaTile, TileType, HAT_OUTLINE},
    tree::MetaTileTree,
};

#[derive(Resource)]
pub struct MetaTileKdTree(pub KdTree<f32, 2>);

#[derive(Resource)]
struct Affines(pub Vec<Affine2>);

#[derive(Debug, Resource)]
struct LifeState {
    // HashSet
    old: Vec<bool>,
    new: Vec<bool>,
}

impl LifeState {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.old, &mut self.new)
    }
}

#[derive(Component)]
struct Cells;

fn touching(a: Affine2, b: Affine2) -> bool {
    let eps = 0.0001;
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

fn neighbors(kdtree: &KdTree<f32, 2>, affines: &Vec<Affine2>, idx: usize) -> Vec<usize> {
    let n = 20;
    let mut ns: Vec<usize> = Vec::with_capacity(n);

    // let _t: Vec<_> = kdtree
    //     .nearest_n(&[0.0, 0.0], n, &squared_euclidean)
    //     .into_iter()
    //     .collect();
    let oa = affines[idx];
    // let _origin = HAT_OUTLINE.iter().map(|p| oa.transform_point2(*p));

    kdtree
        .nearest_n(oa.translation.as_ref(), n, &squared_euclidean)
        .into_iter()
        .filter(|n| touching(oa, affines[n.item]))
        // .filter(|n| n.distance < 80.0)
        .filter(|n| n.item != idx)
        .for_each(|neighbor| {
            ns.push(neighbor.item);
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

fn init_life(mut commands: Commands, affines: Res<Affines>, mut life_state: ResMut<LifeState>) {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..5);

    // life_state.new.clear();
    let mut ne = Vec::with_capacity(CAP);

    for idx in 0..affines.0.len() {
        let t = die.sample(&mut rng);

        if t == 1 {
            // life_state.new.push(idx);
            life_state.new[idx] = true;
            ne.push(idx);
        }
    }
    // dbg!(&life_state);

    spawn_idxs(commands, &affines.0, &ne);
}

fn spawn_idxs(mut commands: Commands, affines: &Vec<Affine2>, idxs: &Vec<usize>) {
    let mut g = GeometryBuilder::new();
    for aff in idxs {
        let points = HAT_OUTLINE
            .iter()
            .map(|p| affines[*aff].transform_point2(*p))
            .collect();
        let poly = shapes::Polygon {
            points,
            closed: true,
        };

        g = g.add(&poly);
    }

    // std::process::exit(0);

    commands.spawn((
        ShapeBundle {
            path: g.build(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Fill::color(Color::rgba(0.0, 0.0, 0.0, 1.0)),
        Cells,
    ));
}

fn step_life(
    mut commands: Commands,
    mut life_state: ResMut<LifeState>,
    affines: Res<Affines>,
    kdtree: Res<MetaTileKdTree>,
    cells: Query<Entity, With<Cells>>,
) {
    for c in cells.iter() {
        commands.entity(c).despawn();
    }
    life_state.swap();
    let mut hs: HashSet<usize> = HashSet::new();
    let mut ne = Vec::with_capacity(CAP);
    let life_state = &mut *life_state;

    for (i, x) in life_state.old.iter().enumerate() {
        let ns = neighbors(&kdtree.0, &affines.0, i);
        let count = ns
            .iter()
            .filter(|idx| life_state.old[**idx] == true)
            .count();
        life_state.new[i] = match x {
            true => match count {
                1 => true,
                2 => true,
                _ => false,
            },
            false => match count {
                2 => true,
                _ => false,
            },
        };
        if life_state.new[i] {
            ne.push(i);
        }

        // ns.iter().for_each(|n| {
        //     let _ = hs.insert(*n);
        // });
    }
    // dbg!(&ne);
    // life_state.new = hs.into_iter().collect();
    spawn_idxs(commands, &affines.0, &ne);
}

fn kdtree(mut commands: Commands, mtt: Res<MetaTileTree>) {
    let mut affines = Vec::with_capacity(CAP);
    make_affines(
        &mut affines,
        Affine2::from_scale(Vec2 { x: 5.0, y: 5.0 }),
        &mtt.0,
    );
    let mut kdtree: KdTree<f32, 2> = KdTree::with_capacity(affines.len());

    affines
        .iter()
        .enumerate()
        .for_each(|(idx, a)| kdtree.add(a.translation.as_ref(), idx));

    let mut life_state = LifeState {
        // old: Vec::with_capacity(CAP),
        // new: Vec::with_capacity(CAP),
        old: vec![false; affines.len()],
        new: vec![false; affines.len()],
    };
    commands.insert_resource(life_state);
    commands.insert_resource(MetaTileKdTree(kdtree));
    commands.insert_resource(Affines(affines));
}

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(kdtree)
            // .insert_resource(life_state)
            .add_startup_system(init_life.in_base_set(PostStartup))
            .add_system(step_life);
        // .add_system(step_life.in_schedule(CoreSchedule::FixedUpdate))
        // configure our fixed timestep schedule to run twice a second
        // .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP));
    }
}
