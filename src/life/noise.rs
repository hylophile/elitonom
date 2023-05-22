use bevy::prelude::*;

pub struct AddNoiseEvent;

// use bevy::ecs::schedule::ShouldRun;

use rand::distributions::{Distribution, Uniform};

use crate::constants::CAP;

use super::init::{Affines, LifeState};
use super::step::spawn_idxs;

pub fn add_noise(
    mut commands: Commands,
    affines: Option<Res<Affines>>,
    mut life_state: Option<ResMut<LifeState>>,
    mut evts: EventReader<AddNoiseEvent>,
) {
    if affines.is_some() && life_state.is_some() {
        let affines = affines.unwrap();
        let mut life_state = life_state.unwrap();
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
}
