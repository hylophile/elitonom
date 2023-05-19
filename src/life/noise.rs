use bevy::prelude::*;

pub struct AddNoiseEvent;

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

use super::init::{Affines, LifeState};
use super::step::spawn_idxs;

pub fn add_noise(
    mut commands: Commands,
    affines: Res<Affines>,
    mut life_state: ResMut<LifeState>,
    mut evts: EventReader<AddNoiseEvent>,
) {
    for _ in evts.iter() {
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

        spawn_idxs(&mut commands, &affines.0, &ne);
    }
}
