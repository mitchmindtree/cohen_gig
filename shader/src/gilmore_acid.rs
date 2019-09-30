use nannou::prelude::*;
use shader_shared::Uniforms;

use crate::signals::*;
use crate::helpers::*;

// https://www.interactiveshaderformat.com/sketches/1680

struct Params {
    speed: f32,
    displace: f32,
    colour_offset: f32,
    grid_size: f32,
    wave: f32,
    zoom_amout: f32,
    rotation_amount: f32,
    brightness: f32,
    saturation: f32,
}

fn calc(tx: Vector2, t: f32, params: &Params, uniforms: &Uniforms) -> f32 {
    let angle = PI * nsin(t * 0.1) + PI / 6.0;
    let len = 1.0 - length(vec3(tx.x,tx.y,0.)) * nsin(t);
    let mut value = 0.0;
    let hex = coord_to_hex(tx, map_range(params.grid_size,0.0,1.0,0.0,200.0) * nsin(t * params.zoom_amout), angle * params.rotation_amount);
    for i in 0..3 {
        let offset = i as f32 / (3.0 + (uniforms.time*0.05).sin() * (params.colour_offset * 2.0));
        let cell = hex_to_cell(hex, 1.0 + i as f32);
        value += nsin(hex_to_float(cell, nsin(len + t + offset)) * map_range(params.wave, 0.0, 1.0, 0.0, 10.0) * nsin(t * 0.5 + offset) + len + t);
    }
    let v = value / 3.0;
    v
}

pub fn shader(p: Vector3, uniforms: &Uniforms) -> LinSrgb {
    let params = Params {
        speed: 0.05,
        displace: 0.01,
        colour_offset: 0.85,
        grid_size: 0.345,
        wave: 0.088,
        zoom_amout: 0.0,
        rotation_amount: 0.0,
        brightness: 2.0,
        saturation: 0.15,
    };

    let t = uniforms.time * params.speed;
    
    let x = map_range(p.x, -0.13, 0.13, -1.0, 1.0);
    let y = map_range(p.y, 0.3, 1.0, -1.0, 1.0);
    let mut uv = vec2(x,y);
    uv.x *= uniforms.resolution.x / uniforms.resolution.y;
    let mut rgb = vec![0.0,0.0,0.0];
    for i in 0..3 {
        let t2 = t + i as f32 * params.displace;
        rgb[i] += calc(uv, t2, &params, &uniforms).powf(map_range(params.saturation,0.0,1.0,5.0,1.0));
    }
    
    lin_srgb(rgb[0] * params.brightness, rgb[1] * params.brightness, rgb[2] * params.brightness)
}
