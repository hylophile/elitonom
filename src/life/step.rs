use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;
// use bevy::ecs::schedule::ShouldRun;
use bevy::{math::Affine2, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::constants::CAP;
use crate::meta_tiles::HAT_OUTLINE;

use super::init::{Affines, AliveCells, HatNeighbors, LifeState};

use super::{LifeConfig, StepTimer};

pub fn hatsmesh(idxs: &Vec<usize>, affines: &[Affine2]) -> Mesh {
    // let extent_x = quad.size.x / 2.0;
    // let extent_y = quad.size.y / 2.0;

    // let (u_left, u_right) = if quad.flip { (1.0, 0.0) } else { (0.0, 1.0) };
    // let vertices = [
    //     ([-extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [u_left, 1.0]),
    //     ([-extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [u_left, 0.0]),
    //     ([extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [u_right, 0.0]),
    //     ([extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [u_right, 1.0]),
    // ];
    let vertices: Vec<_> = idxs
        .iter()
        .map(|id| {
            [
                [
                    affines[*id].translation.x + 10.0,
                    affines[*id].translation.y,
                    0.0,
                ],
                [
                    affines[*id].translation.x + 10.0,
                    affines[*id].translation.y + 10.0,
                    0.0,
                ],
                [affines[*id].translation.x, affines[*id].translation.y, 0.0],
            ]
        })
        .flatten()
        .collect();

    let is: Vec<u32> = (0..vertices.len() as u32).collect();
    let indices = Indices::U32(is);

    let positions: Vec<_> = vertices.iter().map(|a| *a).collect();
    let normals: Vec<_> = vertices.iter().map(|_| [0.0, 0.0, 1.0]).collect();
    // let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|_| [1.0, 0.0]).collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

pub fn spawn_idxs(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

    // std::process::exit(0);

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut life_state: ResMut<LifeState>,
    life_config: Res<LifeConfig>,
    cells: Query<Entity, With<AliveCells>>,
    neighbors: Res<HatNeighbors>,
    affines: Res<Affines>,
    time: Res<Time>,
    mut step_timer: ResMut<StepTimer>,
) {
    if step_timer.0.tick(time.delta()).finished() {
        for c in cells.iter() {
            commands.entity(c).despawn();
        }
        life_state.swap();
        let mut ne = Vec::with_capacity(CAP);
        let life_state = &mut *life_state;

        for (i, x) in life_state.old.iter().enumerate() {
            let ns = &neighbors.0[i];
            let count = ns.iter().filter(|idx| life_state.old[**idx]).count() as u32;
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
        let hatss = hatsmesh(&ne, &affines.0);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(hatss).into(),
                // transform: Transform::default().with_scale(Vec3::splat(128.)),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            },
            AliveCells,
        ));
        // spawn_idxs(&mut commands, &affines.0, &ne);
    }
}
