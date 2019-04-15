#version 330 core
layout (location = 0) pos;

void main() {
    gl_Position = (pos.x, pos.y, pos.z 1.0);
}
