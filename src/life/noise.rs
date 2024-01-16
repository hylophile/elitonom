use bevy::prelude::*;

pub struct AddNoiseEvent {
    pub fraction: f32,
}

pub struct RemoveNoiseEvent {
    pub fraction: f32,
}

use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

use crate::constants::CAP;
use crate::tree::TreeConfig;

use super::init::{Affines, AliveCells, LifeState};
use super::step::gen_mesh;

pub fn add_noise(
    mut commands: Commands,
    (mut meshes, mut materials, life_state): (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        Option<ResMut<LifeState>>,
    ),
    affines: Option<Res<Affines>>,
    tree_config: Res<TreeConfig>,
    alive_cells: Query<Entity, With<AliveCells>>,
    mut evts: EventReader<AddNoiseEvent>,
) {
    if let (Some(affines), Some(mut life_state)) = (affines, life_state) {
        for evt in evts.iter() {
            for c in alive_cells.iter() {
                commands.entity(c).despawn();
            }
            let mut rng = rand::thread_rng();

            let mut new_cell_idxs = Vec::with_capacity(CAP);

            for idx in 0..affines.0.len() {
                let random_float: f32 = rng.gen();

                if random_float < evt.fraction {
                    life_state.new[idx] = true;
                }
                if life_state.new[idx] {
                    new_cell_idxs.push(idx)
                }
            }

            let mesh = gen_mesh(&new_cell_idxs, &affines.0, tree_config.spectre);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(mesh).into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    ..default()
                },
                AliveCells,
            ));
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
    affines: Option<Res<Affines>>,
    tree_config: Res<TreeConfig>,
    alive_cells: Query<Entity, With<AliveCells>>,
    mut evts: EventReader<RemoveNoiseEvent>,
) {
    if let (Some(affines), Some(mut life_state)) = (affines, life_state) {
        for evt in evts.iter() {
            for c in alive_cells.iter() {
                if let Some(mut e) = commands.get_entity(c) {
                    e.despawn()
                }
            }

            let mut rng = rand::thread_rng();
            let mut new_cell_idxs = Vec::with_capacity(CAP);

            for idx in 0..affines.0.len() {
                let random_float: f32 = rng.gen();

                if random_float < evt.fraction {
                    life_state.new[idx] = false;
                }
                if life_state.new[idx] {
                    new_cell_idxs.push(idx)
                }
            }

            let mesh = gen_mesh(&new_cell_idxs, &affines.0, tree_config.spectre);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(mesh).into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    ..default()
                },
                AliveCells,
            ));
        }
    }
}
