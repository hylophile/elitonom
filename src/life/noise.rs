use bevy::prelude::*;

pub struct AddNoiseEvent;
pub struct RemoveNoiseEvent;

// use bevy::ecs::schedule::ShouldRun;

use bevy::sprite::MaterialMesh2dBundle;
use rand::distributions::Uniform;
use rand::Rng;

use crate::constants::CAP;

use super::init::{Affines, AliveCells, LifeState};
use super::step::hatsmesh;
use super::LifeConfig;

pub fn add_noise(
    mut commands: Commands,
    (mut meshes, mut materials, life_state): (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        Option<ResMut<LifeState>>,
    ),
    (life_config, affines): (Res<LifeConfig>, Option<Res<Affines>>),
    cells: Query<Entity, With<AliveCells>>,
    mut evts: EventReader<AddNoiseEvent>,
) {
    if let (Some(affines), Some(mut life_state)) = (affines, life_state) {
        for _ in evts.iter() {
            for c in cells.iter() {
                commands.entity(c).despawn();
            }
            let mut rng = rand::thread_rng();
            let _die = Uniform::from(1..5);

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
    (mut meshes, mut materials, life_state): (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        Option<ResMut<LifeState>>,
    ),
    (life_config, affines): (Res<LifeConfig>, Option<Res<Affines>>),
    cells: Query<Entity, With<AliveCells>>,
    mut evts: EventReader<RemoveNoiseEvent>,
) {
    if let (Some(affines), Some(mut life_state)) = (affines, life_state) {
        for _ in evts.iter() {
            for c in cells.iter() {
                if let Some(mut e) = commands.get_entity(c) {
                    e.despawn()
                }
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
