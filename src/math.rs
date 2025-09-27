#![allow(unused)]

pub fn lerp(v0: f64, v1: f64, t: f64) -> f64 {
    (1.0 - t) * v0 + t * v1
}

pub fn bilerp(tl: f64, tr: f64, bl: f64, br: f64, u: f64, v: f64) -> f64 {
    let top = lerp(tl, tr, u);
    let bot = lerp(bl, br, u);
    lerp(top, bot, v)
}

pub fn smoothstep(x: f64) -> f64 {
    x * x * (3.0 - 2.0 * x)
}

pub fn smootherstep(x: f64) -> f64 {
    x * x * x * (x * (6.0 * x - 15.0) + 10.0)
}
