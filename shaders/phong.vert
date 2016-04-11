#version 330

uniform mat4 mvp;
uniform mat4 model_view;
// normalMatrix = (modelView).transpose().inverse()
uniform mat4 normal_mat;

uniform vec3 light_pos;

in vec3 pos;
in vec3 normal;
in vec2 uv;

out vec3 t_light;
out vec3 t_view;
out vec3 t_normal;
out vec2 t_uv;

void main() {
	vec4 vert_pos = mvp * vec4(pos, 1.0);
	t_light = normalize(vec3(mvp * vec4(light_pos, 1.0) - vert_pos));
	t_view = normalize(vec3(-vert_pos));
	t_uv = uv;
	t_normal = normalize(vec3(normal_mat * vec4(normal, 0.0)));
	gl_Position = vert_pos;
}
