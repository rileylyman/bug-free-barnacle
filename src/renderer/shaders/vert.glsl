#version 330 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

out vec4 ourColor;

uniform mat4 model;

void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    ourColor = model * vec4(color, 1.0);
}
