#version 330

uniform mat4 mvp;
// normalMatrix = (modelView).transpose().inverse()
uniform mat4 normal_mat;

in vec3 pos;
in vec3 normal;
in vec2 uv;

out vec3 t_pos;
out vec3 t_normal;
out vec2 t_uv;

void main() {
	t_pos = pos;
	t_uv = uv;
	t_normal = (normal_mat * vec4(normal, 1.0)).xyz;
	gl_Position = mvp * vec4(pos, 1.0);
}
