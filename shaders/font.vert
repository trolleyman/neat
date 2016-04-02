#version 330

in vec2 pos;
in vec2 uv;

out vec2 transfer_uv;
out vec3 transfer_color;

uniform vec3 color;

void main() {
	transfer_uv = uv;
	gl_Position = vec4(pos, 0.0, 1.0);
}
