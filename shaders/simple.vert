#version 330

in vec3 pos;

out vec3 t_color;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

uniform vec3 color;

void main() {
	t_color = color;
	gl_Position = projection * view * model * vec4(pos, 1.0);
}
