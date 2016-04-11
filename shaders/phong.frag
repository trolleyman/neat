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

uniform vec3 camera_pos;
uniform vec3 light_pos;

in vec3 t_pos;
in vec3 t_normal;
in vec2 t_uv;

out vec4 color;

void main() {
	// Direction from point to light
	vec3 l = normalize(light_pos - t_pos);
	// Direction from point to camera
	vec3 v = normalize(camera_pos - t_pos);
	// l . t_normal
	float ln = dot(l, t_normal);
	// Direction that a perfect light ray would travel in when reflecting off this point
	vec3 r = 2.0 * ln * t_normal - l;
	// Calculate intermediate product
	float s = pow(dot(r, v), shininess);
	// Calculate intensity at this point
	vec4 i = kA * iA + kD * ln * iD + kS * vec4(s, s, s, 1.0) * iS;
	color = texture(tex, t_uv) * i;
}
