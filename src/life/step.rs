// use bevy::ecs::schedule::ShouldRun;
use bevy::{math::Affine2, prelude::*};
use bevy_prototype_lyon::prelude::*;
use kiddo::{distance::squared_euclidean, KdTree};
use std::collections::HashSet;
use std::ops::Mul;

use rand::distributions::{Distribution, Uniform};

use crate::constants::CAP;
use crate::{
    constants::*,
    meta_tiles::{MetaTile, TileType, HAT_OUTLINE},
    tree::MetaTileTree,
};

use super::init::{Affines, Cells, HatNeighbors, LifeState};
use super::noise::AddNoiseEvent;
use super::LifeConfig;

pub fn spawn_idxs(mut commands: &mut Commands, affines: &Vec<Affine2>, idxs: &Vec<usize>) {
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

pub fn step_life(
    mut commands: Commands,
    mut life_state: ResMut<LifeState>,
    life_config: Res<LifeConfig>,
    cells: Query<Entity, With<Cells>>,
    neighbors: Res<HatNeighbors>,
    affines: Res<Affines>,
) {
    for c in cells.iter() {
        commands.entity(c).despawn();
    }
    life_state.swap();
    let mut ne = Vec::with_capacity(CAP);
    let life_state = &mut *life_state;

    for (i, x) in life_state.old.iter().enumerate() {
        let ns = &neighbors.0[i];
        let count = ns
            .iter()
            .filter(|idx| life_state.old[**idx] == true)
            .count() as u32;
        life_state.new[i] = match x {
            true => life_config.survival.contains(&count),
            false => life_config.birth.contains(&count),
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
    spawn_idxs(&mut commands, &affines.0, &ne);
}
