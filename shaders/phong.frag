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

in vec3 t_light;
in vec3 t_view;
in vec3 t_normal;
in vec2 t_uv;

out vec4 color;

void main() {
	float len2 = dot(t_light, t_light);
	vec3 norm_light = t_light / sqrt(len2);
	float ldotn = max(dot(norm_light, t_normal), 0.0);
	// Direction that a perfect light ray would travel in when reflecting off this point
	vec3 reflected = 2.0 * ldotn * t_normal - norm_light;
	// Calculate intensity at this point
	float lightIntensity = 1.0 / len2;
	vec4 ambient = kA * iA;
	vec4 diffuse = vec4(0.0);
	vec4 specular = vec4(0.0);
	
	if (ldotn > 0.0) {
		diffuse = kD * ldotn * iD * lightIntensity;
		//specular = kS * pow(max(dot(reflected, t_view), 0.0), shininess) * iS;
	}
	vec4 intensity = ambient + diffuse + specular;
	color = texture(tex, t_uv) * intensity;
}
