#version 330

in vec3 t_color;

out vec4 color;

void main() {
	color = vec4(t_color, 1.0);
}
