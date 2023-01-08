#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

@group(1) @binding(0)
var<uniform> foreground_color: vec4<f32>;
@group(1) @binding(1)
var<uniform> background_color: vec4<f32>;

@group(1) @binding(2)
var texture: texture_2d<f32>;
@group(1) @binding(3)
var texture_sampler: sampler;

@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

struct FragmentInput {
    #import bevy_sprite::mesh2d_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var font_px_color: vec4<f32> = textureSample(texture, texture_sampler, in.uv);
    var font_px_present: bool = (font_px_color[3] == 1.0);

    var output_color: vec4<f32> = font_px_color;
    if (font_px_present) {
        output_color = output_color * foreground_color;
    } else {
        output_color = background_color;
    }

    return output_color;
}
