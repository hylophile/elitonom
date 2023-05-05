use std::ops::Mul;

use bevy::{math::Affine2, prelude::*};

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

pub fn match_two(p1: Vec2, q1: Vec2, p2: Vec2, q2: Vec2) -> Affine2 {
    match_segment(p2, q2).mul(match_segment(p1, q1).inverse())
}

// #[test]
// fn test_rot() {
//     let p = Vec2::new(1.0, 0.0);
//     let res = rot_about(p, PI);
//     println!("{res}");
//     assert_f32_near!(res, [[-1.0, 0.0], [0.0, -1.0], [2.0, 0.0]]);
// }
pub fn rot_about(p: Vec2, angle: f32) -> Affine2 {
    Affine2::from_translation(p).mul(Affine2::from_angle(angle).mul(Affine2::from_translation(-p)))
}

pub fn intersect(p1: Vec2, q1: Vec2, p2: Vec2, q2: Vec2) -> Vec2 {
    let d = (q2.y - p2.y) * (q1.x - p1.x) - (q2.x - p2.x) * (q1.y - p1.y);
    let u_a = ((q2.x - p2.x) * (p1.y - p2.y) - (q2.y - p2.y) * (p1.x - p2.x)) / d;
    // const uB =
    //   ((q1.x - p1.x) * (p1.y - p2.y) - (q1.y - p1.y) * (p1.x - p2.x)) / d;

    return Vec2 {
        x: p1.x + u_a * (q1.x - p1.x),
        y: p1.y + u_a * (q1.y - p1.y),
    };
}
