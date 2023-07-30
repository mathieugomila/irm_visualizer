#version 330
in vec2 position;
out vec2 position_pass;

void main() {
    gl_Position = vec4(position, 0.01, 1.0);
    position_pass = position;
}
