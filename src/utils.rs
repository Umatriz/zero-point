use bevy::{
    math::{Vec3, Vec4},
    render::color::Color,
};

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + b * t
}

pub fn inv_lerp(a: f64, b: f64, v: f64) -> f64 {
    (v - a) / (b - a)
}

pub fn color_lerp(color0: Color, color1: Color, t: f32) -> Color {
    let r0 = color0.r();
    let g0 = color0.g();
    let b0 = color0.b();
    let r1 = color1.r();
    let g1 = color1.g();
    let b1 = color1.b();

    Color::rgb(r0 + t * (r1 - r0), g0 + t * (g1 - g0), b0 + t * (b1 - b0))
}

pub fn color_lerp_linear(color0: Color, color1: Color, t: f32) -> Color {
    let l0 = color0.as_rgba_linear();
    let r0 = l0.r();
    let g0 = l0.g();
    let b0 = l0.b();

    let l1 = color1.as_rgba_linear();
    let r1 = l1.r();
    let g1 = l1.g();
    let b1 = l1.b();

    Color::rgb_linear(r0 + t * (r1 - r0), g0 + t * (g1 - g0), b0 + t * (b1 - b0))
}
