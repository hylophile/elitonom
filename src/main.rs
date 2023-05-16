const LEVELS: usize = 5;
const H_COLOR: Color = Color::WHITE;
const H_MIRROR_COLOR: Color = Color::SEA_GREEN;
const T_COLOR: Color = Color::TEAL;

const P_COLOR: Color = Color::CRIMSON;
const F_COLOR: Color = Color::GOLD;
// static STROKE_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
static STROKE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);
// static BG_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
static BG_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

const STROKE_WIDTH: f32 = 0.5;

use bevy::{
    math::Affine2,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    // sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;
use kiddo::{distance::squared_euclidean, float::neighbour::Neighbour, KdTree};
use rand::prelude::*;
use std::{f32::consts::PI, ops::Mul, rc::Rc};

mod utils;
use utils::*;

mod meta_tiles;
use meta_tiles::*;

mod tree;
use tree::{construct_meta_tiles, construct_patch, AllFour};

fn is_hat(s: TileType) -> bool {
    match s {
        TileType::H => false,
        TileType::T => false,
        TileType::P => false,
        TileType::F => false,
        TileType::H1Hat => true,
        TileType::HHat => true,
        TileType::THat => true,
        TileType::PHat => true,
        TileType::FHat => true,
    }
}

type HatPoints = Vec<shapes::Polygon>;
struct HatPolys {
    h: HatPoints,
    h1: HatPoints,
    t: HatPoints,
    f: HatPoints,
    p: HatPoints,
    meta: Vec<Vec<shapes::Polygon>>,
}
fn make_polygons(polys: &mut HatPolys, t: Affine2, tree: &MetaTile) {
    for child in &tree.children {
        make_polygons(polys, t.mul(tree.transform), &child)
    }

    let tt = t.mul(tree.transform);
    let points = tree
        .outline
        .iter()
        .map(|p| tt.transform_point2(*p))
        .collect();
    let poly = shapes::Polygon {
        points,
        closed: true,
    };
    match tree.shape {
        TileType::H1Hat => polys.h1.push(poly),
        TileType::HHat => polys.h.push(poly),
        TileType::THat => polys.t.push(poly),
        TileType::PHat => polys.p.push(poly),
        TileType::FHat => polys.f.push(poly),
        _ => {
            let level = (tree.width as f32).log2() as usize;
            polys.meta[level].push(poly);
        }
    }
}

fn make_affines(affines: &mut Vec<Affine2>, t: Affine2, tree: &MetaTile) {
    let new_transform = t.mul(tree.transform);
    for child in &tree.children {
        make_affines(affines, new_transform, &child)
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

fn setup(
    mut commands: Commands,
    // ass: Res<AssetServer>,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 1.0,
                ..default()
            },
            ..default()
        })
        .insert(bevy_pancam::PanCam::default());

    let mut a = AllFour {
        h: h_init(),
        t: t_init(),
        p: p_init(),
        f: f_init(),
    };

    for _ in 0..LEVELS {
        let patch = construct_patch(a.h, a.t, a.p, a.f);
        a = construct_meta_tiles(patch);
    }
    // let cap = 13_usize.pow(LEVELS.try_into().unwrap());
    let cap = 200_000;
    let which_meta_tile = a.h;

    let mut polys = HatPolys {
        h: Vec::with_capacity(cap),
        h1: Vec::with_capacity(cap),
        t: Vec::with_capacity(cap),
        f: Vec::with_capacity(cap),
        p: Vec::with_capacity(cap),
        meta: vec![Vec::new(); LEVELS + 2],
    };

    make_polygons(
        &mut polys,
        Affine2::from_scale(Vec2 { x: 5.0, y: 5.0 }),
        &which_meta_tile,
    );
    // std::process::exit(0);

    for (i, shape) in [
        TileType::H1Hat,
        TileType::HHat,
        TileType::THat,
        TileType::FHat,
        TileType::PHat,
    ]
    .iter()
    .enumerate()
    {
        let polys = match shape {
            TileType::H1Hat => &polys.h1,
            TileType::HHat => &polys.h,
            TileType::THat => &polys.t,
            TileType::PHat => &polys.p,
            TileType::FHat => &polys.f,
            _ => panic!(),
        };
        for chunk in polys.chunks(50_000) {
            let mut g = GeometryBuilder::new();
            for tile in chunk {
                g = g.add(tile);
            }

            // std::process::exit(0);

            commands.spawn((
                ShapeBundle {
                    path: g.build(),
                    transform: Transform::from_xyz(0.0, 0.0, i as f32 * 0.01),
                    ..default()
                },
                // Fill::color(shape_to_fill_color(*shape)),
                Stroke::new(STROKE_COLOR, STROKE_WIDTH),
            ));
        }
    }

    if false {
        for (i, outlines) in polys.meta.iter().enumerate() {
            let mut g = GeometryBuilder::new();
            for outline in outlines {
                g = g.add(outline);
            }

            // std::process::exit(0);

            commands.spawn((
                ShapeBundle {
                    path: g.build(),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                },
                Stroke::new(STROKE_COLOR, STROKE_WIDTH * 2.0_f32.powi(i as i32)),
            ));
        }
    }

    let mut affines = Vec::with_capacity(cap);
    make_affines(
        &mut affines,
        Affine2::from_scale(Vec2 { x: 5.0, y: 5.0 }),
        &which_meta_tile,
    );
    let mut kdtree: KdTree<f32, 2> = KdTree::with_capacity(affines.len());

    affines
        .iter()
        .enumerate()
        .for_each(|(idx, a)| kdtree.add(a.translation.as_ref(), idx));
    let n = 20;
    let mut ns: Vec<Affine2> = Vec::with_capacity(n);

    let t: Vec<_> = kdtree
        .nearest_n(&[0.0, 0.0], n, &squared_euclidean)
        .into_iter()
        .collect();
    // let a = affines[t[0].item];
    // let b = affines[t[1].item];

    fn touching(a: Affine2, b: Affine2) -> bool {
        let eps = 0.0001;
        let pa = HAT_OUTLINE.iter().map(|p| a.transform_point2(*p));
        let mut pb = HAT_OUTLINE.iter().map(|p| b.transform_point2(*p));
        for p in pa {
            let t = pb
                .clone()
                .any(|q| (p.x - q.x).abs() < eps && (p.y - q.y).abs() < eps);
            // dbg!(p);
            dbg!(pb.clone().map(|q| (q - p).abs()));
            if t {
                return true;
            }
        }
        return false;
    }

    // let c = touching(a, b);
    // dbg!(c);
    let oa = affines[100];
    let origin = HAT_OUTLINE.iter().map(|p| oa.transform_point2(*p));

    let x = kdtree
        .nearest_n(&oa.translation.as_ref(), n, &squared_euclidean)
        .into_iter()
        .filter(|n| touching(oa, affines[n.item]))
        // .filter(|n| n.distance < 80.0)
        .filter(|n| n.item != 100)
        .for_each(|neighbor| {
            ns.push(affines[neighbor.item].clone());
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
    for aff in ns.iter() {
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

    // std::process::exit(0);
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(ShapePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(BG_COLOR))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct HTile;
#[derive(Component)]
struct TTile;
#[derive(Component)]
struct PTile;
#[derive(Component)]
struct FTile;

fn shape_to_fill_color(shape: TileType) -> Color {
    let _tr = 0.4;
    match shape {
        TileType::H1Hat => H_MIRROR_COLOR,
        TileType::HHat => H_COLOR,
        TileType::THat => T_COLOR,
        TileType::PHat => P_COLOR,
        TileType::FHat => F_COLOR,
        // TileType::H => Color::rgba(0.0, 0.0, 1.0, tr),
        // TileType::T => Color::rgba(0.0, 1.0, 1.0, tr),
        // TileType::P => Color::rgba(1.0, 0.0, 1.0, tr),
        // TileType::F => Color::rgba(1.0, 1.0, 0.0, tr),
        // TileType::Pseudo => _
        _ => Color::rgba(1.0, 1.0, 1.0, 0.0),
        // _ => Color::rgba(0.0, 0.0, 0.0, 0.0),
    }
}
