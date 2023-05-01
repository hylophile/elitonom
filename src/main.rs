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
use std::{f32::consts::PI, fmt::Display, ops::Mul};
use trees::Tree;
// const SCALE: f32 = 10.0;
const SQ3: f32 = 1.732_050_8;
const HR3: f32 = 0.866_025_4;

fn match_segment(p: Vec2, q: Vec2) -> Affine2 {
    Affine2::from_cols_array_2d(&[[q.x - p.x, q.y - p.y], [p.y - q.y, q.x - p.x], [p.x, p.y]])
}

#[test]
fn ms2() {
    let p = Vec2 { x: 1.73, y: -0.21 };
    let q = Vec2 { x: 1.73, y: 0.21 };
    let res = match_segment(p, q);
    println!("{}", res);
    assert_eq!(1, 0);
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
        outline: H_OUTLINE.to_vec(),
    });
    h.push_back(Tree::new(MetaTile {
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[5], H_OUTLINE[0]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));
    h.push_back(Tree::new(MetaTile {
        transform: match_two(HAT_OUTLINE[9], HAT_OUTLINE[11], H_OUTLINE[1], H_OUTLINE[2]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));
    h.push_back(Tree::new(MetaTile {
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[3], H_OUTLINE[4]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));
    h.push_back(Tree::new(MetaTile {
        transform: Affine2::from_cols_array_2d(&[
            [-0.25, 0.5 * HR3],
            [0.5 * HR3, 0.25],
            [2.5, HR3],
        ]),
        shape: TileType::H1Hat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    h
}

const T_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(1.5, 3.0 * HR3),
];

fn t_init() -> Tree<MetaTile> {
    let mut t = Tree::new(MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::T,
        width: 2,
        outline: T_OUTLINE.to_vec(),
    });
    t.push_back(Tree::new(MetaTile {
        transform: Affine2::from_cols_array_2d(&[[0.5, 0.0], [0.0, 0.5], [0.5, HR3]]),
        shape: TileType::THat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    t
}

const P_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(4.0, 0.0),
    Vec2::new(3.0, 2.0 * HR3),
    Vec2::new(-1.0, 2.0 * HR3),
];

fn p_init() -> Tree<MetaTile> {
    let mut p = Tree::new(MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::P,
        width: 2,
        outline: P_OUTLINE.to_vec(),
    });
    p.push_back(Tree::new(MetaTile {
        transform: Affine2::from_cols_array_2d(&[[0.5, 0.0], [0.0, 0.5], [1.5, HR3]]),
        shape: TileType::PHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));
    p.push_back(Tree::new(MetaTile {
        transform: Affine2::from_cols_array_2d(&[
            [0.25, -0.5 * HR3],
            [0.5 * HR3, 0.25],
            [0.0, SQ3],
        ]),
        shape: TileType::PHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    p
}

const F_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(3.5, HR3),
    Vec2::new(3.0, 2.0 * HR3),
    Vec2::new(-1.0, 2.0 * HR3),
];

fn f_init() -> Tree<MetaTile> {
    let mut f = Tree::new(MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::F,
        width: 2,
        outline: F_OUTLINE.to_vec(),
    });
    f.push_back(Tree::new(MetaTile {
        transform: Affine2::from_cols_array_2d(&[[0.5, 0.0], [0.0, 0.5], [1.5, HR3]]),
        shape: TileType::FHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));
    f.push_back(Tree::new(MetaTile {
        transform: Affine2::from_cols_array_2d(&[
            [0.25, -0.5 * HR3],
            [0.5 * HR3, 0.25],
            [0.0, SQ3],
        ]),
        shape: TileType::FHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    f
}

fn build_out(tree: Tree<MetaTile>) {}

fn build_out_h(tree: Tree<MetaTile>) {}

fn draw_tree(commands: &mut Commands, tree: Tree<MetaTile>) {
    let mut z = 0.0;
    for (i, child) in tree.iter().enumerate() {
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

        let is_hat: bool = match child.data().shape {
            TileType::H => false,
            TileType::T => false,
            TileType::P => false,
            TileType::F => false,
            TileType::H1Hat => true,
            TileType::HHat => true,
            TileType::THat => true,
            TileType::PHat => true,
            TileType::FHat => true,
            TileType::Pseudo => true,
        };

        // if is_hat {
        //     continue;
        // }
        // println!("{}", &child.data());
        // let x = trees::Forest::new();
        // x.append(child);
        // let a: trees::Forest<MetaTile> = child
        //     .clone()
        //     .iter_mut()
        //     .map(|mut sub| sub.detach())
        //     .collect();

        // let c: Tree<MetaTile> = *child.detach();
        // draw_tree(commands, c);
        // continue;
        let t = child.data().transform;
        for hat in child.iter() {
            let mut ht = hat.data().clone();
            ht.transform = t.mul(ht.transform);
            z += 0.0001;
            commands.spawn(hat2(ht.clone(), z));
            // *hat.data_mut().transform = *t.mul(ht);
        }
        z += 0.0001;
        commands.spawn(hat2(child.data().clone(), z));
        // if i == 0 {
        //     break;
        // }
    }
    commands.spawn(hat2(tree.data().clone(), z));
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
    Pseudo,
}

#[derive(Debug)]
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

#[derive(Debug, Clone)]
struct MetaTile {
    transform: Affine2,
    shape: TileType,
    outline: Vec<Vec2>,
    width: u8,
}
use std::fmt;
impl fmt::Display for MetaTile {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--\n{:?}\n", self.shape)?;
        write!(f, "{:?}\n", self.transform)?;
        write!(f, "{:?}\n", self.outline)?;
        write!(f, "{:?}\n---", self.width)
    }
}
impl MetaTile {
    pub fn new(transform: Affine2, shape: TileType, width: u8, outline: Vec<Vec2>) -> Self {
        Self {
            transform,
            shape,
            width,
            outline,
        }
    }
}

fn shape_to_id(shape: TileType) -> usize {
    match shape {
        TileType::H => 0,
        TileType::T => 1,
        TileType::P => 2,
        TileType::F => 3,
        _ => panic!(),
    }
}

fn construct_patch(
    h: Tree<MetaTile>,
    t: Tree<MetaTile>,
    p: Tree<MetaTile>,
    f: Tree<MetaTile>,
) -> Tree<MetaTile> {
    let mut root = Tree::new(MetaTile::new(
        Affine2::IDENTITY,
        TileType::Pseudo,
        h.data().width,
        vec![],
    ));
    let shapes = [h, t, p, f];

    for rule in RULES {
        dbg!(rule);
        match rule {
            Rule::H => {
                let mut h = shapes[0].data().clone();
                h.transform = Affine2::IDENTITY;
                let mut h = Tree::new(h);
                let ch = shapes[0].clone().abandon();
                h.append(ch);
                dbg!(h.data());
                root.push_back(h);
            }
            Rule::Four(n_child, n_outline, shape, n_vertex) => {
                let child = root.iter().nth(*n_child).unwrap().data();
                let poly = child.outline.clone();
                let t = child.transform;

                let p2 = t.transform_point2(poly[(n_outline + 1) % poly.len()]);
                let q2 = t.transform_point2(poly[*n_outline]);

                let mut new_shape = shapes[shape_to_id(*shape)].clone(); //todo

                let new_shape_outline = new_shape.data().outline.clone();

                let p1 = new_shape_outline[*n_vertex];
                let q1 = new_shape_outline[(*n_vertex + 1) % new_shape_outline.len()];
                // let new_poly = h; //todo
                let new_transform = match_two(p1, q1, p2, q2);

                // *(new_shape).data_mut().transform = new_transform.into();
                let e = new_shape.data().width;
                let c = new_shape.abandon();
                let mut d = Tree::new(MetaTile {
                    transform: new_transform,
                    shape: *shape,
                    width: e,
                    outline: new_shape_outline,
                });
                d.append(c);

                dbg!(d.data());
                root.push_back(d);
            }
            Rule::Six(n_child_p, n_outline_p, n_child_q, n_outline_q, shape, n_vertex) => {
                let child_p = root.iter().nth(*n_child_p).unwrap().data();
                let child_q = root.iter().nth(*n_child_q).unwrap().data();

                let p2 = child_q
                    .transform
                    .transform_point2(child_q.outline[*n_outline_q]);
                let q2 = child_p
                    .transform
                    .transform_point2(child_p.outline[*n_outline_p]);

                let mut new_shape = shapes[shape_to_id(*shape)].clone();
                let new_shape_outline = new_shape.data().outline.clone();
                let p1 = new_shape_outline[*n_vertex];
                let q1 = new_shape_outline[(*n_vertex + 1) % new_shape_outline.len()];
                let new_transform = match_two(p1, q1, p2, q2);
                // *(new_shape).data_mut().transform = new_transform.into();
                let e = new_shape.data().width;
                let c = new_shape.abandon();

                let mut d = Tree::new(MetaTile {
                    transform: new_transform,
                    shape: *shape,
                    width: e,
                    outline: new_shape_outline,
                });
                d.append(c);

                dbg!(d.data());
                root.push_back(d);
            }
        }
    }
    root
}

struct AllFour {
    h: Tree<MetaTile>,
    t: Tree<MetaTile>,
    p: Tree<MetaTile>,
    f: Tree<MetaTile>,
}

fn rot_about(p: Vec2, angle: f32) -> Affine2 {
    Affine2::from_translation(p).mul(Affine2::from_angle(angle).mul(Affine2::from_translation(-p)))
}

fn intersect(p1: Vec2, q1: Vec2, p2: Vec2, q2: Vec2) -> Vec2 {
    let d = (q2.y - p2.y) * (q1.x - p1.x) - (q2.x - p2.x) * (q1.y - p1.y);
    let u_a = ((q2.x - p2.x) * (p1.y - p2.y) - (q2.y - p2.y) * (p1.x - p2.x)) / d;
    // const uB =
    //   ((q1.x - p1.x) * (p1.y - p2.y) - (q1.y - p1.y) * (p1.x - p2.x)) / d;

    return Vec2 {
        x: p1.x + u_a * (q1.x - p1.x),
        y: p1.y + u_a * (q1.y - p1.y),
    };
}

// #[test]
// fn test_rot() {
//     let p = Vec2::new(1.0, 0.0);
//     let res = rot_about(p, PI);
//     println!("{res}");
//     assert_f32_near!(res, [[-1.0, 0.0], [0.0, -1.0], [2.0, 0.0]]);
// }

fn eval_meta_tile(mt: &MetaTile, i: usize) -> Vec2 {
    let p = mt.outline[i];
    mt.transform.transform_point2(p)
}

fn construct_meta_tiles(patch: Tree<MetaTile>) -> AllFour {
    //-> AllFour {
    let children: Vec<&trees::Node<MetaTile>> = patch.iter().collect();
    let bps1 = eval_meta_tile(children[8].data(), 2);
    let bps2 = eval_meta_tile(children[21].data(), 2);
    let rbps = rot_about(bps1, (-2.0 * PI) / 3.0).transform_point2(bps2);

    let p72 = eval_meta_tile(children[7].data(), 2);
    let p252 = eval_meta_tile(children[25].data(), 2);

    let llc = intersect(bps1, rbps, eval_meta_tile(children[6].data(), 2), p72);
    let w = eval_meta_tile(children[6].data(), 2) - llc;

    let mut new_h_outline = vec![llc, bps1];

    let w = Affine2::from_angle(-PI / 3.0).transform_point2(w);

    // dbg!(bps1);
    // dbg!(llc);

    new_h_outline.push(new_h_outline[1] + w);
    new_h_outline.push(eval_meta_tile(children[14].data(), 2));

    let w = Affine2::from_angle(-PI / 3.0).transform_point2(w);

    new_h_outline.push(new_h_outline[3] - w);
    new_h_outline.push(eval_meta_tile(children[6].data(), 2));

    let mut new_h = Tree::new(MetaTile {
        transform: Affine2::IDENTITY,
        width: patch.data().width * 2,
        shape: TileType::H,
        outline: new_h_outline.clone(),
    });

    // let t = patch.data().transform;
    // let c = patch.abandon();
    let a: Vec<Tree<MetaTile>> = patch
        .clone()
        .iter_mut()
        .map(|mut sub| sub.detach())
        .collect();

    for n_child in [0, 9, 16, 27, 26, 6, 1, 8, 10, 15] {
        new_h.push_back(a[n_child].clone())
    }

    let new_p_outline = vec![p72, (p72 + (bps1 - llc)), bps1, llc];
    // dbg!(&new_p_outline);
    let mut new_p = Tree::new(MetaTile {
        transform: Affine2::IDENTITY,
        width: patch.data().width * 2,
        shape: TileType::P,
        outline: new_p_outline,
    });
    for n_child in [7, 2, 3, 4, 28] {
        new_p.push_back(a[n_child].clone())
    }

    let new_f_outline = vec![
        bps2,
        eval_meta_tile(children[24].data(), 2),
        eval_meta_tile(children[25].data(), 0),
        p252,
        (p252 + (llc - bps1)),
    ];
    let mut new_f = Tree::new(MetaTile {
        transform: Affine2::IDENTITY,
        width: patch.data().width * 2,
        shape: TileType::F,
        outline: new_f_outline,
    });
    for n_child in [21, 20, 22, 23, 24, 25] {
        new_f.push_back(a[n_child].clone())
    }

    let aaa = new_h_outline[2];
    let bbb = (new_h_outline[1] + (new_h_outline[4] - new_h_outline[5]));
    let ccc = rot_about(bbb, -PI / 3.0).transform_point2(aaa);
    let new_t_outline = vec![bbb, ccc, aaa];
    let mut new_t = Tree::new(MetaTile {
        transform: Affine2::IDENTITY,
        width: patch.data().width * 2,
        shape: TileType::T,
        outline: new_t_outline,
    });
    new_t.push_back(a[11].clone());

    AllFour {
        h: new_h.clone(),
        t: new_t.clone(),
        p: new_p.clone(),
        f: new_f.clone(),
    }
}

fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.05,
                ..default()
            },
            ..default()
        })
        .insert(bevy_pancam::PanCam::default());

    let h = h_init();
    let t = t_init();
    let p = p_init();
    let f = f_init();
    // let patch = AllFour { h, t, p, f };
    let patch = construct_patch(h.clone(), t.clone(), p.clone(), f.clone());
    // println!("{}", patch.to_string());
    let a = construct_meta_tiles(patch);
    let patch = construct_patch(a.h, a.t, a.p, a.f);
    let a = construct_meta_tiles(patch);
    // dbg!(&a.t.data());

    // dbg!(patch);

    draw_tree(&mut commands, a.h);
    // draw_tree(&mut commands, f);
    // draw_tree(&mut commands, pp);
    // draw_tree(&mut commands, ff);
}

fn hat2(tile: MetaTile, z: f32) -> (ShapeBundle, Fill, Stroke) {
    let hat_polygon = shapes::Polygon {
        // points: Vec::from(HAT_OUTLINE),
        points: Vec::from(tile.outline),
        closed: true,
    };

    let color = match tile.shape {
        TileType::H1Hat => H_MIRROR_COLOR,
        TileType::HHat => H_COLOR,
        TileType::THat => T_COLOR,
        TileType::PHat => P_COLOR,
        TileType::FHat => F_COLOR,
        _ => Color::rgba(0.0, 0.0, 0.0, 0.0),
    };

    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&hat_polygon),
            transform: Transform::from_matrix(mat4_from_affine2(tile.transform, z)),
            ..default()
        },
        Fill::color(color),
        Stroke::new(
            Color::rgba(0.0, 0.0, 0.0, 1.0),
            0.10 * (tile.width as f32).sqrt(),
        ),
    )
}

fn mat4_from_affine2(affine2: Affine2, z: f32) -> Mat4 {
    Mat4::from_cols(
        Vec4::new(affine2.matrix2.x_axis.x, affine2.matrix2.x_axis.y, 0.0, 0.0),
        Vec4::new(affine2.matrix2.y_axis.x, affine2.matrix2.y_axis.y, 0.0, 0.0),
        Vec4::Z,
        Vec4::new(affine2.translation.x, affine2.translation.y, z, 1.0),
    )
}
