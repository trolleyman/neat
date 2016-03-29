#version 330

in vec3 pos;

out vec3 color;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

uniform vec3 in_color;

void main() {
	color = in_color;
	gl_Position = vec4(pos, 1.0) * projection * view * model;
}
