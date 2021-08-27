#version 140

in vec2 v_tex_coords;
in vec2 pos;
out vec4 color;

uniform sampler2D tex;
uniform float time;

uniform sampler2D noise;

in float n;

void main() {
    color = vec4(n);
    // float timescale = time * 0.8;
    // float rt2 = 3.14;

    // float distortion_amount = 0.08 * texture(noise, v_tex_coords).x;
    // // float distortion_x = 0.1*mod(timescale, rt2) + distortion_amount;
    // float distortion_y = 0.4*cos
    // (mod(timescale, rt2)*mod(timescale, rt2)) + distortion_amount;

    // float distortion_x = 0.1*sin(mod(timescale, rt2)) + distortion_amount;
    // // float distortion_y = 0.4*sin(timescale)*sin(timescale) + distortion_amount;

    // vec2 distortion = vec2(distortion_x, distortion_y);

    // vec2 tex_coords = v_tex_coords + distortion;

    // vec4 texcolor = texture(tex, tex_coords);

    // float saturation = 5.0;
    // vec4 saturatedColor = floor(texcolor*saturation)/saturation;
    // color = saturatedColor;
}