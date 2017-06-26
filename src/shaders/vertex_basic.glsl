#version 130

precision highp float;

in vec3 position;
in vec2 tex_coords;
in vec4 debug_color;

out vec2 v_tex_coords;
out vec4 v_debug_color;

uniform float window_width;
uniform float window_height;

void main() {
    float window_width_new = window_width / 2;
    float window_height_new = window_height / 2;
    
    float x_pos = (position[0] / window_width_new) - 1.0;
    float y_pos = ((window_height_new - position[1]) / window_height_new);
    
    v_debug_color = debug_color;

    gl_Position = vec4(x_pos, y_pos, position[2], 1.0);
}