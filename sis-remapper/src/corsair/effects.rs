use cgmath::Angle;
use icue_bindings::sys::CorsairLedColor;
use sis_core::{rgbau8_to_rgbaf32, ColorAnimation, ColorChangeAnimation, RGBAf32, RippleAnimation, WaveAnimation, RGBA};

const LED_DISTANCE: f64 = 20.0;

pub(crate) struct CorsairLedColorf32 {
    pub id: u32,
    color: RGBAf32
}

type LedInfo = ((f64, f64), CorsairLedColor);
pub(crate) type LedInfof32 = ((f64, f64), CorsairLedColorf32);
type Leds<'a> = Box<dyn Iterator<Item=LedInfo> + 'a>;
type Ledsf32<'a> = Box<dyn Iterator<Item=LedInfof32> + 'a>;

#[derive(Debug, Clone)]
pub(crate) enum Effect {
    Static(RGBAf32),
    Wave(WaveAnimation),
    Ripple(RippleAnimation),
    ColorChange,
}

pub(crate) fn clorled_to_floatled<'a>(leds: Leds<'a>) -> Ledsf32<'a> {
    Box::new(leds.map(|(pos, CorsairLedColor { id, r, g, b, a })| {
        let color = rgbau8_to_rgbaf32((r, g, b, a));
        (pos, CorsairLedColorf32 {
            id,
            color
        })
    }))
}

pub(crate) fn floatled_to_colorled(leds: Ledsf32) -> Leds {
    Box::new(leds.map(|(pos, CorsairLedColorf32 { id, color })| {
        let (r,g,b,a) = rgbaf32_to_rgbau8(color);
        (pos, CorsairLedColor {
            id,
            r,
            g,
            b,
            a
        })
    }))
}

pub(crate) fn static_effect(leds: Ledsf32, effect_color: RGBAf32) -> Ledsf32 {
    Box::new(leds.map(move |key| {
        static_key(key, effect_color)
    }))
}

pub(crate) fn static_key((pos, CorsairLedColorf32 {id, color}): LedInfof32, effect_color: RGBAf32) -> LedInfof32 {
    (pos, CorsairLedColorf32 {
        id,
        color: alpha_compose(color, effect_color)
    })
}

pub(crate) fn wave_effect<'a>(leds: Ledsf32<'a>, dt_millis: u64, wave: &'a WaveAnimation) -> Ledsf32<'a> {
    Box::new(leds.map(move |key| {
        wave_key(key, dt_millis, wave)
    }))
}

pub(crate) fn wave_key((pos, CorsairLedColorf32 {id, color}): LedInfof32, dt_millis: u64, wave: &WaveAnimation) -> LedInfof32 {
    const MIDPOINT: f64 = 100.0;
    let wave_head = (dt_millis % wave.duration.as_millis() as u64) as f64 * wave.speed / 1000.0 * LED_DISTANCE;
    let wave_width = wave.light_amount * LED_DISTANCE;
    let pos_rotated = pos.0 * wave.rotation.cos() as f64- pos.1 * wave.rotation.sin() as f64;
    let distance = if wave.two_sides {
        wave_head - (pos_rotated - MIDPOINT).abs()
    } else {
        wave_head - pos_rotated
    };
    if distance > 0.0 && distance < wave_width {
        // The key is inside the wave
        let sample_point = (distance / wave_width) as f32;
        let effect_color = sample_animation(sample_point, &wave.animation);
        (pos, CorsairLedColorf32 {
            id,
            color: alpha_compose(color, effect_color)
        })
    } else {
        // Don't do nothing
        (pos, CorsairLedColorf32 {
            id,
            color
        })
    }
}


pub(crate) fn ripple_effect<'a>(leds: Ledsf32<'a>, dt_millis: u64, ripple: &'a RippleAnimation) -> Ledsf32<'a> {
    Box::new(leds.map(move |key| {
        ripple_key(key, dt_millis, ripple)
    }))
}

pub(crate) fn ripple_key((pos, CorsairLedColorf32 {id, color}): LedInfof32, dt_millis: u64, ripple: &RippleAnimation) -> LedInfof32 {
    const MIDPOINT_X: f64 = 200.0;
    const MIDPOINT_Y: f64 = 100.0;
    let ripple_head = (dt_millis % ripple.duration.as_millis() as u64) as f64 * ripple.speed / 1000.0 * LED_DISTANCE;
    let ripple_width = ripple.light_amount * LED_DISTANCE;
    let pos = (pos.0 - MIDPOINT_X, pos.1 - MIDPOINT_Y);
    let d = f64::sqrt(f64::powi(pos.0, 2) + f64::powi(pos.1, 2));
    let distance = ripple_head - d;

    if distance > 0.0 && distance < ripple_width {
        // The key is inside the ripple
        let sample_point = (distance / ripple_width) as f32;
        let effect_color = sample_animation(sample_point, &ripple.animation);
        (pos, CorsairLedColorf32 {
            id,
            color: alpha_compose(color, effect_color)
        })
    } else {
        // Don't do nothing
        (pos, CorsairLedColorf32 {
            id,
            color
        })
    }
}

pub(crate) fn colorchange_effect<'a>(leds: Ledsf32<'a>, dt_millis: u64, colorchange: &'a ColorChangeAnimation) -> Ledsf32<'a> {
    Box::new(leds.map(move |key| {
        colorchange_key(key, dt_millis, colorchange)
    }))
}

pub(crate) fn colorchange_key((pos, CorsairLedColorf32 {id, color}): LedInfof32, dt_millis: u64, colorchange: &ColorChangeAnimation) -> LedInfof32 {
    let sample_point = dt_millis as f32 / colorchange.duration.as_millis() as f32;
    let effect_color = sample_animation(sample_point, &colorchange.animation);
    (pos, CorsairLedColorf32 {
        id,
        color: alpha_compose(color, effect_color)
    })
}

fn sample_animation(sample_point: f32, animation: &ColorAnimation) -> RGBAf32 {
    assert!(sample_point <= 1.0);
    assert!(sample_point >= 0.0);
    let mut iter = animation.keyframes.iter();
    let mut previous_color = (0.0,0.0,0.0,0.0);
    let mut next_color = (0.0,0.0,0.0,0.0);
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

pub(super) fn rgbaf32_to_rgbau8(color: RGBAf32) -> RGBA {
    //let color = oklab_to_srgb(color);

    (
        (color.0 * 255.0) as u8,
        (color.1 * 255.0) as u8,
        (color.2 * 255.0) as u8,
        (color.3 * 255.0) as u8,
    )
}

fn alpha_compose(under_color: RGBAf32, over_color: RGBAf32) -> RGBAf32 {
    let (u_r, u_g, u_b, u_a) = under_color;
    let (o_r, o_g, o_b, o_a) = over_color;
    let out_a = o_a + u_a * (1.0 - o_a);
    let out_r = (o_r * o_a + u_r * u_a * (1.0 - o_a)) / out_a;
    let out_g = (o_g * o_a + u_g * u_a * (1.0 - o_a)) / out_a;
    let out_b = (o_b * o_a + u_b * u_a * (1.0 - o_a)) / out_a;

    (out_r,out_g,out_b,out_a)
}
