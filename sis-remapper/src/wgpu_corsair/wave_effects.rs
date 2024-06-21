use bytemuck::{NoUninit, Pod, Zeroable};
use cgmath::Angle;
use icue_bindings::types::{CorsairLedColor, CorsairLedLuid};
use sis_core::{rgbau8_to_rgbaf32, ColorAnimation, ColorChangeAnimation, RGBAf32, RippleAnimation, WaveAnimation, RGBA};

use crate::corsair::effects::CorsairLedColorf32;

const LED_DISTANCE: f64 = 20.0;

struct SimdLeds {
    positions: Vec<(f64, f64)>,
    colors: Vec<SimdRGBLeds>,
    ids: Vec<CorsairLedLuid>
}


#[repr(align(64))]
struct SimdRGBLeds {
    l1: [f32;3],
    l2: [f32;3],
    l3: [f32;3],
    l4: [f32;3],
    l5: [f32;3],
    _pad: f32,
}

type LedInfo = ((f64, f64), CorsairLedColor);
pub(crate) type LedInfof32 = ((f64, f64), CorsairLedColorf32);
type Leds<'a> = Box<dyn Iterator<Item=LedInfo> + 'a>;
type Ledsf32<'a> = Box<dyn Iterator<Item=LedInfof32> + 'a>;
type SimdLedsf32<'a> = Box<dyn Iterator<Item=LedInfof32> + 'a>;

pub(crate) fn clorled_to_floatled<'a>(leds: Leds<'a>) -> Ledsf32<'a> {
    Box::new(leds.map(|(pos, CorsairLedColor { id, r, g, b, a })| {
        let color = rgbau8_to_rgbaf32((r, g, b, a));
        (pos, CorsairLedColorf32 {
            id,
            color
        })
    }))
}

pub(crate) fn floatled_to_colorled(leds: &[LedInfof32]) -> Leds {
    Box::new(leds.iter().map(|(pos, CorsairLedColorf32 { id, color })| {
        let (r,g,b,a) = rgbaf32_to_rgbau8(color);
        ((pos.0, pos.1), CorsairLedColor {
            id: id.clone(),
            r,
            g,
            b,
            a
        })
    }))
}

pub(crate) fn static_effect(leds: &mut [LedInfof32], effect_color: RGBAf32) {
    for led in leds {
        static_key(led, effect_color)
    }
}

pub(crate) fn static_key((pos, CorsairLedColorf32 {id, color}): &mut LedInfof32, effect_color: RGBAf32) {
    alpha_compose(color, &effect_color);
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub(crate) struct WaveParams {
    head: f64,
    width: f64
}
impl WaveParams {
    pub(crate) fn bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

pub(crate) fn wave_params(dt_millis: u64, wave: &WaveAnimation) -> WaveParams {
    let wave_head = (dt_millis % wave.duration.as_millis() as u64) as f64 * wave.speed / 1000.0 * LED_DISTANCE;
    let wave_width = wave.light_amount * LED_DISTANCE;

    WaveParams {
        head: wave_head,
        width: wave_width
    }
}

pub(crate) fn wave_effect<'a>(leds: &mut [LedInfof32], dt_millis: u64, wave: &'a WaveAnimation) {
    let params = wave_params(dt_millis, wave);
    for led in leds {
        wave_led(led, dt_millis, wave, &params);
    }
}

pub(crate) fn wave_key(key: &mut LedInfof32, dt_millis: u64, wave: &WaveAnimation) {
    let params = wave_params(dt_millis, wave);
    wave_led(key, dt_millis, wave, &params)
}

fn wave_led((pos, CorsairLedColorf32 {id, color}): &mut LedInfof32, dt_millis: u64, wave: &WaveAnimation, params: &WaveParams) {
    const MIDPOINT: f64 = 100.0;
    let pos_rotated = pos.0 * wave.rotation.cos() as f64- pos.1 * wave.rotation.sin() as f64;
    let distance = if wave.two_sides {
        params.head - (pos_rotated - MIDPOINT).abs()
    } else {
        params.head - pos_rotated
    };
    if distance > 0.0 && distance < params.width {
        // The key is inside the wave
        let sample_point = (distance / params.width) as f32;
        let effect_color = sample_animation(sample_point, &wave.animation);
        alpha_compose(color, &effect_color);
    }
}

#[derive(Debug, Pod, Clone, Copy, Zeroable)]
#[repr(C)]
pub(crate) struct RippleParams {
    head: f64,
    width: f64
}
impl RippleParams {
    pub(crate) fn bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

pub(crate) fn ripple_params(dt_millis: u64, ripple: &RippleAnimation) -> RippleParams {
    let ripple_head = (dt_millis % ripple.duration.as_millis() as u64) as f64 * ripple.speed / 1000.0 * LED_DISTANCE;
    let ripple_width = ripple.light_amount * LED_DISTANCE;

    RippleParams {
        head: ripple_head,
        width: ripple_width,
    }
}

pub(crate) fn ripple_effect<'a>(leds: &mut [LedInfof32], dt_millis: u64, ripple: &'a RippleAnimation) {
    let params = ripple_params(dt_millis, ripple);
    for led in leds {
        ripple_led(led, dt_millis, ripple, &params)
    }
}

fn ripple_key(key: &mut LedInfof32, dt_millis: u64, ripple: &RippleAnimation) {
    let params = ripple_params(dt_millis, ripple);
    ripple_led(key, dt_millis, ripple, &params);
}

fn ripple_led((pos, CorsairLedColorf32 {id, color}): &mut LedInfof32, dt_millis: u64, ripple: &RippleAnimation, params: &RippleParams) {
    const MIDPOINT_X: f64 = 200.0;
    const MIDPOINT_Y: f64 = 100.0;
    let pos = (pos.0 - MIDPOINT_X, pos.1 - MIDPOINT_Y);
    let d = f64::sqrt(f64::powi(pos.0, 2) + f64::powi(pos.1, 2));
    let distance = params.head - d;

    if distance > 0.0 && distance < params.width {
        // The key is inside the ripple
        let sample_point = (distance / params.width) as f32;
        let effect_color = sample_animation(sample_point, &ripple.animation);
        alpha_compose(color, &effect_color)
    }
}

pub(crate) fn colorchange_effect<'a>(leds: &mut [LedInfof32], dt_millis: u64, colorchange: &'a ColorChangeAnimation) {
    for led in leds {
        colorchange_key(led, dt_millis, colorchange)
    }
}

pub(crate) fn colorchange_key((pos, CorsairLedColorf32 {id, color}): &mut LedInfof32, dt_millis: u64, colorchange: &ColorChangeAnimation) {
    let sample_point = dt_millis as f32 / colorchange.duration.as_millis() as f32;
    let effect_color = sample_animation(sample_point, &colorchange.animation);
    alpha_compose(color, &effect_color)
}

fn sample_animation(sample_point: f32, animation: &ColorAnimation) -> RGBAf32 {
    assert!(sample_point <= 1.0);
    assert!(sample_point >= 0.0);
    let mut iter = animation.keyframes.iter();
    let mut previous_color = [0.0,0.0,0.0,0.0];
    let mut next_color = [0.0,0.0,0.0,0.0];
    let mut previous_timestamp = 0.0;
    let mut next_timestamp = 1.0;
    loop {
        if let Some(keyframe) = iter.next() {
            if keyframe.timestamp > sample_point {
                next_timestamp = keyframe.timestamp;
                next_color = keyframe.color;
                break;
            } else {
                previous_timestamp = keyframe.timestamp;
                previous_color = keyframe.color;
                next_color = keyframe.color;
            }
        } else {
            break
        }
    }

    linear_interpolation(previous_color, next_color, (sample_point-previous_timestamp)/(next_timestamp-previous_timestamp))
}

fn linear_interpolation(previous_color: RGBAf32, next_color: RGBAf32, t: f32) -> RGBAf32 {
    [
        (1.0 - t) * previous_color[0] + t * next_color[0],
        (1.0 - t) * previous_color[1] + t * next_color[1],
        (1.0 - t) * previous_color[2] + t * next_color[2],
        (1.0 - t) * previous_color[3] + t * next_color[3],
    ]
}

fn srg_to_oklab(color: RGBAf32) -> RGBAf32 {
    let l = 0.4122214708f32 * color[0] as f32 + 0.5363325363f32 * color[1] as f32 + 0.0514459929f32 * color[2] as f32;
	let m = 0.2119034982f32 * color[0] as f32 + 0.6806995451f32 * color[1] as f32 + 0.1073969566f32 * color[2] as f32;
	let s = 0.0883024619f32 * color[0] as f32 + 0.2817188376f32 * color[1] as f32 + 0.6299787005f32 * color[2] as f32;

    let l_ = f32::cbrt(l);
    let m_ = f32::cbrt(m);
    let s_ = f32::cbrt(s);

    [
        0.2104542553f32*l_ + 0.7936177850f32*m_ - 0.0040720468f32*s_,
        1.9779984951f32*l_ - 2.4285922050f32*m_ + 0.4505937099f32*s_,
        0.0259040371f32*l_ + 0.7827717662f32*m_ - 0.8086757660f32*s_,
        color[3] as f32,
    ]
}

fn oklab_to_srgb(color: RGBAf32) -> RGBAf32 {
    let l_ = color[0] + 0.3963377774f32 * color[1] + 0.2158037573f32 * color[2];
    let m_ = color[0] - 0.1055613458f32 * color[1] - 0.0638541728f32 * color[2];
    let s_ = color[0] - 0.0894841775f32 * color[1] - 1.2914855480f32 * color[2];

    let l = l_*l_*l_;
    let m = m_*m_*m_;
    let s = s_*s_*s_;

    [
		4.0767416621f32 * l - 3.3077115913f32 * m + 0.2309699292f32 * s,
		-1.2684380046f32 * l + 2.6097574011f32 * m - 0.3413193965f32 * s,
		-0.0041960863f32 * l - 0.7034186147f32 * m + 1.7076147010f32 * s,
        color[3],
    ]
}

pub(super) fn rgbaf32_to_rgbau8(color: &RGBAf32) -> RGBA {
    //let color = oklab_to_srgb(color);

    (
        (color[0] * 255.0) as u8,
        (color[1] * 255.0) as u8,
        (color[2] * 255.0) as u8,
        (color[3] * 255.0) as u8,
    )
}

#[repr(align(16))] // aligned to u128
struct AlignedRGBA([f32;4]);

fn alpha_compose(under_color: &mut RGBAf32, over_rgb: &RGBAf32) {
    use std::arch::x86_64::_mm_load1_ps as simd_set_f32;
    use std::arch::x86_64::_mm_loadu_ps as simd_load_f32;
    use std::arch::x86_64::_mm_add_ps as simd_add_f32;
    use std::arch::x86_64::_mm_mul_ps as simd_mul_f32;
    use std::arch::x86_64::_mm_store_ps as simd_recover_f32;

    //let (u_r, u_g, u_b, u_a) = under_color;
    //let (o_r, o_g, o_b, o_a) = over_color;
    let u_a = under_color[3];
    let o_a = over_rgb[3];
    let repr_o_a = 1.0 - o_a;
    let out_a = o_a + u_a * repr_o_a;
    let inv_out_a = 1.0 / out_a;
    let a_1 = o_a * inv_out_a;
    let a_2 = u_a * repr_o_a * inv_out_a;

    let u_rgb = &under_color;
    unsafe {
        //rgb = o_rgb * a_1 + u_rgb * a_2
        let u_rgb = simd_load_f32(&u_rgb[0]);
        let o_rgb = simd_load_f32(&over_rgb[0]);
        let a_1 = simd_set_f32(&a_1);
        let a_2 = simd_set_f32(&a_2);
        let m1 = simd_mul_f32(o_rgb, a_1);
        let m2 = simd_mul_f32(u_rgb, a_2);

        let out_rgb = simd_add_f32(m1, m2);
        simd_recover_f32(&mut under_color[0], out_rgb)
    }
    under_color[3] = out_a;
}
