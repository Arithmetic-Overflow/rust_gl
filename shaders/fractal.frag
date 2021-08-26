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

vec2 complexPow(vec2 complex, float p) {
    float arg = atan(complex.y, complex.x);
    float len = length(complex);
    float real = pow(len, p) * cos(p*arg);
    float im = pow(len, p) * sin(p*arg);
    return vec2(real, im);
}

/*
Colouring done in this function is arbitrary
I just thought it looked nice with a disco on top of it
*/
vec4 mandleIter(int numIter, vec2 position) {
	vec2 z = vec2(0.0, 0.0);
	float exponent = mod(time, 5.0) + 1.0;
	for(int i = 0; i < numIter; i++) {
		z = complexPow(z, exponent) + position;

		if(length(z) > 2.0) {
			float curr = i;
			float maxIter = numIter;

			float intensity = curr/maxIter;

			return intensity * vec4(1.0, 0.5, 0.0, 1.0);
		}
	}

	return vec4(0.0, 0.0, 0.0, 0.0);
}

void main() {
	float xoffset = 0.0;
	float yoffset = 0.0;

	float zoom = 1.8;

	vec2 position = vec2(zoom*(pos.x) + xoffset, zoom*(pos.y) + yoffset);

	color = mandleIter(300, position);
}