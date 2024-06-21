@group(0)
@binding(0)
var<storage, read> in_colors: array<vec4<f32>>;

@group(0)
@binding(1)
var<storage, read_write> out_colors: array<vec4<f32>>;

struct PushConstants {
    effect_color: vec4<f32>
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

fn static_effect_impl(color: vec4<f32>) -> vec4<f32>{
    return alpha_compose(color, push_constants.effect_color);
}

@compute
@workgroup_size(1)
fn static_effect(@builtin(global_invocation_id) global_id: vec3<u32>) {
    out_colors[global_id.x] = static_effect_impl(in_colors[global_id.x]);
}
