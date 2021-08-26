#version 140

in vec2 v_tex_coords;
in vec2 pos;
out vec4 color;

uniform sampler2D tex;
uniform float time;

uniform sampler2D noise;

void main() {
    float distortion_amount = 2.0 * (texture(noise, v_tex_coords).x - 0.5);
    vec2 distortion = 0.1*vec2(distortion_amount, distortion_amount);

    vec2 tex_coords = v_tex_coords + distortion;

    vec4 texcolor = texture(tex, tex_coords);

    float saturation = 5.0;
    vec4 saturatedColor = floor(texcolor*saturation)/saturation;
    color = saturatedColor;
}