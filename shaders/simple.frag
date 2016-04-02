#version 330

in vec3 transfer_color;

void main() {
	gl_FragColor = vec4(transfer_color, 1.0);
}
