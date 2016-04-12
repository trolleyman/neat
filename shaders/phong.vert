#version 330

uniform mat4 mvp;
uniform mat4 model;
// normalMatrix = (modelView).inverse().transpose()
uniform mat4 normal_mat;

uniform vec3 light_pos;
uniform vec3 camera_pos;

in vec3 pos;
in vec3 normal;
in vec2 uv;

out vec3 t_light;
out vec3 t_view;
out vec3 t_normal;
out vec2 t_uv;

void main() {
	vec4 m_pos4 = model * vec4(pos, 1.0);
	vec3 m_pos = vec3(m_pos4) / m_pos4.w;
	t_light = normalize(vec3(light_pos - m_pos));
	t_view = normalize(vec3(camera_pos - m_pos));
	t_uv = uv;
	t_normal = vec3(model * vec4(normal, 0.0));
	gl_Position = mvp * vec4(pos, 1.0);
}
