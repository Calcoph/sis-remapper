#[cfg(feature = "testable_privates")]
use std::time::Duration;
#[cfg(feature = "testable_privates")]
use cgmath::Deg;
use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(feature = "testable_privates")]
use sis_core::{ColorAnimation, Keyframe, RippleAnimation, WaveAnimation};
#[cfg(feature = "testable_privates")]
use sis_remapper::wgpu_corsair::test_exposer::{PubCorsairState as CorsairState, PubEffect as Effect, corsair_singlethread_connect};
//use sis_remapper::corsair::test_exposer::{PubCorsairState as CorsairState, PubEffect as Effect, corsair_singlethread_connect};

#[cfg(feature = "testable_privates")]
fn baseline(c: &mut Criterion, corsair_state: &mut CorsairState) {
    corsair_state.remove_all_effects();

    c.bench_function("baseline", |b| b.iter(|| corsair_state.get_led_colors()));
}

#[cfg(feature = "testable_privates")]
fn static_color_useless(c: &mut Criterion, corsair_state: &mut CorsairState) {
    corsair_state.remove_all_effects();
    let effect = Effect::Static([0.0, 0.0, 0.0, 0.0]);
    corsair_state.add_effect(effect);

    c.bench_function("static color useless", |b| b.iter(|| corsair_state.get_led_colors()));
}

#[cfg(feature = "testable_privates")]
fn static_color_opaque(c: &mut Criterion, corsair_state: &mut CorsairState) {
    corsair_state.remove_all_effects();
    let effect = Effect::Static([1.0, 1.0, 1.0, 1.0]);
    corsair_state.add_effect(effect);

    c.bench_function("static color opaque", |b| b.iter(|| corsair_state.get_led_colors()));
}

#[cfg(feature = "testable_privates")]
fn static_color_opaque_2_layers(c: &mut Criterion, corsair_state: &mut CorsairState) {
    corsair_state.remove_all_effects();
    let effect = Effect::Static([1.0, 1.0, 1.0, 1.0]);
    corsair_state.add_effect(effect);

    c.bench_function("static color opaque 2 layers", |b| b.iter(|| corsair_state.get_led_colors()));
}

#[cfg(feature = "testable_privates")]
fn static_color_transparent_2_layers(c: &mut Criterion, corsair_state: &mut CorsairState) {
    corsair_state.remove_all_effects();
    let effect = Effect::Static([0.5, 0.5, 0.5, 0.5]);
    corsair_state.add_effect(effect);
    let effect = Effect::Static([0.8, 0.0, 0.0, 0.5]);
    corsair_state.add_effect(effect);

    c.bench_function("static color transparent 2 layers", |b| b.iter(|| corsair_state.get_led_colors()));
}

#[cfg(feature = "testable_privates")]
fn static_color_transparent_8_layers(c: &mut Criterion, corsair_state: &mut CorsairState) {
    corsair_state.remove_all_effects();
    let effect = Effect::Static([0.5, 0.5, 0.5, 0.5]);
    corsair_state.add_effect(effect);
    let effect = Effect::Static([0.8, 0.0, 0.0, 0.5]);
    corsair_state.add_effect(effect);
    let effect = Effect::Static([0.8, 0.0, 0.0, 0.5]);
    corsair_state.add_effect(effect);
    let effect = Effect::Static([0.8, 0.0, 0.0, 0.5]);
    corsair_state.add_effect(effect);
    let effect = Effect::Static([0.8, 0.0, 0.0, 0.5]);
    corsair_state.add_effect(effect);
    let effect = Effect::Static([0.8, 0.0, 0.0, 0.5]);
    corsair_state.add_effect(effect);
    let effect = Effect::Static([0.8, 0.0, 0.0, 0.5]);
    corsair_state.add_effect(effect);
    let effect = Effect::Static([0.8, 0.0, 0.0, 0.5]);
    corsair_state.add_effect(effect);

    c.bench_function("static color transparent 8 layers", |b| b.iter(|| corsair_state.get_led_colors()));
}


#[cfg(feature = "testable_privates")]
fn get_wave_effect() -> Effect {
    let color_animation = ColorAnimation {
        name: "name".into(),
        keyframes: vec![
            Keyframe {
                timestamp: 0.0,
                color: [0.0,1.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 0.2,
                color: [1.0,0.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 0.4,
                color: [0.0,1.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 0.6,
                color: [1.0,0.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 0.8,
                color: [0.0,1.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 1.0,
                color: [1.0,0.0,0.0,1.0]
            },
        ],
    };
    let animation = WaveAnimation {
        animation: color_animation,
        duration: Duration::from_millis(0_500),
        speed: 5.0,
        rotation: Deg(80.0).into(),
        light_amount: 7.0,
        two_sides: true,
    };
    Effect::Wave(animation)
}

#[cfg(feature = "testable_privates")]
fn wave_effect(c: &mut Criterion, corsair_state: &mut CorsairState) {
    corsair_state.remove_all_effects();
    let effect = get_wave_effect();
    corsair_state.add_effect(effect);

    c.bench_function("wave effect", |b| b.iter(|| corsair_state.get_led_colors()));
}

#[cfg(feature = "testable_privates")]
fn get_ripple_effect() -> Effect {
    let color_animation = ColorAnimation {
        name: "name".into(),
        keyframes: vec![
            Keyframe {
                timestamp: 0.0,
                color: [0.0,1.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 0.2,
                color: [1.0,0.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 0.4,
                color: [0.0,1.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 0.6,
                color: [1.0,0.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 0.8,
                color: [0.0,1.0,0.0,1.0]
            },
            Keyframe {
                timestamp: 1.0,
                color: [1.0,0.0,0.0,1.0]
            },
        ],
    };
    let animation = RippleAnimation {
        animation: color_animation,
        duration: Duration::from_millis(0_500),
        speed: 5.0,
        light_amount: 7.0,
    };
    Effect::Ripple(animation)
}

#[cfg(feature = "testable_privates")]
fn ripple_effect(c: &mut Criterion, corsair_state: &mut CorsairState) {
    corsair_state.remove_all_effects();
    let effect = get_ripple_effect();
    corsair_state.add_effect(effect);

    c.bench_function("ripple effect", |b| b.iter(|| corsair_state.get_led_colors()));
}

#[cfg(feature = "testable_privates")]
fn mix_1(c: &mut Criterion, corsair_state: &mut CorsairState) {
    corsair_state.remove_all_effects();
    let effect = Effect::Static([0.5, 0.5, 0.5, 0.5]);
    corsair_state.add_effect(effect);
    let effect = Effect::Static([0.8, 0.0, 0.0, 0.5]);
    corsair_state.add_effect(effect);
    let effect = get_ripple_effect();
    corsair_state.add_effect(effect);
    let effect = get_wave_effect();
    corsair_state.add_effect(effect);

    c.bench_function("mix 1", |b| b.iter(|| corsair_state.get_led_colors()));
}

#[cfg(feature = "testable_privates")]
fn run_benchmarks(c: &mut Criterion) {
    let mut corsair_state = corsair_singlethread_connect();
    baseline(c, &mut corsair_state);
    static_color_useless(c, &mut corsair_state);
    static_color_opaque(c, &mut corsair_state);
    static_color_opaque_2_layers(c, &mut corsair_state);
    static_color_transparent_2_layers(c, &mut corsair_state);
    static_color_transparent_8_layers(c, &mut corsair_state);
    wave_effect(c, &mut corsair_state);
    ripple_effect(c, &mut corsair_state);
    mix_1(c, &mut corsair_state);
}

#[allow(unused_variables)]
fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "testable_privates")]
    run_benchmarks(c);

    #[cfg(not(feature = "testable_privates"))]
    {
        eprintln!();
        eprintln!("ERROR ERROR ERROR ERROR ERROR ERROR ERROR ");
        eprintln!("######################");
        eprintln!("You must run this test using \"testable_privates\" feture.");
        eprintln!("\n");
        eprintln!("Run with \"cargo bench --features testable_privates\"");
        eprintln!("######################");
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
