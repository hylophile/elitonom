use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;

// use bevy::ecs::schedule::ShouldRun;
use bevy::{math::Affine2, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::meta_tiles::HAT_OUTLINE;

use super::init::{Affines, AliveCells, HatNeighbors, LifeState};

use super::{LifeConfig, StepTimer};

#[rustfmt::skip]
static IND: [u32; 24] = [
    0,  1,  3,
    0,  3,  4,
    0,  5,  9,
    0,  9, 10,
    0, 11, 12,
    1,  2,  3,
    5,  6,  7,
    5,  7,  8,
];

pub fn hatsmesh(idxs: &Vec<usize>, affines: &[Affine2]) -> Mesh {
    let mut vertices = Vec::with_capacity(affines.len());
    vertices.extend(idxs.iter().flat_map(|id| {
        HAT_OUTLINE.iter().map(|p| {
            let p2 = affines[*id].transform_point2(*p);
            [p2.x, p2.y, 0.0]
        })
    }));

    let mut is: Vec<u32> = Vec::with_capacity(IND.len() * idxs.len());
    is.extend((0..idxs.len() as u32).flat_map(|j| IND.iter().map(move |i| i + j * 13)));
    let indices = Indices::U32(is);

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

pub fn step_life(
    mut commands: Commands,
    (mut meshes, mut materials, mut life_state, mut step_timer): (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        ResMut<LifeState>,
        ResMut<StepTimer>,
    ),
    (life_config, neighbors, affines, time): (
        Res<LifeConfig>,
        Res<HatNeighbors>,
        Res<Affines>,
        Res<Time>,
    ),
    cells: Query<Entity, With<AliveCells>>,
) {
    if step_timer.0.tick(time.delta()).finished() {
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
        let hatss = hatsmesh(&ne, &affines.0);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(hatss).into(),
                material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.99))),
                // material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.1))),
                ..default()
            },
            AliveCells,
        ));
    }
}
