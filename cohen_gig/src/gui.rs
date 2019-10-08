use crate::conf::Config;
use crate::{shader, Osc, State};
use nannou::prelude::*;
use nannou::ui::conrod_core::widget_ids;
use nannou::ui::prelude::*;
use std::f64::consts::PI;
use std::net::SocketAddr;

pub const COLUMN_W: Scalar = 240.0;
pub const DEFAULT_WIDGET_H: Scalar = 30.0;
pub const DEFAULT_SLIDER_H: Scalar = 20.0;
pub const PAD: Scalar = 20.0;
pub const WINDOW_WIDTH: u32 = ((COLUMN_W + PAD * 2.0) as u32);
pub const WINDOW_HEIGHT: u32 = 1080 - (2.0 * PAD) as u32;
pub const WIDGET_W: Scalar = COLUMN_W;
pub const HALF_WIDGET_W: Scalar = WIDGET_W * 0.5 - PAD * 0.25;
pub const THIRD_WIDGET_W: Scalar = WIDGET_W * 0.33 - PAD * 0.25;

widget_ids! {
    pub struct Ids {
        background,
        scrollbar,
        title_text,
        dmx_button,
        osc_button,
        midi_button,
        osc_address_text,
        osc_address_text_box,
        shader_title_text,
        shader_state_text,

        wash_dmx_addrs_text,
        wash_dmx_addrs_list,

        led_shader_left_text,
        led_shader_left_ddl,

        led_shader_right_text,
        led_shader_right_ddl,

        colour_post_process_text,
        colour_post_process_ddl,

        blend_mode_text,
        blend_mode_ddl,

        shader_mix_left_right,
        led_fade_to_black,
        wash_fade_to_black,
    }
}


widget_ids! {
    pub struct AcidGradientIds {
        speed,
        zoom,
        offset,
    }
}

widget_ids! {
    pub struct BlinkyCirclesIds {
        speed,
        zoom,
        offset,
    }
}

widget_ids! {
    pub struct BwGradientIds {
        speed,
        dc,
        amp,
        freq,
        mirror,
    }
}

widget_ids! {
    pub struct ColourGridIds {
        speed,
        zoom_amount,
    }
}

widget_ids! {
    pub struct EscherTilingsIds {
        speed,
        scale,
        shape_iter,
    }
}

widget_ids! {
    pub struct GilmoreAcidIds {
        speed,
        displace,
        colour_offset,
        grid_size,
        wave,
        zoom_amount,
        rotation_amount,
        brightness,
        saturation,
    }
}

widget_ids! {
    pub struct JustRelaxIds {
        speed,
        shape_offset,
        iter,
    }
}

widget_ids! {
    pub struct LifeLedWallIds {
        speed,
        size,
        red,
        green,
        blue,
        saturation,
        colour_offset,
    }
}

widget_ids! {
    pub struct LineGradientIds {
        speed,
        num_stripes,
        stripe_width,
        angle,
        smooth_width,
    }
}

widget_ids! {
    pub struct MetafallIds {
        speed,
        scale,
        red,
        green,
        blue,
    }
}

widget_ids! {
    pub struct ParticleZoomIds {
        speed,
        density,
        shape,
        tau,
    }
}

widget_ids! {
    pub struct RadialLinesIds {
        speed,
        zoom_amount,
    }
}

widget_ids! {
    pub struct SatisSpiralingIds {
        speed,
        loops,
        mirror,
        rotate,
    }
}

widget_ids! {
    pub struct SpiralIntersectIds {
        speed,
        g1,
        g2,
        rot1,
        rot2,
        colours,
    }
}

widget_ids! {
    pub struct SquareTunnelIds {
        speed,
        rotation_speed,
        rotation_offset,
        zoom,
    }
}

widget_ids! {
    pub struct ThePulseIds {
        speed,
        scale,
        colour_iter,
        thickness,
    }
}

widget_ids! {
    pub struct TunnelProjectionIds {
        speed,
        res,
    }
}

widget_ids! {
    pub struct VertColourGradientIds {
        speed,
        scale,
        colour_iter,
        line_amp,
        diag_amp,
        boarder_amp,
    }
}

widget_ids! {
    pub struct SolidHsvColourIds {
        hue,
        saturation,
        value,
    }
}

widget_ids! {
    pub struct SolidRgbColourIds {
        red,
        green,
        blue,
    }
}

/// Update the user interface.
pub fn update(
    ref mut ui: UiCell,
    state: &mut State,
    config: &mut Config,
    osc: &mut Osc,
    since_start: std::time::Duration,
    shader_activity: shader::Activity,
    ids: &Ids,
    acid_gradient_ids: &AcidGradientIds,
    blinky_circles_ids: &BlinkyCirclesIds,
    bw_gradient_ids: &BwGradientIds,
    colour_grid_ids: &ColourGridIds,
    escher_tilings_ids: &EscherTilingsIds,
    gilmore_acid_ids: &GilmoreAcidIds,
    just_relax_ids: &JustRelaxIds,
    life_led_wall_ids: &LifeLedWallIds,
    line_gradient_ids: &LineGradientIds,
    metafall_ids: &MetafallIds,
    particle_zoom_ids: &ParticleZoomIds,
    radial_lines_ids: &RadialLinesIds,
    satis_spiraling_ids: &SatisSpiralingIds,
    spiral_intersect_ids: &SpiralIntersectIds,
    square_tunnel_ids: &SquareTunnelIds,
    the_pulse_ids: &ThePulseIds,
    tunnel_projection_ids: &TunnelProjectionIds,
    vert_colour_gradient_ids: &VertColourGradientIds,
    solid_hsv_colour_ids: &SolidHsvColourIds,
    solid_rgb_colour_ids: &SolidRgbColourIds,
) {
    widget::Canvas::new()
        .border(0.0)
        .rgb(0.1, 0.1, 0.1)
        .pad(PAD)
        .scroll_kids_vertically()
        .set(ids.background, ui);

    text("COHEN GIG")
        .mid_top_of(ids.background)
        .set(ids.title_text, ui);

    if button()
        .color(toggle_color(config.dmx_on))
        .label("DMX")
        .w(THIRD_WIDGET_W)
        .mid_left_of(ids.background)
        .down(PAD * 1.5)
        .set(ids.dmx_button, ui)
        .was_clicked()
    {
        config.dmx_on = !config.dmx_on;
    }

    if button()
        .color(toggle_color(config.osc_on))
        .label("OSC")
        .right(PAD * 0.5)
        .w(THIRD_WIDGET_W)
        .set(ids.osc_button, ui)
        .was_clicked()
    {
        config.osc_on = !config.osc_on;
    }

    if button()
        .color(toggle_color(config.midi_on))
        .label("MIDI")
        .right(PAD * 0.5)
        .w(THIRD_WIDGET_W)
        .set(ids.midi_button, ui)
        .was_clicked()
    {
        config.midi_on = !config.midi_on;
    }

    text("OSC Address")
        .mid_left_of(ids.background)
        .down(PAD * 1.5)
        .set(ids.osc_address_text, ui);

    let color = match state.osc_addr_textbox_string.parse::<SocketAddr>() {
        Ok(socket) => {
            match osc.addr == socket {
                true => color::BLACK,
                false => color::DARK_GREEN.with_luminance(0.1),
            }
        }
        Err(_) => color::DARK_RED.with_luminance(0.1),
    };
    for event in widget::TextBox::new(&state.osc_addr_textbox_string)
        .w_h(WIDGET_W, DEFAULT_WIDGET_H)
        .border(0.0)
        .color(color)
        .text_color(color::WHITE)
        .font_size(14)
        .set(ids.osc_address_text_box, ui)
    {
        match event {
            widget::text_box::Event::Update(string) => state.osc_addr_textbox_string = string,
            widget::text_box::Event::Enter => {
                if let Ok(socket) = state.osc_addr_textbox_string.parse() {
                    osc.addr = socket;
                }
            },
        }
    }

    text("Shader State")
        .mid_left_of(ids.background)
        .down(PAD * 1.5)
        .set(ids.shader_title_text, ui);

    let (string, color) = match shader_activity {
        shader::Activity::Incoming => {
            let s = "Compiling".into();
            let l = (since_start.secs() * 2.0 * PI).sin() * 0.35 + 0.5;
            let c = ui::color::YELLOW.with_luminance(l as _);
            (s, c)
        },
        shader::Activity::LastIncoming(last) => match last {
            shader::LastIncoming::Succeeded => {
                let s = "Succeeded".into();
                let c = ui::color::GREEN;
                (s, c)
            },
            shader::LastIncoming::Failed(_err) => {
                let s = format!("Compilation Failed");
                let c = ui::color::RED;
                (s, c)
            },
        }
    };
    text(&string)
        .color(color)
        .down(PAD)
        .set(ids.shader_state_text, ui);

    text("Wash and Spot DMX Addrs")
        .mid_left_of(ids.background)
        .down(PAD * 1.5)
        .set(ids.wash_dmx_addrs_text, ui);

    let wash_count = config.wash_dmx_addrs.len();
    let spot_count = config.spot_dmx_addrs.len();
    let n_items = wash_count + spot_count;
    let (mut items, scrollbar) = widget::List::flow_down(n_items)
        .item_size(DEFAULT_WIDGET_H)
        .scrollbar_next_to()
        .h(DEFAULT_WIDGET_H * 4.0)
        .mid_left_of(ids.background)
        .down(PAD)
        .w(COLUMN_W)
        .set(ids.wash_dmx_addrs_list, ui);

    while let Some(item) = items.next(ui) {
        let i = item.i;
        let is_wash = i < wash_count;
        let light_i = if is_wash { i } else { i - wash_count };
        let label = match is_wash {
            true => format!("Wash {}", light_i),
            false => format!("Spot {}", light_i),
        };
        let v = match is_wash {
            true => config.wash_dmx_addrs[light_i],
            false => config.spot_dmx_addrs[light_i],
        };
        let min = 0.0;
        let max = (crate::DMX_ADDRS_PER_UNIVERSE - 1) as f32;
        let precision = 0;
        let dialer = widget::NumberDialer::new(v as f32, min, max, precision)
            .border(0.0)
            .label(&label)
            .label_color(color::WHITE)
            .label_font_size(14)
            .color(color::DARK_CHARCOAL);
        for v in item.set(dialer, ui) {
            match is_wash {
                true => config.wash_dmx_addrs[light_i] = v as u8,
                false => config.spot_dmx_addrs[light_i] = v as u8,
            }
        }
    }

    if let Some(s) = scrollbar { s.set(ui) }


    //---------------------- LED SHADER LEFT
    text("LED Shader Left")
        .mid_left_of(ids.background)
        .down(PAD * 1.5)
        .color(color::WHITE)
        .set(ids.led_shader_left_text, ui);

    for selected_idx in widget::DropDownList::new(&state.shader_names, state.led_shader_idx_left)
        .w_h(COLUMN_W, PAD * 2.0)
        .down(10.0)
        .max_visible_items(15)
        .rgb(0.176, 0.513, 0.639)
        .label("LED Shader Preset")
        .label_font_size(15)
        .label_rgb(1.0, 1.0, 1.0)
        .scrollbar_on_top()
        .set(ids.led_shader_left_ddl, ui)
    {
        state.led_shader_idx_left = Some(selected_idx);
    }

    match state.shader_names[state.led_shader_idx_left.unwrap()].as_str() {
        "AcidGradient" => set_acid_gradient_widgets(ui, &acid_gradient_ids, state),
        "BlinkyCircles" => set_blinky_circles_widgets(ui, &blinky_circles_ids, state),
        "BwGradient" => set_bw_gradient_widgets(ui, &bw_gradient_ids, state),
        "ColourGrid" => set_colour_grid_widgets(ui, &colour_grid_ids, state),
        "EscherTilings" => set_escher_tilings_widgets(ui, &escher_tilings_ids, state),
        "GilmoreAcid" => set_gilmore_acid_widgets(ui, &gilmore_acid_ids, state),
        "JustRelax" => set_just_relax_widgets(ui, &just_relax_ids, state),
        "LifeLedWall" => set_life_led_wall_widgets(ui, &life_led_wall_ids, state),
        "LineGradient" => set_line_gradient_widgets(ui, &line_gradient_ids, state),
        "Metafall" => set_metafall_widgets(ui, &metafall_ids, state),
        "ParticleZoom" => set_particle_zoom_widgets(ui, &particle_zoom_ids, state),
        "RadialLines" => set_radial_lines_widgets(ui, &radial_lines_ids, state),
        "SatisSpiraling" => set_satis_spiraling_widgets(ui, &satis_spiraling_ids, state),
        "SpiralIntersect" => set_spiral_intersect_widgets(ui, &spiral_intersect_ids, state),
        "SquareTunnel" => set_square_tunnel_widgets(ui, &square_tunnel_ids, state),
        "ThePulse" => set_the_pulse_widgets(ui, &the_pulse_ids, state),
        "TunnelProjection" => set_tunnel_projection_widgets(ui, &tunnel_projection_ids, state),
        "VertColourGradient" => set_vert_colour_gradient_widgets(ui, &vert_colour_gradient_ids, state),
        "SolidHsvColour" => set_solid_hsv_colour_widgets(ui, &solid_hsv_colour_ids, state),
        "SolidRgbColour" => set_solid_rgb_colour_widgets(ui, &solid_rgb_colour_ids, state),
        _ => (),
    }

    //---------------------- LED SHADER RIGHT
    text("LED Shader Right")
        .down(20.0)
        .color(color::WHITE)
        .set(ids.led_shader_right_text, ui);

    for selected_idx in widget::DropDownList::new(&state.shader_names, state.led_shader_idx_right)
        .w_h(COLUMN_W, PAD * 2.0)
        .down(10.0)
        .max_visible_items(15)
        .rgb(0.176, 0.513, 0.639)
        .label("LED Shader Preset")
        .label_font_size(15)
        .label_rgb(1.0, 1.0, 1.0)
        .scrollbar_on_top()
        .set(ids.led_shader_right_ddl, ui)
    {
        state.led_shader_idx_right = Some(selected_idx);
    }

    match state.shader_names[state.led_shader_idx_right.unwrap()].as_str() {
        "AcidGradient" => set_acid_gradient_widgets(ui, &acid_gradient_ids, state),
        "BlinkyCircles" => set_blinky_circles_widgets(ui, &blinky_circles_ids, state),
        "BwGradient" => set_bw_gradient_widgets(ui, &bw_gradient_ids, state),
        "ColourGrid" => set_colour_grid_widgets(ui, &colour_grid_ids, state),
        "EscherTilings" => set_escher_tilings_widgets(ui, &escher_tilings_ids, state),
        "GilmoreAcid" => set_gilmore_acid_widgets(ui, &gilmore_acid_ids, state),
        "JustRelax" => set_just_relax_widgets(ui, &just_relax_ids, state),
        "LifeLedWall" => set_life_led_wall_widgets(ui, &life_led_wall_ids, state),
        "LineGradient" => set_line_gradient_widgets(ui, &line_gradient_ids, state),
        "Metafall" => set_metafall_widgets(ui, &metafall_ids, state),
        "ParticleZoom" => set_particle_zoom_widgets(ui, &particle_zoom_ids, state),
        "RadialLines" => set_radial_lines_widgets(ui, &radial_lines_ids, state),
        "SatisSpiraling" => set_satis_spiraling_widgets(ui, &satis_spiraling_ids, state),
        "SpiralIntersect" => set_spiral_intersect_widgets(ui, &spiral_intersect_ids, state),
        "SquareTunnel" => set_square_tunnel_widgets(ui, &square_tunnel_ids, state),
        "ThePulse" => set_the_pulse_widgets(ui, &the_pulse_ids, state),
        "TunnelProjection" => set_tunnel_projection_widgets(ui, &tunnel_projection_ids, state),
        "VertColourGradient" => set_vert_colour_gradient_widgets(ui, &vert_colour_gradient_ids, state),
        "SolidHsvColour" => set_solid_hsv_colour_widgets(ui, &solid_hsv_colour_ids, state),
        "SolidRgbColour" => set_solid_rgb_colour_widgets(ui, &solid_rgb_colour_ids, state),
        _ => (),
    }

    //---------------------- COLOUR POST PROCESS SHADER
    text("Colour Post Process")
        .down(20.0)
        .color(color::WHITE)
        .set(ids.colour_post_process_text, ui);

    for selected_idx in widget::DropDownList::new(&state.solid_colour_names, state.solid_colour_idx)
        .w_h(COLUMN_W, PAD * 2.0)
        .down(10.0)
        .max_visible_items(15)
        .rgb(0.176, 0.513, 0.639)
        .label("Wash Shader Preset")
        .label_font_size(15)
        .label_rgb(1.0, 1.0, 1.0)
        .scrollbar_on_top()
        .set(ids.colour_post_process_ddl, ui)
    {
        state.solid_colour_idx = Some(selected_idx);
    }

    match state.solid_colour_names[state.solid_colour_idx.unwrap()].as_str() {
        "SolidHsvColour" => set_solid_hsv_colour_widgets(ui, &solid_hsv_colour_ids, state),
        "SolidRgbColour" => set_solid_rgb_colour_widgets(ui, &solid_rgb_colour_ids, state),
        _ => (),
    }

    //---------------------- BLEND MODES
    text("LED Blend Mode")
        .down(20.0)
        .color(color::WHITE)
        .set(ids.blend_mode_text, ui);

    for selected_idx in widget::DropDownList::new(&state.blend_mode_names, state.blend_mode_idx)
        .w_h(COLUMN_W, PAD * 2.0)
        .down(10.0)
        .max_visible_items(15)
        .rgb(0.176, 0.513, 0.639)
        .label("Wash Shader Preset")
        .label_font_size(15)
        .label_rgb(1.0, 1.0, 1.0)
        .scrollbar_on_top()
        .set(ids.blend_mode_ddl, ui)
    {
        state.blend_mode_idx = Some(selected_idx);
    }

    for value in slider(state.led_left_right_mix, -1.0, 1.0)
        .down(10.0)
        .label("Left Right Mix")
        .set(ids.shader_mix_left_right, ui)
    {
        state.led_left_right_mix = value;
    }

    for value in slider(state.led_fade_to_black, 0.0, 1.0)
        .down(10.0)
        .label("LED Fade to Black")
        .set(ids.led_fade_to_black, ui)
    {
        state.led_fade_to_black = value;
    }

    for value in slider(state.wash_fade_to_black, 0.0, 1.0)
        .down(10.0)
        .label("Wash Fade to Black")
        .set(ids.wash_fade_to_black, ui)
    {
        state.wash_fade_to_black = value;
    }

    // A scrollbar for the canvas.
    widget::Scrollbar::y_axis(ids.background).auto_hide(true).set(ids.scrollbar, ui);
}

pub fn set_acid_gradient_widgets(ui: &mut UiCell, ids: &AcidGradientIds, state: &mut State) {
    for value in slider(state.shader_params.acid_gradient.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.acid_gradient.speed = value;
    }
    for value in slider(state.shader_params.acid_gradient.zoom, 0.0, 1.0)
        .down(10.0)
        .label("Zoom")
        .set(ids.zoom, ui)
    {
        state.shader_params.acid_gradient.zoom = value;
    }
    for value in slider(state.shader_params.acid_gradient.offset, 0.0, 1.0)
        .down(10.0)
        .label("Offset")
        .set(ids.offset, ui)
    {
        state.shader_params.acid_gradient.offset = value;
    }
}

pub fn set_blinky_circles_widgets(ui: &mut UiCell, ids: &BlinkyCirclesIds, state: &mut State) {
    for value in slider(state.shader_params.blinky_circles.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.blinky_circles.speed = value;
    }
    for value in slider(state.shader_params.blinky_circles.zoom, 0.0, 1.0)
        .down(10.0)
        .label("Zoom")
        .set(ids.zoom, ui)
    {
        state.shader_params.blinky_circles.zoom = value;
    }
    for value in slider(state.shader_params.blinky_circles.offset, 0.0, 1.0)
        .down(10.0)
        .label("Offset")
        .set(ids.offset, ui)
    {
        state.shader_params.blinky_circles.offset = value;
    }
}

pub fn set_bw_gradient_widgets(ui: &mut UiCell, ids: &BwGradientIds, state: &mut State) {
    for value in slider(state.shader_params.bw_gradient.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.bw_gradient.speed = value;
    }
    for value in slider(state.shader_params.bw_gradient.dc, 0.0, 1.0)
        .down(10.0)
        .label("Dc")
        .set(ids.dc, ui)
    {
        state.shader_params.bw_gradient.dc = value;
    }
    for value in slider(state.shader_params.bw_gradient.amp, 0.0, 1.0)
        .down(10.0)
        .label("Amp")
        .set(ids.amp, ui)
    {
        state.shader_params.bw_gradient.amp = value;
    }
    for value in slider(state.shader_params.bw_gradient.freq, 0.0, 1.0)
        .down(10.0)
        .label("Freq")
        .set(ids.freq, ui)
    {
        state.shader_params.bw_gradient.freq = value;
    }
    for value in toggle(state.shader_params.bw_gradient.mirror)
        .down(10.0)
        .w_h(COLUMN_W, PAD)
        .label("Mirror")
        .set(ids.mirror, ui)
    {
        state.shader_params.bw_gradient.mirror = value;
    }
}

pub fn set_colour_grid_widgets(ui: &mut UiCell, ids: &ColourGridIds, state: &mut State) {
    for value in slider(state.shader_params.colour_grid.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.colour_grid.speed = value;
    }
    for value in slider(state.shader_params.colour_grid.zoom_amount, 0.0, 1.0)
        .down(10.0)
        .label("Zoom Amount")
        .set(ids.zoom_amount, ui)
    {
        state.shader_params.colour_grid.zoom_amount = value;
    }
}

pub fn set_escher_tilings_widgets(ui: &mut UiCell, ids: &EscherTilingsIds, state: &mut State) {
    for value in slider(state.shader_params.escher_tilings.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.escher_tilings.speed = value;
    }
    for value in slider(state.shader_params.escher_tilings.scale, 0.0, 1.0)
        .down(10.0)
        .label("Scale")
        .set(ids.scale, ui)
    {
        state.shader_params.escher_tilings.scale = value;
    }
    for value in slider(state.shader_params.escher_tilings.shape_iter, 0.0, 1.0)
        .down(10.0)
        .label("Shape Iter")
        .set(ids.shape_iter, ui)
    {
        state.shader_params.escher_tilings.shape_iter = value;
    }
}

pub fn set_gilmore_acid_widgets(ui: &mut UiCell, ids: &GilmoreAcidIds, state: &mut State) {
    for value in slider(state.shader_params.gilmore_acid.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.gilmore_acid.speed = value;
    }
    for value in slider(state.shader_params.gilmore_acid.displace, 0.0, 1.0)
        .down(10.0)
        .label("Displace")
        .set(ids.displace, ui)
    {
        state.shader_params.gilmore_acid.displace = value;
    }
    for value in slider(state.shader_params.gilmore_acid.colour_offset, 0.0, 1.0)
        .down(10.0)
        .label("Colour Offset")
        .set(ids.colour_offset, ui)
    {
        state.shader_params.gilmore_acid.colour_offset = value;
    }
    for value in slider(state.shader_params.gilmore_acid.grid_size, 0.0, 1.0)
        .down(10.0)
        .label("Grid Size")
        .set(ids.grid_size, ui)
    {
        state.shader_params.gilmore_acid.grid_size = value;
    }
    for value in slider(state.shader_params.gilmore_acid.wave, 0.0, 1.0)
        .down(10.0)
        .label("Wave")
        .set(ids.wave, ui)
    {
        state.shader_params.gilmore_acid.wave = value;
    }
    for value in slider(state.shader_params.gilmore_acid.zoom_amount, 0.0, 1.0)
        .down(10.0)
        .label("Zoom Amount")
        .set(ids.zoom_amount, ui)
    {
        state.shader_params.gilmore_acid.zoom_amount = value;
    }
    for value in slider(state.shader_params.gilmore_acid.rotation_amount, 0.0, 1.0)
        .down(10.0)
        .label("Rotation Amount")
        .set(ids.rotation_amount, ui)
    {
        state.shader_params.gilmore_acid.rotation_amount = value;
    }
    for value in slider(state.shader_params.gilmore_acid.brightness, 0.0, 1.0)
        .down(10.0)
        .label("Brightness")
        .set(ids.brightness, ui)
    {
        state.shader_params.gilmore_acid.brightness = value;
    }
    for value in slider(state.shader_params.gilmore_acid.saturation, 0.0, 1.0)
        .down(10.0)
        .label("Saturation")
        .set(ids.saturation, ui)
    {
        state.shader_params.gilmore_acid.saturation = value;
    }
}

pub fn set_just_relax_widgets(ui: &mut UiCell, ids: &JustRelaxIds, state: &mut State) {
    for value in slider(state.shader_params.just_relax.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.just_relax.speed = value;
    }
    for value in slider(state.shader_params.just_relax.shape_offset, 0.0, 1.0)
        .down(10.0)
        .label("Shape Offset")
        .set(ids.shape_offset, ui)
    {
        state.shader_params.just_relax.shape_offset = value;
    }
    for value in slider(state.shader_params.just_relax.iter, 0.0, 1.0)
        .down(10.0)
        .label("Iter")
        .set(ids.iter, ui)
    {
        state.shader_params.just_relax.iter = value;
    }
}

pub fn set_life_led_wall_widgets(ui: &mut UiCell, ids: &LifeLedWallIds, state: &mut State) {
    for value in slider(state.shader_params.life_led_wall.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.life_led_wall.speed = value;
    }
    for value in slider(state.shader_params.life_led_wall.size, 0.0, 1.0)
        .down(10.0)
        .label("Size")
        .set(ids.size, ui)
    {
        state.shader_params.life_led_wall.size = value;
    }
    for value in slider(state.shader_params.life_led_wall.red, 0.0, 1.0)
        .down(10.0)
        .label("Red")
        .set(ids.red, ui)
    {
        state.shader_params.life_led_wall.red = value;
    }
    for value in slider(state.shader_params.life_led_wall.green, 0.0, 1.0)
        .down(10.0)
        .label("Green")
        .set(ids.green, ui)
    {
        state.shader_params.life_led_wall.green = value;
    }
    for value in slider(state.shader_params.life_led_wall.blue, 0.0, 1.0)
        .down(10.0)
        .label("Blue")
        .set(ids.blue, ui)
    {
        state.shader_params.life_led_wall.blue = value;
    }
    for value in slider(state.shader_params.life_led_wall.saturation, 0.0, 1.0)
        .down(10.0)
        .label("Saturation")
        .set(ids.saturation, ui)
    {
        state.shader_params.life_led_wall.saturation = value;
    }
    for value in slider(state.shader_params.life_led_wall.colour_offset, 0.0, 1.0)
        .down(10.0)
        .label("Colour Offset")
        .set(ids.colour_offset, ui)
    {
        state.shader_params.life_led_wall.colour_offset = value;
    }
}

pub fn set_line_gradient_widgets(ui: &mut UiCell, ids: &LineGradientIds, state: &mut State) {
    for value in slider(state.shader_params.line_gradient.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.line_gradient.speed = value;
    }
    for value in slider(state.shader_params.line_gradient.num_stripes, 0.0, 1.0)
        .down(10.0)
        .label("Num Stripes")
        .set(ids.num_stripes, ui)
    {
        state.shader_params.line_gradient.num_stripes = value;
    }
    for value in slider(state.shader_params.line_gradient.stripe_width, 0.0, 1.0)
        .down(10.0)
        .label("Stripe Width")
        .set(ids.stripe_width, ui)
    {
        state.shader_params.line_gradient.stripe_width = value;
    }
    for value in slider(state.shader_params.line_gradient.angle, 0.0, 1.0)
        .down(10.0)
        .label("Angle")
        .set(ids.angle, ui)
    {
        state.shader_params.line_gradient.angle = value;
    }
    for value in slider(state.shader_params.line_gradient.smooth_width, 0.0, 1.0)
        .down(10.0)
        .label("Smooth Width")
        .set(ids.smooth_width, ui)
    {
        state.shader_params.line_gradient.smooth_width = value;
    }
}

pub fn set_metafall_widgets(ui: &mut UiCell, ids: &MetafallIds, state: &mut State) {
    for value in slider(state.shader_params.metafall.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.metafall.speed = value;
    }
    for value in slider(state.shader_params.metafall.scale, 0.0, 1.0)
        .down(10.0)
        .label("Scale")
        .set(ids.scale, ui)
    {
        state.shader_params.metafall.scale = value;
    }
    for value in slider(state.shader_params.metafall.red, 0.0, 1.0)
        .down(10.0)
        .label("Red")
        .set(ids.red, ui)
    {
        state.shader_params.metafall.red = value;
    }
    for value in slider(state.shader_params.metafall.green, 0.0, 1.0)
        .down(10.0)
        .label("Green")
        .set(ids.green, ui)
    {
        state.shader_params.metafall.green = value;
    }
    for value in slider(state.shader_params.metafall.blue, 0.0, 1.0)
        .down(10.0)
        .label("Blue")
        .set(ids.blue, ui)
    {
        state.shader_params.metafall.blue = value;
    }
}

pub fn set_particle_zoom_widgets(ui: &mut UiCell, ids: &ParticleZoomIds, state: &mut State) {
    for value in slider(state.shader_params.particle_zoom.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.particle_zoom.speed = value;
    }
    for value in slider(state.shader_params.particle_zoom.density, 0.0, 1.0)
        .down(10.0)
        .label("Density")
        .set(ids.density, ui)
    {
        state.shader_params.particle_zoom.density = value;
    }
    for value in slider(state.shader_params.particle_zoom.shape, 0.0, 1.0)
        .down(10.0)
        .label("Shape")
        .set(ids.shape, ui)
    {
        state.shader_params.particle_zoom.shape = value;
    }
    for value in slider(state.shader_params.particle_zoom.tau, 0.0, 1.0)
        .down(10.0)
        .label("Tau")
        .set(ids.tau, ui)
    {
        state.shader_params.particle_zoom.tau = value;
    }
}

pub fn set_radial_lines_widgets(ui: &mut UiCell, ids: &RadialLinesIds, state: &mut State) {
    for value in slider(state.shader_params.radial_lines.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.radial_lines.speed = value;
    }
    for value in slider(state.shader_params.radial_lines.zoom_amount, 0.0, 1.0)
        .down(10.0)
        .label("Zoom Amount")
        .set(ids.zoom_amount, ui)
    {
        state.shader_params.radial_lines.zoom_amount = value;
    }
}

pub fn set_satis_spiraling_widgets(ui: &mut UiCell, ids: &SatisSpiralingIds, state: &mut State) {
    for value in slider(state.shader_params.satis_spiraling.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.satis_spiraling.speed = value;
    }
    for value in slider(state.shader_params.satis_spiraling.loops, 0.0, 1.0)
        .down(10.0)
        .label("Loops")
        .set(ids.loops, ui)
    {
        state.shader_params.satis_spiraling.loops = value;
    }
    for value in toggle(state.shader_params.satis_spiraling.mirror)
        .down(10.0)
        .w_h(COLUMN_W, PAD)
        .label("Mirror")
        .set(ids.mirror, ui)
    {
        state.shader_params.satis_spiraling.mirror = value;
    }
    for value in toggle(state.shader_params.satis_spiraling.rotate)
        .down(10.0)
        .w_h(COLUMN_W, PAD)
        .label("Rotate")
        .set(ids.rotate, ui)
    {
        state.shader_params.satis_spiraling.rotate = value;
    }
}

pub fn set_spiral_intersect_widgets(ui: &mut UiCell, ids: &SpiralIntersectIds, state: &mut State) {
    for value in slider(state.shader_params.spiral_intersect.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.spiral_intersect.speed = value;
    }
    for value in slider(state.shader_params.spiral_intersect.g1, 0.0, 1.0)
        .down(10.0)
        .label("G1")
        .set(ids.g1, ui)
    {
        state.shader_params.spiral_intersect.g1 = value;
    }
    for value in slider(state.shader_params.spiral_intersect.g2, 0.0, 1.0)
        .down(10.0)
        .label("G2")
        .set(ids.g2, ui)
    {
        state.shader_params.spiral_intersect.g2 = value;
    }
    for value in slider(state.shader_params.spiral_intersect.rot1, 0.0, 1.0)
        .down(10.0)
        .label("Rot1")
        .set(ids.rot1, ui)
    {
        state.shader_params.spiral_intersect.rot1 = value;
    }
    for value in slider(state.shader_params.spiral_intersect.rot2, 0.0, 1.0)
        .down(10.0)
        .label("Rot2")
        .set(ids.rot2, ui)
    {
        state.shader_params.spiral_intersect.rot2 = value;
    }
    for value in slider(state.shader_params.spiral_intersect.colours, 0.0, 1.0)
        .down(10.0)
        .label("Colours")
        .set(ids.colours, ui)
    {
        state.shader_params.spiral_intersect.colours = value;
    }
}

pub fn set_square_tunnel_widgets(ui: &mut UiCell, ids: &SquareTunnelIds, state: &mut State) {
    for value in slider(state.shader_params.square_tunnel.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.square_tunnel.speed = value;
    }
    for value in slider(state.shader_params.square_tunnel.rotation_speed, 0.0, 1.0)
        .down(10.0)
        .label("Rotation Speed")
        .set(ids.rotation_speed, ui)
    {
        state.shader_params.square_tunnel.rotation_speed = value;
    }
    for value in slider(state.shader_params.square_tunnel.rotation_offset, 0.0, 1.0)
        .down(10.0)
        .label("Rotation Offset")
        .set(ids.rotation_offset, ui)
    {
        state.shader_params.square_tunnel.rotation_offset = value;
    }
    for value in slider(state.shader_params.square_tunnel.zoom, 0.0, 1.0)
        .down(10.0)
        .label("Zoom")
        .set(ids.zoom, ui)
    {
        state.shader_params.square_tunnel.zoom = value;
    }
}

pub fn set_the_pulse_widgets(ui: &mut UiCell, ids: &ThePulseIds, state: &mut State) {
    for value in slider(state.shader_params.the_pulse.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.the_pulse.speed = value;
    }
    for value in slider(state.shader_params.the_pulse.scale, 0.0, 1.0)
        .down(10.0)
        .label("Scale")
        .set(ids.scale, ui)
    {
        state.shader_params.the_pulse.scale = value;
    }
    for value in slider(state.shader_params.the_pulse.colour_iter, 0.0, 1.0)
        .down(10.0)
        .label("Colour Iter")
        .set(ids.colour_iter, ui)
    {
        state.shader_params.the_pulse.colour_iter = value;
    }
    for value in slider(state.shader_params.the_pulse.thickness, 0.0, 1.0)
        .down(10.0)
        .label("Thickness")
        .set(ids.thickness, ui)
    {
        state.shader_params.the_pulse.thickness = value;
    }
}

pub fn set_tunnel_projection_widgets(ui: &mut UiCell, ids: &TunnelProjectionIds, state: &mut State) {
    for value in slider(state.shader_params.tunnel_projection.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.tunnel_projection.speed = value;
    }
    for value in slider(state.shader_params.tunnel_projection.res, 0.0, 1.0)
        .down(10.0)
        .label("Resolution")
        .set(ids.res, ui)
    {
        state.shader_params.tunnel_projection.res = value;
    }
}

pub fn set_vert_colour_gradient_widgets(ui: &mut UiCell, ids: &VertColourGradientIds, state: &mut State) {
    for value in slider(state.shader_params.vert_colour_gradient.speed, 0.0, 1.0)
        .down(10.0)
        .label("Speed")
        .set(ids.speed, ui)
    {
        state.shader_params.vert_colour_gradient.speed = value;
    }
    for value in slider(state.shader_params.vert_colour_gradient.scale, 0.0, 1.0)
        .down(10.0)
        .label("Scale")
        .set(ids.scale, ui)
    {
        state.shader_params.vert_colour_gradient.scale = value;
    }
    for value in slider(state.shader_params.vert_colour_gradient.colour_iter, 0.0, 1.0)
        .down(10.0)
        .label("Colour Iter")
        .set(ids.colour_iter, ui)
    {
        state.shader_params.vert_colour_gradient.colour_iter = value;
    }
    for value in slider(state.shader_params.vert_colour_gradient.line_amp, 0.0, 1.0)
        .down(10.0)
        .label("Line Amp")
        .set(ids.line_amp, ui)
    {
        state.shader_params.vert_colour_gradient.line_amp = value;
    }
    for value in slider(state.shader_params.vert_colour_gradient.diag_amp, 0.0, 1.0)
        .down(10.0)
        .label("Diag Amp")
        .set(ids.diag_amp, ui)
    {
        state.shader_params.vert_colour_gradient.diag_amp = value;
    }
    for value in slider(state.shader_params.vert_colour_gradient.boarder_amp, 0.0, 1.0)
        .down(10.0)
        .label("Boarder Amp")
        .set(ids.boarder_amp, ui)
    {
        state.shader_params.vert_colour_gradient.boarder_amp = value;
    }
}

pub fn set_solid_hsv_colour_widgets(ui: &mut UiCell, ids: &SolidHsvColourIds, state: &mut State) {
    for value in slider(state.shader_params.solid_hsv_colour.hue, 0.0, 1.0)
        .down(10.0)
        .label("Hue")
        .set(ids.hue, ui)
    {
        state.shader_params.solid_hsv_colour.hue = value;
    }
    for value in slider(state.shader_params.solid_hsv_colour.saturation, 0.0, 1.0)
        .down(10.0)
        .label("Saturation")
        .set(ids.saturation, ui)
    {
        state.shader_params.solid_hsv_colour.saturation = value;
    }
    for value in slider(state.shader_params.solid_hsv_colour.value, 0.0, 1.0)
        .down(10.0)
        .label("Value")
        .set(ids.value, ui)
    {
        state.shader_params.solid_hsv_colour.value = value;
    }
}

pub fn set_solid_rgb_colour_widgets(ui: &mut UiCell, ids: &SolidRgbColourIds, state: &mut State) {
    for value in slider(state.shader_params.solid_rgb_colour.red, 0.0, 1.0)
        .down(10.0)
        .label("Red")
        .set(ids.red, ui)
    {
        state.shader_params.solid_rgb_colour.red = value;
    }
    for value in slider(state.shader_params.solid_rgb_colour.green, 0.0, 1.0)
        .down(10.0)
        .label("Green")
        .set(ids.green, ui)
    {
        state.shader_params.solid_rgb_colour.green = value;
    }
    for value in slider(state.shader_params.solid_rgb_colour.blue, 0.0, 1.0)
        .down(10.0)
        .label("Blue")
        .set(ids.blue, ui)
    {
        state.shader_params.solid_rgb_colour.blue = value;
    }
}

fn text(s: &str) -> widget::Text {
    widget::Text::new(s).color(color::WHITE)
}

fn toggle_color(on: bool) -> ui::Color {
    match on {
        true => color::BLUE,
        false => color::BLACK,
    }
}

fn button() -> widget::Button<'static, widget::button::Flat> {
    widget::Button::new()
        .w_h(COLUMN_W, DEFAULT_WIDGET_H)
        .label_font_size(12)
        .color(color::DARK_CHARCOAL)
        .label_color(color::WHITE)
        .border(0.0)
}

// Shorthand for the toggle style we'll use.
fn toggle(b: bool) -> widget::Toggle<'static> {
    widget::Toggle::new(b)
        .w_h(COLUMN_W, DEFAULT_SLIDER_H)
        .label_font_size(14)
        .rgb(0.176, 0.513, 0.639)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

// Shorthand for the slider style we'll use
fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(COLUMN_W, DEFAULT_SLIDER_H)
        .label_font_size(14)
        .rgb(0.176, 0.513, 0.639)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}
