use nannou::prelude::*;
use shader_shared::{Uniforms, Vertex, Light};

use crate::signals::*;
use crate::helpers::*;

// struct Params {
//     speed: f32,
//     num_stripes: f32,
//     stripe_width: f32,
//     angle: f32,
//     smooth_width: f32,
// }

pub fn shader(v: Vertex , uniforms: &Uniforms) -> LinSrgb {
    let mut params = uniforms.params.line_gradient;

    if uniforms.use_midi {
        params.num_stripes = uniforms.slider1;
        params.angle = uniforms.slider2;
    }
    //params.angle = map_range(uniforms.time * 0.5, -1.0 ,1.0, 0.0, 0.5);

    let signal_type = Signal::TRIANGLE;
    let phase = (uniforms.time * params.speed).fract();
    
    let p = match v.light {
        Light::Wash{index} => pt2(v.position.x,v.position.z * 2.0 - 1.0),
        Light::Led{index,col_row,normalised_coords} => normalised_coords,
    };

    let x = map_range(p.x, -1.0, 1.0, 0.0, 1.0);
    let y = map_range(p.y, -1.0, 1.0, 0.0, 1.0);
    let uv = vec2(x,y);
    let mut rotated_uv = uv;
    
    // rotate context
    rotated_uv -= vec2(0.5, 0.5);
    rotated_uv = multiply_mat2_with_vec2(rotate_2d( (HALF_PI+params.angle*PI) * -rotated_uv.x.signum() ), rotated_uv);
    rotated_uv += vec2(0.5, 0.5);

    let mut line_phase = signal_type.amp(phase);
    match signal_type {
        Signal::Lfo(_) => {
            line_phase += HALF_PI*0.496;	
            line_phase *= PI;	
        },
        _ => (),
    };

    let mut stripe_uv = rotated_uv;
    let gradient = 1.0 - ((rotated_uv.y * (params.num_stripes*40.0)).floor() / (params.num_stripes*40.0) + line_phase).fract();
    stripe_uv.y = 1.0 - (stripe_uv.y * (params.num_stripes*40.0)).fract();
    let gradient_width = params.stripe_width * gradient;
    let mut stripes = smoothstep((1.0 - gradient_width) - params.smooth_width, (1.0 - gradient_width) + params.smooth_width, stripe_uv.y);
    stripes -= 1.0 - smoothstep(1.0, 1.0 - params.smooth_width * 2.0, stripe_uv.y);
    
    lin_srgb(stripes * gradient, stripes * gradient, stripes * gradient)
}
