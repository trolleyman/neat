#version 330

in vec2 transfer_uv;
in vec3 transfer_color;

uniform sampler2D tex;

void main() {
	gl_FragColor = vec4(vec3(1.0, 1.0, 1.0), texture(tex, transfer_uv));
}
