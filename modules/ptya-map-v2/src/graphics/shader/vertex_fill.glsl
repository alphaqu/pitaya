#version 300 es

precision mediump float;

uniform vec2 pos;
uniform vec2 scale;

out vec4 f_color;

layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec4 a_color;

void main() {
    vec2 pos = ((a_pos + pos) * scale);
    gl_Position = vec4(vec2((pos.x * 2.0) - 1.0, -((pos.y * 2.0) - 1.0)), 0.0, 1.0);
    f_color = a_color;
}
