#version 330

// Texture of the object
uniform sampler2D tex;

uniform vec4 iA; // Ambient intensity
uniform vec4 iS; // Specular intensity
uniform vec4 iD; // Diffuse intensity

uniform vec4 kA; // Ambient reflection constant
uniform vec4 kS; // Specular reflection constant
uniform vec4 kD; // Diffuse reflection constant
uniform float shininess;

uniform vec3 light_pos;

in vec3 t_normal;
in vec2 t_uv;

out vec4 color;

void main() {
	color = texture(tex, t_uv) * kA * iA;
}
