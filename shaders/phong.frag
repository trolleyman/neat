#version 330

// Texture of the object
uniform sampler2D tex;

uniform vec3 iA; // Ambient intensity
uniform vec3 iS; // Specular intensity
uniform vec3 iD; // Diffuse intensity

uniform float kA; // Ambient reflection constant
uniform float kS; // Specular reflection constant
uniform float kD; // Diffuse reflection constant
uniform float shininess;

uniform vec3 light_pos;

in vec3 t_normal;
in vec2 t_uv;

void main() {
	gl_FragColor = kA * iA;
}
