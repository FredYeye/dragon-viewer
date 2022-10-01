#version 450 core

//-----

layout (location = 0) in vec2 vertices;
layout (location = 1) in vec2 uv_in;
layout (location = 2) in vec4 color_in;

layout (location = 3) uniform vec2 size = vec2(720.0, 720.0);

out vec2 uv;
out vec4 color;

// https://github.com/emilk/egui/blob/4ac1e28eaefd1c389034e1bc065d83801882e8c7/crates/egui_glium/src/shader/vertex_300es.glsl
// out v_rgba_gamma;
// out a_tc;

//-----

vec3 linear_from_srgb(vec3 srgb) // 0-1 linear from 0-255 sRGB
{
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, cutoff);
}

vec4 linear_from_srgba(vec4 srgba)
{
    return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
}

void main()
{
    gl_Position = vec4(
        2.0 * vertices.x / size.x - 1.0,
        1.0 - 2.0 * vertices.y / size.y,
        0.0,
        1.0
    );

    // v_rgba_gamma = a_srgba / 255.0;
    // v_tc = a_tc;

    uv = uv_in;
    color = linear_from_srgba(color_in);
}
