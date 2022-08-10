#version 300 es

precision mediump float;

uniform vec2 view_pos;
uniform float scale;

attribute vec2 a_pos;

void main() {
    gl_Position = vec4(((a_pos * scale) - view_pos), 0.0, 1.0);
}
