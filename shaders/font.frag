#version 330

in vec2 transfer_uv;
in vec3 transfer_color;

uniform sampler2D tex;

void main() {
	gl_FragColor = vec4(transfer_color, texture(tex, transfer_uv));
}
