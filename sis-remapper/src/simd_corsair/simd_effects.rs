use cgmath::Angle;
use icue_bindings::types::{CorsairLedColor, CorsairLedLuid, CorsairLedPosition};
use m512::{f32x16, ConstM512};
use sis_core::{rgbau8_to_rgbaf32, ColorAnimation, ColorChangeAnimation, RGBAf32, RippleAnimation, SimdColorAnimation, WaveAnimation, RGBA};

use crate::corsair::effects::CorsairLedColorf32;

mod m512;

const LED_DISTANCE: f64 = 20.0;

const COLORS_PER_SIMD: usize = 5;
const POSITIONS_PER_SIMD: usize = 8;

#[repr(C, align(64))]
struct SimdRGBLeds {
    l0: [f32;3],
    l1: [f32;3],
    l2: [f32;3],
    l3: [f32;3],
    l4: [f32;3],
    _pad: f32,
}
impl SimdRGBLeds {
    fn as_ptr(&self) -> *const f32 {
        self.l0.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut f32 {
        self.l0.as_mut_ptr()
    }
}

pub(crate) struct SimdRGBALeds {
    color: SimdRGBLeds,
    alpha: SimdRGBLeds, // Each channel has an alpha for SIMD reasons, but they are just duplicated so every channel has the same alpha
}
impl SimdRGBALeds {
    fn copy_color(effect_color: &(f32, f32, f32, f32)) -> SimdRGBALeds {
        let color = [effect_color.0, effect_color.1, effect_color.2];
        let alpha = [effect_color.3;3];
        SimdRGBALeds {
            color: SimdRGBLeds {
                l0: color,
                l1: color,
                l2: color,
                l3: color,
                l4: color,
                _pad: 0.0,
            },
            alpha: SimdRGBLeds {
                l0: alpha,
                l1: alpha,
                l2: alpha,
                l3: alpha,
                l4: alpha,
                _pad: 0.0
            }
        }
    }

    fn get_leds(&self, ids: &[CorsairLedLuid]) -> Vec<CorsairLedColor> {
        let convert_color = |id, rgb: [f32;3], a| {
            let a = (a * 255.0) as u8;
            CorsairLedColor {
                id,
                r: (rgb[0] * 255.0) as u8,
                g: (rgb[1] * 255.0) as u8,
                b: (rgb[2] * 255.0) as u8,
                a,
            }
        };
        ids.into_iter()
            .enumerate()
            .map(|(i, id)| {
                let (rgb, a) = match i {
                    0 => (self.color.l0, self.alpha.l0[0]),
                    1 => (self.color.l1, self.alpha.l1[0]),
                    2 => (self.color.l2, self.alpha.l2[0]),
                    3 => (self.color.l3, self.alpha.l3[0]),
                    4 => (self.color.l4, self.alpha.l4[0]),
                    _ => unreachable!()
                };

                convert_color(*id, rgb, a)
            }).collect()
    }
}

#[repr(C, align(64))]
#[derive(Clone)]
struct SimdPositions([[f32;2];POSITIONS_PER_SIMD]);

pub(crate) struct SimdLeds {
    initial_positions: Vec<SimdPositions>,
    positions: Vec<SimdPositions>, // a (mutable) copy of initial_positions to avoid constant allocations and freeings
    colors: Vec<SimdRGBALeds>,
    ids: Vec<CorsairLedLuid>,
}
impl SimdLeds {
    pub(crate) fn new() -> Self {
        Self {
            initial_positions: Vec::new(),
            colors: Vec::new(),
            ids: Vec::new(),
            positions: Vec::new(),
        }
    }

    pub(crate) fn load(&mut self, leds: Vec<CorsairLedPosition>) {
        const EMPTY_RGBA: SimdRGBALeds = SimdRGBALeds {
            color: SimdRGBLeds {
                l0: [0.0,0.0,0.0],
                l1: [0.0,0.0,0.0],
                l2: [0.0,0.0,0.0],
                l3: [0.0,0.0,0.0],
                l4: [0.0,0.0,0.0],
                _pad: 0.0,
            },
            alpha: SimdRGBLeds {
                l0: [1.0,1.0,1.0],
                l1: [1.0,1.0,1.0],
                l2: [1.0,1.0,1.0],
                l3: [1.0,1.0,1.0],
                l4: [1.0,1.0,1.0],
                _pad: 0.0,
            },
        };
        const EMPTY_POSITION: SimdPositions = SimdPositions([
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
        ]);

        self.initial_positions.clear();
        self.colors.clear();
        self.ids.clear();

        for (led_count, led) in leds.into_iter().enumerate() {
            let CorsairLedPosition {
                id,
                cx,
                cy,
            } = led;

            self.ids.push(id);

            const LAST_LED_POSITION: usize = POSITIONS_PER_SIMD - 2;
            match led_count % POSITIONS_PER_SIMD {
                0 => {
                    let mut position = EMPTY_POSITION;
                    position.0[0][0] = cx as f32;
                    position.0[0][1] = cy as f32;
                    self.initial_positions.push(position);
                    self.positions.push(EMPTY_POSITION);
                },
                i @ 1..=LAST_LED_POSITION => {
                    let position = self.initial_positions.last_mut().unwrap();
                    position.0[i][0] = cx as f32;
                    position.0[i][1] = cy as f32;
                }
                _ => unreachable!()
            }

            const LAST_LED_COLOR: usize = COLORS_PER_SIMD - 2;
            match led_count % COLORS_PER_SIMD {
                0 => {
                    self.colors.push(EMPTY_RGBA);
                },
                1..=LAST_LED_COLOR => (),
                _ => unreachable!()
            }
        }
    }

    pub(crate) fn get_leds(&mut self) -> &mut Vec<SimdRGBALeds> {
        // Reset colors
        for color in self.colors.iter_mut() {
            color.color.l0 = [0.0,0.0,0.0];
            color.color.l1 = [0.0,0.0,0.0];
            color.color.l2 = [0.0,0.0,0.0];
            color.color.l3 = [0.0,0.0,0.0];
            color.color.l4 = [0.0,0.0,0.0];
            color.color._pad = 0.0;

            color.alpha.l0 = [1.0,1.0,1.0];
            color.alpha.l1 = [1.0,1.0,1.0];
            color.alpha.l2 = [1.0,1.0,1.0];
            color.alpha.l3 = [1.0,1.0,1.0];
            color.alpha.l4 = [1.0,1.0,1.0];
            color.alpha._pad = 0.0;
        };

        &mut self.colors
    }

    pub(crate) fn get_positions(&mut self) -> &mut Vec<SimdPositions> {
        // Reset positions
        for (position, original) in self.positions.iter_mut().zip(self.initial_positions.iter()) {
            *position = original.clone();
        };

        &mut self.positions
    }

    pub(crate) fn get_led_colors(&self) -> Vec<CorsairLedColor> {
        self.ids.chunks(COLORS_PER_SIMD)
            .zip(self.colors.iter())
            .flat_map(|(ids, colors)| colors.get_leds(ids))
            .collect()
    }
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

pub(crate) fn static_effect(leds: &mut [SimdRGBALeds], effect_color: &RGBAf32) {
    let effect_color = SimdRGBALeds::copy_color(effect_color);
    let (over_rgb, over_alpha) = (
        f32x16::load_aligned(effect_color.color.as_ptr()),
        f32x16::load_aligned(effect_color.alpha.as_ptr()),
    );
    for led in leds {
        leds_alpha_compose(led, over_rgb, over_alpha)
    }
}

pub(crate) fn static_key((pos, CorsairLedColorf32 {id, color}): &mut LedInfof32, effect_color: RGBAf32) {
    alpha_compose(color, &effect_color);
}

struct WaveParams {
    head: f64,
    width: f64
}

fn wave_params(dt_millis: u64, wave: &WaveAnimation) -> WaveParams {
    let wave_head = (dt_millis % wave.duration.as_millis() as u64) as f64 * wave.speed / 1000.0 * LED_DISTANCE;
    let wave_width = wave.light_amount * LED_DISTANCE;

    WaveParams {
        head: wave_head,
        width: wave_width
    }
}

pub(crate) fn wave_effect<'a>(leds: &mut [SimdRGBALeds], dt_millis: u64, wave: &'a WaveAnimation) {
    let params = wave_params(dt_millis, wave);
    for led in leds {
        todo!()
        //wave_led(led, dt_millis, wave, &params);
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

struct RippleParams {
    head: ConstM512,
    width: ConstM512
}

struct LoadedRippleParams {
    head: f32x16,
    width: f32x16,
}

impl RippleParams {
    fn load(&self) -> LoadedRippleParams {
        let head = f32x16::load_aligned(self.head.as_ptr());
        let width = f32x16::load_aligned(self.width.as_ptr());
        LoadedRippleParams {
            head,
            width
        }
    }
}

fn ripple_params(dt_millis: u64, ripple: &RippleAnimation) -> RippleParams {
    let ripple_head = (dt_millis % ripple.duration.as_millis() as u64) as f32 * (ripple.speed as f32) / 1000.0 * (LED_DISTANCE as f32);
    let ripple_width = (ripple.light_amount as f32) * (LED_DISTANCE as f32);

    RippleParams {
        head: ConstM512::single(ripple_head),
        width: ConstM512::single(ripple_width),
    }
}

pub(crate) fn ripple_effect<'a>(leds: &mut [SimdRGBALeds], positions: &mut [SimdPositions], dt_millis: u64, ripple: &'a RippleAnimation) {
    const MIDPOINT: ConstM512 = ConstM512::repeat_2(200.0, 100.0);

    let midpoint = f32x16::load_aligned(MIDPOINT.as_ptr());

    let params = ripple_params(dt_millis, ripple);
    let loaded_params = params.load();
    let mut distances = Vec::with_capacity(positions.len() * POSITIONS_PER_SIMD); // there are 8 distances calculated per SimdPositions
    let mut sampling_points = Vec::with_capacity(positions.len() * 8); // there are 8 sampling points calculated per SimdPositions
    for position in positions {
        ripple_calculate_sample_points(position, midpoint, &loaded_params);
        for distance_and_sample in position.0 {
            distances.push(distance_and_sample[0]);
            sampling_points.push(distance_and_sample[1]);
        }
    };
    for (i, led) in leds.iter_mut().enumerate() {
        let array_range = (i*COLORS_PER_SIMD)..((i*COLORS_PER_SIMD)+16);
        ripple_leds(
            led,
            &distances[array_range.clone()],
            &sampling_points[array_range],
            &ripple.animation,
            &params
        )
    }
}

fn ripple_calculate_sample_points(position: &mut SimdPositions, midpoint: f32x16, params: &LoadedRippleParams) {
    // pos = (pos.0 - MIDPOINT_X, pos.1 - MIDPOINT_Y);
    let pos = f32x16::load_aligned(position.0[0].as_ptr());
    let pos = pos - midpoint;

    // d = sqrt(pos.0**2 + pos.1**2);
    let pos_squared = pos * pos;
    let alternate_pos_squared = pos_squared.swap2_same();
    let d = (pos_squared + alternate_pos_squared).sqrt();

    let distance = params.head - d;
    // distance now contains all the distances (duplicated), like so:
    // d0 d0 d1 d1
    // d2 d2 d3 d3
    // d4 d4 d5 d5
    // d6 d6 d7 d7

    let distance = distance.swap2_right();
    // distance now contains all the distances (duplicated), like so:
    // d0 d1 d1 d0
    // d2 d3 d3 d2
    // d4 d5 d5 d4
    // d6 d7 d7 d6

    let sample_point = distance / params.width;
    // sample_point now contains all the sample points (duplicated), like so:
    // s0 s1 s1 s0
    // s2 s3 s3 s2
    // s4 s5 s5 s4
    // s6 s7 s7 s6

    let distance_and_samples = distance.unpack_even(sample_point);
    // distance_and_samples now contains all the sample points and distances, like so:
    // d0 s0 d1 s1
    // d2 s2 d3 s3
    // d4 s4 d5 s5
    // d6 s6 d7 s7

    distance_and_samples.recover(position.0[0].as_mut_ptr())
}

fn ripple_leds(led: &mut SimdRGBALeds, distances: &[f32], sampling_points: &[f32], color_animation: &SimdColorAnimation, params: &RippleParams) {
    // TODO: only set color if distance > 0.0 && distance < params.width
    if distance > 0.0 && distance < params.width {
        // The led is inside the ripple
        let effect_color = sample_animation(sampling_points, color_animation);
        let over_alpha = load_f32(effect_color.alpha.as_ptr());
        let over_rgb = load_f32(effect_color.rgb.as_ptr());
        leds_alpha_compose(led, over_rgb, over_alpha);
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

struct SampledAnimation {
    rgb: f32x16,
    alpha: f32x16
}

fn sample_animation(sampling_points: &[f32], animation: &SimdColorAnimation) -> SampledAnimation {
    assert!(sampling_points.len() == 16);
    for sample_point in sampling_points {
        // TODO: enable these asserts only in debug mode
        assert!(*sample_point <= 1.0);
        assert!(*sample_point >= 0.0);
    }
    let sample_point = f32x16::load_unaligned(sampling_points.as_ptr());
    let counter = f32x16::zero();
    let sum = f32x16::load_aligned(ConstM512::single(1.0).as_ptr()); // TODO: can be const
    let mut previous_color = (0.0,0.0,0.0,0.0);
    let mut next_color = (0.0,0.0,0.0,0.0);
    let mut previous_timestamp = 0.0;
    let mut next_timestamp = 1.0;
    let mut iter = animation.keyframes.timestamps.iter();
    for timestamp in iter {
        let timestamp = f32x16::load_aligned(ConstM512::single(*timestamp).as_ptr());
        timestamp.incr_if_ge(sample_point, counter, sum);
        if keyframe.timestamp > sample_point {
            next_timestamp = keyframe.timestamp;
            next_color = keyframe.color;
            break;
        } else {
            previous_timestamp = keyframe.timestamp;
            previous_color = keyframe.color;
            next_color = keyframe.color;
        }
    }
    /* loop {
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
    } */

    let t = (sample_point-previous_timestamp)/(next_timestamp-previous_timestamp);
    simd_linear_interpolation(previous_color, next_color, t)
}

fn simd_linear_interpolation(previous_color: f32x16, next_color: f32x16, t: f32) -> f32x16 {
    let rev_t = 1.0 - t;
    let t = f32x16::load_aligned(ConstM512::single(t).as_ptr());
    let rev_t = f32x16::load_aligned(ConstM512::single(rev_t).as_ptr());

    rev_t * previous_color + t * next_color
}

fn linear_interpolation(previous_color: RGBAf32, next_color: RGBAf32, t: f32) -> RGBAf32 {
    (
        (1.0 - t) * previous_color.0 + t * next_color.0,
        (1.0 - t) * previous_color.1 + t * next_color.1,
        (1.0 - t) * previous_color.2 + t * next_color.2,
        (1.0 - t) * previous_color.3 + t * next_color.3,
    )
}

fn srg_to_oklab(color: RGBAf32) -> RGBAf32 {
    let l = 0.4122214708f32 * color.0 as f32 + 0.5363325363f32 * color.1 as f32 + 0.0514459929f32 * color.2 as f32;
	let m = 0.2119034982f32 * color.0 as f32 + 0.6806995451f32 * color.1 as f32 + 0.1073969566f32 * color.2 as f32;
	let s = 0.0883024619f32 * color.0 as f32 + 0.2817188376f32 * color.1 as f32 + 0.6299787005f32 * color.2 as f32;

    let l_ = f32::cbrt(l);
    let m_ = f32::cbrt(m);
    let s_ = f32::cbrt(s);

    (
        0.2104542553f32*l_ + 0.7936177850f32*m_ - 0.0040720468f32*s_,
        1.9779984951f32*l_ - 2.4285922050f32*m_ + 0.4505937099f32*s_,
        0.0259040371f32*l_ + 0.7827717662f32*m_ - 0.8086757660f32*s_,
        color.3 as f32,
    )
}

fn oklab_to_srgb(color: RGBAf32) -> RGBAf32 {
    let l_ = color.0 + 0.3963377774f32 * color.1 + 0.2158037573f32 * color.2;
    let m_ = color.0 - 0.1055613458f32 * color.1 - 0.0638541728f32 * color.2;
    let s_ = color.0 - 0.0894841775f32 * color.1 - 1.2914855480f32 * color.2;

    let l = l_*l_*l_;
    let m = m_*m_*m_;
    let s = s_*s_*s_;

    (
		4.0767416621f32 * l - 3.3077115913f32 * m + 0.2309699292f32 * s,
		-1.2684380046f32 * l + 2.6097574011f32 * m - 0.3413193965f32 * s,
		-0.0041960863f32 * l - 0.7034186147f32 * m + 1.7076147010f32 * s,
        color.3,
    )
}

pub(super) fn rgbaf32_to_rgbau8(color: &RGBAf32) -> RGBA {
    //let color = oklab_to_srgb(color);

    (
        (color.0 * 255.0) as u8,
        (color.1 * 255.0) as u8,
        (color.2 * 255.0) as u8,
        (color.3 * 255.0) as u8,
    )
}

#[repr(align(16))] // aligned to u128
struct AlignedRGBA([f32;4]);

fn leds_alpha_compose(under_color: &mut SimdRGBALeds, over_rgb: f32x16, over_alpha: f32x16) {
    const ONE: ConstM512 = ConstM512::single(1.0);

    unsafe {
        let one = f32x16::load_aligned(ONE.as_ptr());
        // repr_o_a = 1.0 - over_alpha;
        let repr_o_a = one - over_alpha;

        let u_a = f32x16::load_aligned(under_color.alpha.as_ptr());
        let out_a = over_alpha + u_a * repr_o_a;
        out_a.recover(under_color.alpha.as_mut_ptr());

        // inv_out_a = 1.0 / out_a;
        let inv_out_a = out_a.reciprocal();

        let a_1 =  over_alpha * inv_out_a;
        let a_2 = u_a * repr_o_a * inv_out_a;

        let u_rgb = f32x16::load_aligned(under_color.color.as_ptr());
        let rgb = over_rgb * a_1 + u_rgb * a_2;
        rgb.recover(under_color.color.as_mut_ptr());
    }
}

fn alpha_compose(under_color: &mut RGBAf32, over_rgb: &RGBAf32) {
    use std::arch::x86_64::_mm_load1_ps as simd_set_f32;
    use std::arch::x86_64::_mm_loadu_ps as simd_load_f32;
    use std::arch::x86_64::_mm_add_ps as simd_add_f32;
    use std::arch::x86_64::_mm_mul_ps as simd_mul_f32;
    use std::arch::x86_64::_mm_store_ps as simd_recover_f32;

    //let (u_r, u_g, u_b, u_a) = under_color;
    //let (o_r, o_g, o_b, o_a) = over_color;
    let u_a = under_color.3;
    let o_a = over_rgb.3;
    let repr_o_a = 1.0 - o_a;
    let out_a = o_a + u_a * repr_o_a;
    let inv_out_a = 1.0 / out_a;
    let a_1 = o_a * inv_out_a;
    let a_2 = u_a * repr_o_a * inv_out_a;

    let u_rgb = &under_color;
    unsafe {
        //rgb = o_rgb * a_1 + u_rgb * a_2
        let u_rgb = simd_load_f32(&u_rgb.0);
        let o_rgb = simd_load_f32(&over_rgb.0);
        let a_1 = simd_set_f32(&a_1);
        let a_2 = simd_set_f32(&a_2);
        let m1 = simd_mul_f32(o_rgb, a_1);
        let m2 = simd_mul_f32(u_rgb, a_2);

        let out_rgb = simd_add_f32(m1, m2);
        simd_recover_f32(&mut under_color.0, out_rgb)
    }
    under_color.3 = out_a;
}
