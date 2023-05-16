use std::ops::Mul;

use bevy_prototype_lyon::prelude::*;
use kiddo::{distance::squared_euclidean, KdTree};

use bevy::{math::Affine2, prelude::*};

use crate::{
    meta_tiles::{MetaTile, TileType, HAT_OUTLINE},
    tree::MetaTileTree,
};

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

fn make_affines(affines: &mut Vec<Affine2>, t: Affine2, tree: &MetaTile) {
    let new_transform = t.mul(tree.transform);
    for child in &tree.children {
        make_affines(affines, new_transform, child)
    }

    match tree.shape {
        TileType::H1Hat => affines.push(new_transform),
        TileType::HHat => affines.push(new_transform),
        TileType::THat => affines.push(new_transform),
        TileType::PHat => affines.push(new_transform),
        TileType::FHat => affines.push(new_transform),
        _ => {
            // let level = (tree.width as f32).log2() as usize;
            // polys.meta[level].push(poly);
        }
    }
}

fn n(mut commands: Commands, mtt: Res<MetaTileTree>) {
    let cap = 200_000;
    let mut affines = Vec::with_capacity(cap);
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
    let n = 20;
    let mut ns: Vec<Affine2> = Vec::with_capacity(n);

    let _t: Vec<_> = kdtree
        .nearest_n(&[0.0, 0.0], n, &squared_euclidean)
        .into_iter()
        .collect();
    // let a = affines[t[0].item];
    // let b = affines[t[1].item];

    // let c = touching(a, b);
    // dbg!(c);
    let oa = affines[100];
    let _origin = HAT_OUTLINE.iter().map(|p| oa.transform_point2(*p));

    kdtree
        .nearest_n(oa.translation.as_ref(), n, &squared_euclidean)
        .into_iter()
        .filter(|n| touching(oa, affines[n.item]))
        // .filter(|n| n.distance < 80.0)
        .filter(|n| n.item != 100)
        .for_each(|neighbor| {
            ns.push(affines[neighbor.item]);
            dbg!(neighbor.distance);
            // let af = affines[neighbor.item];
            // commands.spawn(MaterialMesh2dBundle {
            //     mesh: _meshes.add(Mesh::from(shape::Quad::default())).into(),
            //     transform: Transform::default()
            //         .with_translation(Vec3::new(af.translation.x, af.translation.y, 1.0))
            //         .with_scale(Vec3::splat(3.)),
            //     material: _materials.add(ColorMaterial::from(Color::PURPLE)),
            //     ..default()
            // });
            // dbg!("{}", affines[neighbor.item]);
        });

    let mut g = GeometryBuilder::new();
    for aff in &ns {
        let points = HAT_OUTLINE
            .iter()
            .map(|p| aff.transform_point2(*p))
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
        Fill::color(Color::rgba(0.0, 0.0, 0.0, 0.5)),
    ));
}
