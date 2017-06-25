#version 130

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D texture;

void main() {

    // color = vec4(1.0, (mod(gl_FragCoord.y, 256) / 256), 1.0, 1.0);
    color = vec4(1.0, 0.0, 0.0, 1.0);
}