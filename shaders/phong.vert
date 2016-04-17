#version 330

uniform mat4 mvp;
uniform mat4 model;
// m_3x3_inv_transp
uniform mat3 normal_mat;

in vec3 pos;
in vec2 uv;
in vec3 normal;

out vec4 t_pos;    // position of the vertex (and fragment) in world space
out vec3 t_normal; // surface normal vector in world space
out vec2 t_uv;

void main() {
	t_pos = model * vec4(pos, 1.0);
	t_normal = normalize(normal_mat * normal);
	t_uv = uv;
	gl_Position = mvp * vec4(pos, 1.0);
}
