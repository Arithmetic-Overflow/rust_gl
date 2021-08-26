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

float mandleIter(int numIter, vec2 position) {
    vec2 z = vec2(0.0, sin(time));
    for(int i = 0; i < numIter; i++) {
        z = complexSquare(z) + position;

        if(length(z) > 2.0) {
            float curr = i;
            float maxIter = numIter;

            return curr/maxIter;
        }
    }

    return 1.0;
}

void main() {
    // float xoffset = -0.5 - min(1.2, time);
    // float yoffset = 0.0;

    float xoffset = -0.5;
    float yoffset = 0.0;


    // float zoom = 1.4 * max(1-time, 0.00001);
    float zoom = 1.4;

    vec2 position = vec2(zoom*(pos.x) + xoffset, zoom*(pos.y) + yoffset);

    float intensity = mandleIter(100, position);
    color = vec4(intensity, intensity, intensity, intensity);
}