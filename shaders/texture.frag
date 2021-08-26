#version 140

in vec2 v_tex_coords;
in vec2 pos;
out vec4 color;

uniform sampler2D tex;
uniform float time;


void main() {
    float periodScale = 0.8;
    vec2 distortion = 0.8 * vec2(
        sin(periodScale*time*mod(pos.x + 1.0, 0.4)),
        sin(periodScale*time*mod(pos.y + 1.0, 0.4))
    );

    // distortion = abs(distortion);

    vec2 tex_coords = v_tex_coords + 0.01 * distortion;
    vec4 texcolor = texture(tex, tex_coords);
    float saturation = 5.0;
    vec4 saturatedColor = floor(texcolor*saturation)/saturation;
    color = saturatedColor;
}