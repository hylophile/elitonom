const LEVELS: usize = 4;

use bevy::{
    math::Affine2,
    prelude::*,
    // sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;
use petgraph::{visit::Bfs, Graph};
use std::{f32::consts::PI, ops::Mul};
// use trees::Tree;
type Tree = Graph<MetaTile, (), petgraph::Directed>;
// const SCALE: f32 = 10.0;
//
const SQ3: f32 = 1.732_050_8;
const HR3: f32 = 0.866_025_4;
use rand::prelude::*;

use petgraph::graph::NodeIndex;
fn match_segment(p: Vec2, q: Vec2) -> Affine2 {
    Affine2::from_cols_array_2d(&[[q.x - p.x, q.y - p.y], [p.y - q.y, q.x - p.x], [p.x, p.y]])
    // let mut tree :  = Graph::new();
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

// const H_COLOR: Color = Color::BLACK;
// const H_MIRROR_COLOR: Color = Color::BLACK;
// const T_COLOR: Color = Color::BLACK;

// const P_COLOR: Color = Color::BLACK;
// const F_COLOR: Color = Color::WHITE;
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

fn h_init() -> Tree {
    let mut h = Tree::new();
    let h_root = h.add_node(MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::H,
        width: 2,
        outline: H_OUTLINE.to_vec(),
    });

    let h_0 = h.add_node(MetaTile {
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[5], H_OUTLINE[0]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    h.add_edge(h_root, h_0, ());

    let h_1 = h.add_node(MetaTile {
        transform: match_two(HAT_OUTLINE[9], HAT_OUTLINE[11], H_OUTLINE[1], H_OUTLINE[2]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    h.add_edge(h_root, h_1, ());

    let h_2 = h.add_node(MetaTile {
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[3], H_OUTLINE[4]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    h.add_edge(h_root, h_2, ());
    let h_3 = h.add_node(MetaTile {
        transform: Affine2::from_cols_array_2d(&[
            [-0.25, 0.5 * HR3],
            [0.5 * HR3, 0.25],
            [2.5, HR3],
        ]),
        shape: TileType::H1Hat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    h.add_edge(h_root, h_3, ());

    h
}

const T_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(1.5, 3.0 * HR3),
];

fn t_init() -> Tree {
    let mut t = Tree::new();
    let t_root = t.add_node(MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::T,
        width: 2,
        outline: T_OUTLINE.to_vec(),
    });

    let t_0 = t.add_node(MetaTile {
        transform: Affine2::from_cols_array_2d(&[[0.5, 0.0], [0.0, 0.5], [0.5, HR3]]),
        shape: TileType::THat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    t.add_edge(t_root, t_0, ());

    t
}

const P_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(4.0, 0.0),
    Vec2::new(3.0, 2.0 * HR3),
    Vec2::new(-1.0, 2.0 * HR3),
];

fn p_init() -> Tree {
    let mut p = Tree::new();
    let p_root = p.add_node(MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::P,
        width: 2,
        outline: P_OUTLINE.to_vec(),
    });
    let p_0 = p.add_node(MetaTile {
        transform: Affine2::from_cols_array_2d(&[[0.5, 0.0], [0.0, 0.5], [1.5, HR3]]),
        shape: TileType::PHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    let p_1 = p.add_node(MetaTile {
        transform: Affine2::from_cols_array_2d(&[
            [0.25, -0.5 * HR3],
            [0.5 * HR3, 0.25],
            [0.0, SQ3],
        ]),
        shape: TileType::PHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    p.add_edge(p_root, p_0, ());
    p.add_edge(p_root, p_1, ());

    p
}

const F_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(3.5, HR3),
    Vec2::new(3.0, 2.0 * HR3),
    Vec2::new(-1.0, 2.0 * HR3),
];

fn f_init() -> Tree {
    let mut f = Tree::new();
    let f_root = f.add_node(MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::F,
        width: 2,
        outline: F_OUTLINE.to_vec(),
    });
    let f_0 = f.add_node(MetaTile {
        transform: Affine2::from_cols_array_2d(&[[0.5, 0.0], [0.0, 0.5], [1.5, HR3]]),
        shape: TileType::FHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    let f_1 = f.add_node(MetaTile {
        transform: Affine2::from_cols_array_2d(&[
            [0.25, -0.5 * HR3],
            [0.5 * HR3, 0.25],
            [0.0, SQ3],
        ]),
        shape: TileType::FHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });

    f.add_edge(f_root, f_0, ());
    f.add_edge(f_root, f_1, ());

    f
}

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
        TileType::Pseudo => true,
    }
}

type HatPoints = Vec<shapes::Polygon>;
struct HatPolys {
    h: HatPoints,
    h1: HatPoints,
    t: HatPoints,
    f: HatPoints,
    p: HatPoints,
}

fn make_polygons(polys: &mut HatPolys, t: Affine2, tree: &Tree, index: NodeIndex) {
    let nd = tree[index];
    // let mut bfs = Bfs::new(&tree, 0.into());
    // while let Some(nx) = bfs.next(&tree) {
    //     todo!()
    // }
    for child in tree.neighbors(index) {
        make_polygons(polys, t.mul(nd.transform), tree, child)
    }

    // if is_hat(nd.shape) {
    let tt = t.mul(nd.transform);
    let points = nd.outline.iter().map(|p| tt.transform_point2(*p)).collect();
    let poly = shapes::Polygon {
        points,
        closed: true,
    };
    match nd.shape {
        TileType::H1Hat => polys.h1.push(poly),
        TileType::HHat => polys.h.push(poly),
        TileType::THat => polys.t.push(poly),
        TileType::PHat => polys.p.push(poly),
        TileType::FHat => polys.f.push(poly),
        _ => (),
        // TileType::H => todo!(),
        // TileType::T => todo!(),
        // TileType::P => todo!(),
        // TileType::F => todo!(),
        // TileType::Pseudo => todo!(),
    }
}

fn setup(
    mut commands: Commands,
    // ass: Res<AssetServer>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
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

    let h = h_init();
    let t = t_init();
    let p = p_init();
    let f = f_init();
    let mut a = AllFour {
        h: h.clone(),
        t: t.clone(),
        p: p.clone(),
        f: f.clone(),
    };

    for _ in 0..LEVELS {
        let patch = construct_patch(a.h, a.t, a.p, a.f);
        a = construct_meta_tiles(patch);
    }
    let cap = a.h.node_count().clone();
    let which_meta_tile = a.h;

    // _ = draw_tree(&mut commands, Affine2::IDENTITY, which_meta_tile, 0.0);

    let mut polys = HatPolys {
        h: Vec::with_capacity(cap),
        h1: Vec::with_capacity(cap),
        t: Vec::with_capacity(cap),
        f: Vec::with_capacity(cap),
        p: Vec::with_capacity(cap),
    };
    make_polygons(
        &mut polys,
        Affine2::from_scale(Vec2 { x: 5.0, y: 5.0 }),
        &which_meta_tile,
        0.into(),
    );
    std::process::exit(0);

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
        let mut g = GeometryBuilder::new();
        for tile in polys {
            g = g.add(tile);
        }

        // std::process::exit(0);

        commands.spawn((
            ShapeBundle {
                path: g.build(),
                transform: Transform::from_xyz(0.0, 0.0, i as f32 * 0.01),
                ..default()
            },
            Fill::color(shape_to_fill_color(*shape)),
            Stroke::new(
                Color::rgba(0.0, 0.0, 0.0, 1.0),
                // 0.10 * (tile.width as f32), //.sqrt(),
                // 0.15,
                0.5,
            ),
        ));
    }
}

// fn draw_tree(
//     commands: &mut Commands,
//     // ass: &Res<AssetServer>,
//     t: Affine2,
//     node: Tree,
//     mut z: f32,
// ) -> f32 {
//     for child in node.iter() {
//         let tc = node.data().transform;
//         z = draw_tree(commands, t.mul(tc), child.deep_clone(), z);
//         z += 0.00001;

//         // *hat.data_mut().transform = *t.mul(ht);
//         // if i == 0 {
//         //     break;
//         // }
//     }

//     let mut nn = node.data().clone();
//     nn.transform = t.mul(nn.transform);
//     //|| rand::random::<f32>() < 0.1 {
//     // if !is_hat(node.data().shape) {
//     commands.spawn(polygon_entity(nn, z));
//     // }
//     return z;
// }

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        // .add_system(rotate_colors_playground.in_schedule(CoreSchedule::FixedUpdate))
        // .add_system(rotate_colors_playground)
        .insert_resource(FixedTime::new_from_secs(1.0 / 30.0))
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
    width: usize,
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
    pub fn new(transform: Affine2, shape: TileType, width: usize, outline: Vec<Vec2>) -> Self {
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

fn construct_patch(h: Tree, t: Tree, p: Tree, f: Tree) -> Vec<Tree> {
    // let mut root = Tree::new(MetaTile::new(
    //     Affine2::IDENTITY,
    //     TileType::Pseudo,
    //     h.data().width,
    //     vec![],
    // ));
    let mut root = Vec::with_capacity(30);
    let shapes = [h, t, p, f];

    for rule in RULES {
        // dbg!(rule);
        match rule {
            Rule::H => {
                let mut h = shapes[0].clone();
                h[NodeIndex::from(0)].transform = Affine2::IDENTITY;
                // dbg!(h.data());
                root.push(h);
            }
            Rule::Four(n_child, n_outline, shape, n_vertex) => {
                let child: MetaTile = root[*n_child][NodeIndex::from(0)];
                let poly = child.outline.clone();
                let t = child.transform;

                let p2 = t.transform_point2(poly[(n_outline + 1) % poly.len()]);
                let q2 = t.transform_point2(poly[*n_outline]);

                let mut new_shape = shapes[shape_to_id(*shape)].clone(); //todo
                let new_shape_outline = new_shape[NodeIndex::from(0)].outline.clone();

                let p1 = new_shape_outline[*n_vertex];
                let q1 = new_shape_outline[(*n_vertex + 1) % new_shape_outline.len()];
                // let new_poly = h; //todo
                let new_transform = match_two(p1, q1, p2, q2);

                // *(new_shape).data_mut().transform = new_transform.into();
                // let e = new_shape.data().width;
                // let c = new_shape.abandon();
                // let mut d = Tree::new(MetaTile {
                //     transform: new_transform,
                //     shape: *shape,
                //     width: e,
                //     outline: new_shape_outline,
                // });
                // d.append(c);

                new_shape[NodeIndex::from(0)].transform = new_transform;
                new_shape[NodeIndex::from(0)].outline = new_shape_outline;

                // dbg!(d.data());
                root.push(new_shape);
            }
            Rule::Six(n_child_p, n_outline_p, n_child_q, n_outline_q, shape, n_vertex) => {
                // let child_p = root.iter().nth(*n_child_p).unwrap().data();
                // let child_q = root.iter().nth(*n_child_q).unwrap().data();
                let child_p = root[*n_child_p][NodeIndex::from(0)];
                let child_q = root[*n_child_q][NodeIndex::from(0)];

                let p2 = child_q
                    .transform
                    .transform_point2(child_q.outline[*n_outline_q]);
                let q2 = child_p
                    .transform
                    .transform_point2(child_p.outline[*n_outline_p]);

                let mut new_shape = shapes[shape_to_id(*shape)].clone();
                let new_shape_outline = new_shape[NodeIndex::from(0)].outline.clone();
                let p1 = new_shape_outline[*n_vertex];
                let q1 = new_shape_outline[(*n_vertex + 1) % new_shape_outline.len()];
                let new_transform = match_two(p1, q1, p2, q2);
                // *(new_shape).data_mut().transform = new_transform.into();
                // let e = new_shape.data().width;
                // let c = new_shape.abandon();

                // let mut d = Tree::new(MetaTile {
                //     transform: new_transform,
                //     shape: *shape,
                //     width: e,
                //     outline: new_shape_outline,
                // });
                // d.append(c);

                new_shape[NodeIndex::from(0)].transform = new_transform;
                new_shape[NodeIndex::from(0)].outline = new_shape_outline;

                // dbg!(d.data());
                root.push(new_shape);
            }
        }
    }
    root
}

struct AllFour {
    h: Tree,
    t: Tree,
    p: Tree,
    f: Tree,
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

fn eval_meta_tile(mt: MetaTile, i: usize) -> Vec2 {
    let p = mt.outline[i];
    mt.transform.transform_point2(p)
}

fn construct_meta_tiles(patch: Vec<Tree>) -> AllFour {
    //-> AllFour {
    // let patch: Vec<&trees::Node<MetaTile>> = patch.iter().collect();
    let root = NodeIndex::from(0);
    let bps1 = eval_meta_tile(patch[8][root], 2);
    let bps2 = eval_meta_tile(patch[21][root], 2);
    let rbps = rot_about(bps1, (-2.0 * PI) / 3.0).transform_point2(bps2);

    let p72 = eval_meta_tile(patch[7][root], 2);
    let p252 = eval_meta_tile(patch[25][root], 2);

    let llc = intersect(bps1, rbps, eval_meta_tile(patch[6][root], 2), p72);
    let w = eval_meta_tile(patch[6][root], 2) - llc;

    let mut new_h_outline = vec![llc, bps1];

    let w = Affine2::from_angle(-PI / 3.0).transform_point2(w);

    // dbg!(bps1);
    // dbg!(llc);

    new_h_outline.push(new_h_outline[1] + w);
    new_h_outline.push(eval_meta_tile(patch[14][root], 2));

    let w = Affine2::from_angle(-PI / 3.0).transform_point2(w);

    new_h_outline.push(new_h_outline[3] - w);
    new_h_outline.push(eval_meta_tile(patch[6][root], 2));

    let mut new_h = Tree::new();
    let h_root = new_h.add_node(MetaTile {
        transform: Affine2::IDENTITY,
        width: patch[0][root].width * 2,
        shape: TileType::H,
        outline: new_h_outline,
    });

    for n_child in [0, 9, 16, 27, 26, 6, 1, 8, 10, 15] {
        let new_node = new_h.add_node(patch[n_child]);
        new_h.add_edge(h_root, new_node, ());
    }

    // let new_p_outline = vec![p72, (p72 + (bps1 - llc)), bps1, llc];
    // let mut new_p = Tree::new();
    // let p_root = new_p.add_node(MetaTile {
    //     transform: Affine2::IDENTITY,
    //     width: patch[0][0].width * 2,
    //     shape: TileType::P,
    //     outline: new_p_outline,
    // });

    // for n_child in [7, 2, 3, 4, 28] {
    //     let new_node = new_p.add_node(patch[n_child]);
    //     new_p.add_edge(p_root, new_node, ());
    // }

    // let new_f_outline = vec![
    //     bps2,
    //     eval_meta_tile(patch[24][0], 2),
    //     eval_meta_tile(patch[25][0], 0),
    //     p252,
    //     (p252 + (llc - bps1)),
    // ];

    // let mut new_f = Tree::new();
    // let f_root = new_f.add_node(MetaTile {
    //     transform: Affine2::IDENTITY,
    //     width: patch[0][0].width * 2,
    //     shape: TileType::F,
    //     outline: new_f_outline,
    // });

    // for n_child in [21, 20, 22, 23, 24, 25] {
    //     let new_node = new_p.add_node(patch[n_child]);
    //     new_p.add_edge(p_root, new_node, ());
    // }

    // let aaa = new_h_outline[2];
    // let bbb = (new_h_outline[1] + (new_h_outline[4] - new_h_outline[5]));
    // let ccc = rot_about(bbb, -PI / 3.0).transform_point2(aaa);
    // let new_t_outline = vec![bbb, ccc, aaa];

    // let mut new_t = Tree::new();
    // let t_root = new_t.add_node(MetaTile {
    //     transform: Affine2::IDENTITY,
    //     width: patch[0][0].width * 2,
    //     shape: TileType::T,
    //     outline: new_t_outline,
    // });

    // let t_0 = new_t.add_node(patch[11]);
    // new_t.add_edge(t_root, t_0, ());

    // AllFour {
    //     h: new_h,
    //     t: new_t,
    //     p: new_p,
    //     f: new_f,
    // }
    new_h
}

fn rotate_colors_playground(
    mut query: Query<(&mut Fill, &mut TileType, &mut Transform)>,
    time: Res<Time>,
) {
    // let mut c = query.;
    // *c = Fill::color(Color::rgba(1.0, 1.0, 1.0, 0.1));
    for (mut fill, shape, mut transform) in query.iter_mut() {
        // if fill.color.r() > 0.9 || fill.color.g() > 0.9 || fill.color.b() > 0.9 {
        //     fill.color = Color::rgba(
        //         fill.color.r() - 0.01,
        //         fill.color.g() - 0.01,
        //         fill.color.b() - 0.01,
        //         1.0,
        //     );
        // } else if fill.color.r() < 0.1 || fill.color.g() < 0.1 || fill.color.b() < 0.1 {
        //     fill.color = fill.color + Color::rgba(0.01, 0.01, 0.01, 0.0);
        // }
        // let a = (((time.elapsed_seconds() * 100.0) as usize) % 100) as f32;
        // let a = a / 100.0;
        // transform.rotate_z(0.125_f32.to_radians());
        let tt = transform.translation / 2.0;
        transform.rotate_around(tt * -1.0, Quat::from_rotation_z((1.0_f32).to_radians()));
        let a = time.elapsed_seconds();
        let b = shape_to_fill_color(*shape);
        let c = Color::rgba(
            b.r() * (a.sin() + 1.0),
            b.g() * (a.cos() + 1.0),
            b.b() * ((a + PI).cos() + 1.0),
            1.0 * b.a(),
        );
        // let a = Color::rgba(
        //     (b.r() * a.sin()).abs(),
        //     (b.g() * a.sin()).abs(),
        //     (b.b() * a.sin()).abs(),
        //     1.0 * b.a(),
        // );
        // fill.color = Color::rgba(a.sin(), a.cos(), a.tan(), 1.0)
        // fill.color = Color::rgba(a, a, a, 1.0);
        fill.color = c;
    }
}

fn shape_to_fill_color(shape: TileType) -> Color {
    let tr = 0.4;
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

type TileEntity = (ShapeBundle, Fill, Stroke, TileType);
// type TileEntity = (SpriteBundle, TileType);

fn polygon_entity(tile: MetaTile, z: f32) -> TileEntity {
    let polygon = shapes::Polygon {
        // points: Vec::from(HAT_OUTLINE),
        points: tile
            .outline
            .iter()
            .map(|p| tile.transform.transform_point2(*p))
            .collect(),
        closed: true,
    };

    // return (
    //     SpriteBundle {
    //         texture: asset_server.load("hat-monotile.png"), // nope, performance is worse
    //         ..default()
    //     },
    //     tile.shape,
    // );
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&polygon),
            // transform: Transform::from_matrix(mat4_from_affine2(tile.transform, z)),
            ..default()
        },
        Fill::color(shape_to_fill_color(tile.shape)),
        Stroke::new(
            Color::rgba(0.0, 0.0, 0.0, 1.0),
            // 0.10 * (tile.width as f32), //.sqrt(),
            // 0.15,
            0.10 * (tile.width as f32).sqrt(),
        ),
        tile.shape,
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
