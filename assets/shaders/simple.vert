#version 130

in vec3 pos;

out vec3 t_color;

uniform mat4 mvp;

uniform vec3 color;

void main() {
	t_color = color;
	gl_Position = mvp * vec4(pos, 1.0);
}
