use bevy::prelude::*;

pub struct AddNoiseEvent;
pub struct RemoveNoiseEvent;

// use bevy::ecs::schedule::ShouldRun;

use bevy::sprite::MaterialMesh2dBundle;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

use crate::constants::CAP;

use super::init::{Affines, AliveCells, LifeState};
use super::step::{hatsmesh, spawn_idxs};
use super::LifeConfig;

pub fn add_noise(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    affines: Option<Res<Affines>>,
    life_config: Res<LifeConfig>,
    life_state: Option<ResMut<LifeState>>,
    cells: Query<Entity, With<AliveCells>>,
    mut evts: EventReader<AddNoiseEvent>,
) {
    if affines.is_some() && life_state.is_some() {
        let affines = affines.unwrap();
        let mut life_state = life_state.unwrap();
        for _ in evts.iter() {
            for c in cells.iter() {
                commands.entity(c).despawn();
            }
            let mut rng = rand::thread_rng();
            let die = Uniform::from(1..5);

            // life_state.new.clear();
            let mut ne = Vec::with_capacity(CAP);

            for idx in 0..affines.0.len() {
                let t: f32 = rng.gen();

                if t < life_config.add_noise_percent {
                    life_state.new[idx] = true;
                }
                if life_state.new[idx] {
                    ne.push(idx)
                }
            }

            // dbg!(&life_state);

            let hatss = hatsmesh(&ne, &affines.0);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(hatss).into(),
                    // transform: Transform::default().with_scale(Vec3::splat(128.)),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    ..default()
                },
                AliveCells,
            ));
            // spawn_idxs(&mut commands, &affines.0, &ne);
        }
    }
}

pub fn remove_noise(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    life_config: Res<LifeConfig>,
    cells: Query<Entity, With<AliveCells>>,
    affines: Option<Res<Affines>>,
    life_state: Option<ResMut<LifeState>>,
    mut evts: EventReader<RemoveNoiseEvent>,
) {
    if affines.is_some() && life_state.is_some() {
        let affines = affines.unwrap();
        let mut life_state = life_state.unwrap();
        for _ in evts.iter() {
            for c in cells.iter() {
                commands.entity(c).despawn();
            }

            let mut rng = rand::thread_rng();

            // life_state.new.clear();
            let mut ne = Vec::with_capacity(CAP);

            for idx in 0..affines.0.len() {
                let t: f32 = rng.gen();

                if t < life_config.remove_noise_percent {
                    life_state.new[idx] = false;
                }
                if life_state.new[idx] {
                    ne.push(idx)
                }
            }

            // dbg!(&life_state);

            let hatss = hatsmesh(&ne, &affines.0);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(hatss).into(),
                    // transform: Transform::default().with_scale(Vec3::splat(128.)),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    ..default()
                },
                AliveCells,
            ));
            // spawn_idxs(&mut commands, &affines.0, &ne);
        }
    }
}
