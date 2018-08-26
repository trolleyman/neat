#version 330

in vec2 t_uv;
in vec3 t_color;

out vec4 color;

uniform sampler2D tex;

void main() {
	float v = texture(tex, t_uv).r;
	color = vec4(t_color, v);
}
