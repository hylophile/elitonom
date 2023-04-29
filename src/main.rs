//! Shows how to render simple primitive shapes with a single color.
#![feature(fn_traits)]

use bevy::{
    math::Affine2,
    prelude::*,
    // sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;
use std::ops::Mul;
use trees::Tree;
// const SCALE: f32 = 10.0;
const SQ3: f32 = 1.732_050_8;
const HR3: f32 = 0.866_025_4;

fn match_segment(p: Vec2, q: Vec2) -> Affine2 {
    Affine2::from_cols_array_2d(&[[q.x - p.x, q.y - p.y], [p.y - q.y, q.x - p.x], [p.x, p.y]])
}

fn match_two(p1: Vec2, q1: Vec2, p2: Vec2, q2: Vec2) -> Affine2 {
    match_segment(p2, q2).mul(match_segment(p1, q1).inverse())
}

const HAT_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(-1.5, -0.5 * SQ3),
    Vec2::new(-1.0, -SQ3),
    Vec2::new(1.0, -SQ3),
    Vec2::new(1.5, -0.5 * SQ3),
    Vec2::new(3.0, -SQ3),
    Vec2::new(4.5, -0.5 * SQ3),
    Vec2::new(4.0, 0.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(3.0, SQ3),
    Vec2::new(1.5, 1.5 * SQ3),
    Vec2::new(1.0, SQ3),
    Vec2::new(0.0, SQ3),
];

const H_COLOR: Color = Color::WHITE;
const H_MIRROR_COLOR: Color = Color::SEA_GREEN;
const T_COLOR: Color = Color::TEAL;

const P_COLOR: Color = Color::CRIMSON;
const F_COLOR: Color = Color::GOLD;
const STROKE_COLOR: Color = Color::BLACK;

const H_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(4.0, 0.0),
    Vec2::new(4.5, HR3),
    Vec2::new(2.5, 5.0 * HR3),
    Vec2::new(1.5, 5.0 * HR3),
    Vec2::new(-0.5, HR3),
];

fn h_init() -> Tree<MetaTile> {
    let mut h = Tree::new(MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::H,
        width: 2,
    });
    h.push_back(Tree::new(MetaTile {
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[5], H_OUTLINE[0]),
        shape: TileType::HHat,
        width: 1,
    }));
    h.push_back(Tree::new(MetaTile {
        transform: match_two(HAT_OUTLINE[9], HAT_OUTLINE[11], H_OUTLINE[1], H_OUTLINE[2]),
        shape: TileType::HHat,
        width: 1,
    }));
    h.push_back(Tree::new(MetaTile {
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[3], H_OUTLINE[4]),
        shape: TileType::HHat,
        width: 1,
    }));
    h.push_back(Tree::new(MetaTile {
        transform: Affine2::from_cols_array_2d(&[
            [-0.25, 0.5 * HR3],
            [0.5 * HR3, 0.25],
            [2.5, HR3],
        ]),
        shape: TileType::H1Hat,
        width: 1,
    }));

    h
}

fn draw_tree(commands: &mut Commands, tree: Tree<MetaTile>) {
    for child in tree.iter() {
        // match child.data().shape {
        //     TileType::H => todo!(),
        //     TileType::T => todo!(),
        //     TileType::P => todo!(),
        //     TileType::F => todo!(),
        //     TileType::H1Hat => todo!(),
        //     TileType::HHat => todo!(),
        //     TileType::THat => todo!(),
        //     TileType::PHat => todo!(),
        //     TileType::FHat => todo!(),
        // }
        // let mirror = child.data().shape == TileType::H1Hat;
        commands.spawn(hat2(child.data().transform, child.data().shape));
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
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

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum TileType {
    H,
    T,
    P,
    F,
    H1Hat,
    HHat,
    THat,
    PHat,
    FHat,
}

enum Rule {
    H,
    Four(usize, usize, TileType, usize),
    Six(usize, usize, usize, usize, TileType, usize),
}

static RULES: &[Rule] = &[
    Rule::H,
    Rule::Four(0, 0, TileType::P, 2),
    Rule::Four(1, 0, TileType::H, 2),
    Rule::Four(2, 0, TileType::P, 2),
    Rule::Four(3, 0, TileType::H, 2),
    Rule::Four(4, 4, TileType::P, 2),
    Rule::Four(0, 4, TileType::F, 3),
    Rule::Four(2, 4, TileType::F, 3),
    Rule::Six(4, 1, 3, 2, TileType::F, 0),
    Rule::Four(8, 3, TileType::H, 0),
    Rule::Four(9, 2, TileType::P, 0),
    Rule::Four(10, 2, TileType::H, 0),
    Rule::Four(11, 4, TileType::P, 2),
    Rule::Four(12, 0, TileType::H, 2),
    Rule::Four(13, 0, TileType::F, 3),
    Rule::Four(14, 2, TileType::F, 1),
    Rule::Four(15, 3, TileType::H, 4),
    Rule::Four(8, 2, TileType::F, 1),
    Rule::Four(17, 3, TileType::H, 0),
    Rule::Four(18, 2, TileType::P, 0),
    Rule::Four(19, 2, TileType::H, 2),
    Rule::Four(20, 4, TileType::F, 3),
    Rule::Four(20, 0, TileType::P, 2),
    Rule::Four(22, 0, TileType::H, 2),
    Rule::Four(23, 4, TileType::F, 3),
    Rule::Four(23, 0, TileType::F, 3),
    Rule::Four(16, 0, TileType::P, 2),
    Rule::Six(9, 4, 0, 2, TileType::T, 2),
    Rule::Four(4, 0, TileType::F, 3),
];

#[derive(Debug)]
struct MetaTile {
    transform: Affine2,
    shape: TileType,
    width: u8,
}

impl MetaTile {
    pub fn new(transform: Affine2, shape: TileType, width: u8) -> Self {
        Self {
            transform,
            shape,
            width,
        }
    }
}

fn shape_to_outline(shape: TileType) -> &'static [Vec2] {
    match shape {
        TileType::H => H_OUTLINE,
        TileType::T => todo!(),
        TileType::P => todo!(),
        TileType::F => todo!(),
        TileType::H1Hat => todo!(),
        TileType::HHat => todo!(),
        TileType::THat => todo!(),
        TileType::PHat => todo!(),
        TileType::FHat => todo!(),
    }
}

fn construct_patch(h: MetaTile, t: MetaTile, f: MetaTile, p: MetaTile) {
    let mut root = Tree::new(MetaTile::new(Affine2::IDENTITY, TileType::H, 2));

    for rule in RULES {
        match rule {
            Rule::H => {
                root.push_back(Tree::new(MetaTile::new(Affine2::IDENTITY, TileType::H, 2)));
            }
            Rule::Four(n_child, n_outline, shape, _) => {
                let child = root.iter().nth(*n_child).unwrap().data();
                let poly = shape_to_outline(child.shape);
                let t = child.transform;

                let point_p = t.transform_point2(poly[(n_outline + 1) % poly.len()]);
                let point_q = t.transform_point2(poly[*n_outline]);

                let new_shape = shape; //todo
                let new_poly = h; //todo
                let new_transform = todo!();
                let a = 1;

                // root.push_back(Tree::new(MetaTile));
            }
            Rule::Six(_, _, _, _, _, _) => todo!(),
        }
    }
}

fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    let hh = h_init();
    draw_tree(&mut commands, hh);
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.05,
                ..default()
            },
            ..default()
        })
        .insert(bevy_pancam::PanCam::default());
}

fn hat2(transform: Affine2, shape: TileType) -> (ShapeBundle, Fill, Stroke) {
    let hat_polygon = shapes::Polygon {
        points: Vec::from(HAT_OUTLINE),
        closed: true,
    };

    let color = match shape {
        TileType::H1Hat => H_MIRROR_COLOR,
        TileType::HHat => H_COLOR,
        TileType::THat => T_COLOR,
        TileType::PHat => P_COLOR,
        TileType::FHat => F_COLOR,
        _ => panic!(),
    };

    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&hat_polygon),
            transform: Transform::from_matrix(mat4_from_affine2(transform)),
            ..default()
        },
        Fill::color(color),
        Stroke::new(STROKE_COLOR, 0.2),
    )
}

fn mat4_from_affine2(affine2: Affine2) -> Mat4 {
    Mat4::from_cols(
        Vec4::new(affine2.matrix2.x_axis.x, affine2.matrix2.x_axis.y, 0.0, 0.0),
        Vec4::new(affine2.matrix2.y_axis.x, affine2.matrix2.y_axis.y, 0.0, 0.0),
        Vec4::Z,
        Vec4::new(affine2.translation.x, affine2.translation.y, 0.0, 1.0),
    )
}
