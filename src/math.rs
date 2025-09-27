#![allow(unused)]

pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

pub fn bilerp(tl: f32, tr: f32, bl: f32, br: f32, u: f32, v: f32) -> f32 {
    let top = lerp(tl, tr, u);
    let bot = lerp(bl, br, u);
    lerp(top, bot, v)
}

pub fn smoothstep(x: f32) -> f32 {
    x * x * (3.0 - 2.0 * x)
}

pub fn smootherstep(x: f32) -> f32 {
    x * x * x * (x * (6.0 * x - 15.0) + 10.0)
}
