@group(0)
@binding(0)
var<storage, read_write> colors: array<vec4<f32>>;

@group(0)
@binding(1)
var<storage, read> positions: array<vec2<f32>>;

@group(0)
@binding(2)
var<storage, read> animation: array<Animation>;

struct Ripple {
    head: f32,
    width: f32,
}

struct PushConstants {
    ripple: Ripple,
    midpoint: vec2<f32>,
}

struct Animation {
    timestamp: f32,
    color: vec4<f32>,
}

var<push_constant> push_constants: PushConstants;


fn alpha_compose(under_color: vec4<f32>, over_color: vec4<f32>) -> vec4<f32> {
    let repr_o_a = 1.0 - over_color.w;
    let out_a = over_color.w + under_color.w * repr_o_a;
    let inv_out_a = 1.0 / out_a;

    let out = vec3(
        over_color.xyz * over_color.w * inv_out_a
        + under_color.xyz * under_color.w * repr_o_a * inv_out_a
    );

    return vec4(out.xyz, out_a);
}

fn linear_interpolation(previous_color: vec4<f32>, next_color: vec4<f32>, t: f32) -> vec4<f32> {
    return vec4(
        (1.0 - t) * previous_color.x + t * next_color.x,
        (1.0 - t) * previous_color.y + t * next_color.y,
        (1.0 - t) * previous_color.z + t * next_color.z,
        (1.0 - t) * previous_color.w + t * next_color.w,
    );
}

fn sample_animation(sample_point: f32) -> vec4<f32> {
    var index: u32 = 999999;
    let len = arrayLength(&animation);
    for (var i: u32 = 0; i < len; i++) {
        let timestamp = animation[i].timestamp;
        if timestamp > sample_point && index == 999999 {
            index = i;
        }
    }

    var previous_timestamp = 0.0;
    var previous_color = vec4(0.0,0.0,0.0,0.0);
    var next_color = animation[0].color;
    var next_timestamp = animation[0].timestamp;
    if index == 999999 {
        previous_color = animation[len].color;
        next_color = previous_color;
        previous_timestamp = animation[len].timestamp;
        next_timestamp = 1.0;
    } else if index == 0 {
    } else {
        previous_color = animation[index-1].color;
        next_color = animation[index].color;
        previous_timestamp = animation[index-1].timestamp;
        next_timestamp = animation[index].timestamp;
    }

    let t = (sample_point - previous_timestamp) / (next_timestamp - previous_timestamp);
    return linear_interpolation(previous_color, next_color, t);
}

fn ripple_effect_impl(color: vec4<f32>, position: vec2<f32>) -> vec4<f32>{
    let pos = vec2(
        position.x - push_constants.midpoint.x,
        position.y - push_constants.midpoint.y
    );
    let d = sqrt(pos.x * pos.x + pos.y * pos.y);
    let distance = push_constants.ripple.head - d;

    if distance > 0.0 && distance < push_constants.ripple.width {
        let sample_point = distance / push_constants.ripple.width;
        let effect_color = sample_animation(sample_point);
        return alpha_compose(color, effect_color);
    }
    return color;
}

@compute
@workgroup_size(1)
fn ripple_effect(@builtin(global_invocation_id) global_id: vec3<u32>) {
    colors[global_id.x] = ripple_effect_impl(colors[global_id.x], positions[global_id.x]);
}
