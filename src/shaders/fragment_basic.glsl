#version 130

in vec2 v_tex_coords;
in vec4 v_debug_color;
out vec4 color;

uniform sampler2D texture;

void main() {

    // color = vec4(1.0, (mod(gl_FragCoord.y, 256) / 256), 1.0, 1.0);
    color = v_debug_color;
}