use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;

use bevy::{math::Affine2, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::tree::hat_meta_tiles::HAT_OUTLINE;
use crate::tree::spectre::SPECTRE_OUTLINE;
use crate::tree::TreeConfig;

use super::init::{Affines, AliveCells, HatNeighbors, LifeState};

use super::{LifeConfig, StepTimer};

#[rustfmt::skip]
static HAT_INDICES: [u32; 24] = [
    0,  1,  3,
    0,  3,  4,
    0,  5,  9,
    0,  9, 10,
    0, 11, 12,
    1,  2,  3,
    5,  6,  7,
    5,  7,  8,
];

#[rustfmt::skip]
static SPECTRE_INDICES: [u32; 36] = [
    0,   1,   13,
    1,   2,   3,
    1,   3,   4,
    1,   4,   8,
    1,   8,   9,
    1,   9,   13,
    4,   5,   7,
    4,   7,   8,
    5,   6,   7,
    13,  10,  11,
    12,  13,  11,
    13,  9,   10,
];

pub fn gen_mesh(idxs: &Vec<usize>, affines: &[Affine2], is_spectre: bool) -> Mesh {
    let (outline, shape_indices): (&[Vec2], &[u32]) = if is_spectre {
        (SPECTRE_OUTLINE, &SPECTRE_INDICES)
    } else {
        (HAT_OUTLINE, &HAT_INDICES)
    };

    let mut vertices = Vec::with_capacity(affines.len());
    vertices.extend(idxs.iter().flat_map(|id| {
        outline.iter().map(|p| {
            let p2 = affines[*id].transform_point2(*p);
            [p2.x, p2.y, 0.0]
        })
    }));

    let indices = {
        let mut indices: Vec<u32> = Vec::with_capacity(shape_indices.len() * idxs.len());
        indices.extend((0..idxs.len() as u32).flat_map(|j| {
            shape_indices
                .iter()
                .map(move |i| i + j * outline.len() as u32)
        }));
        Indices::U32(indices)
    };
    let positions: Vec<_> = vertices.clone();
    let normals: Vec<_> = vertices.iter().map(|_| [0.0, 0.0, 1.0]).collect();
    let uvs: Vec<_> = vertices.iter().map(|_| [1.0, 0.0]).collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

pub fn _spawn_idxs(
    commands: &mut Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    affines: &[Affine2],
    idxs: &Vec<usize>,
) {
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

    commands.spawn((
        ShapeBundle {
            path: g.build(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Fill::color(Color::rgba(0.0, 0.0, 0.0, 1.0)),
        AliveCells,
    ));
}

pub struct StepLifeEvent;

pub fn step_life(
    mut commands: Commands,
    mut events: EventReader<StepLifeEvent>,
    (mut meshes, mut materials, life_state): (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        Option<ResMut<LifeState>>,
    ),
    (life_config, tree_config, neighbors, affines): (
        Res<LifeConfig>,
        Res<TreeConfig>,
        Option<Res<HatNeighbors>>,
        Option<Res<Affines>>,
    ),
    cells: Query<Entity, With<AliveCells>>,
) {
    if let (Some(mut life_state), Some(neighbors), Some(affines)) = (life_state, neighbors, affines)
    {
        for _ in events.iter() {
            for c in cells.iter() {
                commands.entity(c).despawn();
            }
            life_state.swap();
            let mut ne = Vec::with_capacity(affines.0.len());
            let life_state = &mut *life_state;

            for (i, x) in life_state.old.iter().enumerate() {
                let ns = &neighbors.0[i];
                let count = ns.iter().filter(|idx| life_state.old[**idx]).count(); // as u32;
                life_state.new[i] = match x {
                    true => life_config.survival[count],
                    false => life_config.birth[count],
                };
                if life_state.new[i] {
                    ne.push(i);
                }
            }
            let mesh = gen_mesh(&ne, &affines.0, tree_config.spectre);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(mesh).into(),
                    material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.99))),
                    ..default()
                },
                AliveCells,
            ));
        }
    }
}
pub fn tick_life(
    time: Res<Time>,
    mut events: EventWriter<StepLifeEvent>,
    mut step_timer: ResMut<StepTimer>,
) {
    if step_timer.0.tick(time.delta()).finished() {
        events.send(StepLifeEvent)
    }
}
