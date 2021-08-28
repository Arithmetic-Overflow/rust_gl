#version 140

in vec2 v_tex_coords;
in vec2 pos;
out vec4 color;

uniform sampler2D tex;
uniform float time;


// got the helper functions off the internet
#define NUM_OCTAVES 25

float rand(float n){return fract(sin(n) * 43758.5453123);}

float rand(vec2 n) { 
	return fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453);
}

float noise(float p){
	float fl = floor(p);
  float fc = fract(p);
	return mix(rand(fl), rand(fl + 1.0), fc);
}

float noise(vec2 n) {
	const vec2 d = vec2(0.0, 1.0);
  vec2 b = floor(n), f = smoothstep(vec2(0.0), vec2(1.0), fract(n));
	return mix(mix(rand(b), rand(b + d.yx), f.x), mix(rand(b + d.xy), rand(b + d.yy), f.x), f.y);
}

float fbm(vec2 x) {
	float v = 0.0;
	float a = 0.5;
	vec2 shift = vec2(100);
	// Rotate to reduce axial bias
	mat2 rot = mat2(cos(0.5), sin(0.5), -sin(0.5), cos(0.50));
	for (int i = 0; i < NUM_OCTAVES; ++i) {
		v += a * noise(x);
		x = rot * x * 2.0 + shift;
		a *= 0.5;
	}
	return v;
}


void main() {
	float timescale = time * 0.8;
	float freqscale = 0.4;
	float amplitude = 0.015;

	float n = fbm(freqscale*gl_FragCoord.xy);

	vec2 distortion = vec2(amplitude * n);

	vec2 tex_coords = v_tex_coords + distortion;

	vec4 texcolor = texture(tex, tex_coords);

	float saturation = 5.0;
	vec4 saturatedColor = floor(texcolor*saturation)/saturation;
	color = saturatedColor;
}