#version 330

in vec2 pos;
in vec2 uv;

out vec2 t_uv;
out vec3 t_color;

uniform vec3 color;

void main() {
	t_uv = uv;
	t_color = color;
	gl_Position = vec4(pos, 0.0, 1.0);
}
