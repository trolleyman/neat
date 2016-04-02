#version 330

in vec3 t_color;

void main() {
	gl_FragColor = vec4(t_color, 1.0);
}
