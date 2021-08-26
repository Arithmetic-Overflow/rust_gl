#version 140

out vec4 color;
in vec2 pos;

uniform float time;

vec2 complexSquare(vec2 complex) {
	return vec2(
		complex.x * complex.x - complex.y * complex.y,
		2.0 * complex.x * complex.y
	);
} 

/*
Colouring done in this function is arbitrary
I just thought it looked nice with a disco on top of it
*/
vec4 mandleIter(int numIter, vec2 position) {
	vec2 z = vec2(sin(time), 0.0);
	for(int i = 0; i < numIter; i++) {
		z = complexSquare(z) + position;

		if(length(z) > 2.0) {
			float curr = i;
			float maxIter = numIter;

			float intensity = curr/maxIter;

			// minimum hues
			float minHue = 0.2;
			float hueSpeed = 4;
			float scaleFactor = 0.5;

			float mr = sin(hueSpeed*pos.x);
			float mg = scaleFactor*(cos(hueSpeed*pos.x) + sin(hueSpeed*pos.x));
			float mb = scaleFactor*(sin(hueSpeed*pos.y) + cos(hueSpeed*pos.x));

			// actual hues
			float r = max(minHue, mr);
			float g = max(minHue, mg);
			float b = max(minHue, mb);

			float a = 1.0;

			vec4 hue = vec4(r, g, b, a);
			return hue * intensity;
		}
	}

	float mr = 2.0 * sin(pos.y*pos.y);
	float mg = 2.0 * sin(pos.x*pos.x);
	float mb = 2.0 * cos(pos.x*pos.y);
	float a = 1.0;

	float r = min(1.0, mr);
	float g = min(1.0, mg);
	float b = min(1.0, mb);

	float colorfulness = 0.85;

	vec4 hue = colorfulness * vec4(r, g, b, a);
	vec4 pointColor = colorfulness * hue + (1-colorfulness) * vec4(1.0, 1.0, 1.0, 1.0);

	return pointColor;
}

void main() {
	float xoffset = -0.5;
	float yoffset = 0.0;

	float zoom = 1.4;

	vec2 position = vec2(zoom*(pos.x) + xoffset, zoom*(pos.y) + yoffset);

	color = mandleIter(100, position);
}