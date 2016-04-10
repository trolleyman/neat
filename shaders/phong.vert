#version 330

uniform mat4 mvp;
// normalMatrix = (modelView).transpose().inverse()
uniform mat4 normalMatrix;

in vec3 pos;
in vec3 normal;
in vec2 uv;

out vec3 t_normal;
out vec2 t_uv;

void main() {
	t_uv = uv;
	t_normal = (normalMatrix * vec4(normal, 1.0)).xyz;
	gl_Position = mvp * vec4(pos, 1.0);
}
