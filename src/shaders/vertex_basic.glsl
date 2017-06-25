#version 130

precision highp float;

in vec3 position;
uniform float window_width;
uniform float window_height;

void main() {
    float x_pos = (position[0] / window_width) - 1.0;
    float y_pos = ((window_height - position[1]) / window_height);
    gl_Position = vec4(x_pos, y_pos, position[2], 1.0);
}