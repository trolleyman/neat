#version 330

// Texture of the object
uniform sampler2D tex;

// Ambient lighting
uniform vec4 ambient;

uniform mat4 v_inv;

struct Light {
	vec4 pos;
	vec4 diffuse;
	vec4 specular;
	float constant_attenuation, linear_attenuation, quadratic_attenuation;
	float spot_cutoff, spot_exponent;
	vec3 spot_direction;
};
uniform Light light;

struct Material {
	vec4 ambient;
	vec4 diffuse;
	vec4 specular;
	float shininess;
};
uniform Material material;

in vec4 t_pos;
in vec3 t_normal;
in vec2 t_uv;

void main() {
	vec3 normal_dir = normalize(t_normal);
	vec3 view_dir = normalize(vec3(v_inv * vec4(0.0, 0.0, 0.0, 1.0) - position));
	vec3 light_dir;
	float attenuation;
	
	if (light.position.w == 0.0) { // Directional light?
		attenuation = 1.0; // no attenuation
		light_dir = normalize(vec3(light.position));
	} else {
		// point light or spotlight (or other kind of light) 
		vec3 pos_to_light = light.position.xyz - t_pos;
		float distance = length(pos_to_light);
		light_dir = normalize(pos_to_light);
		attenuation = 1.0 / (light.constant_attenuation
			+ light.linear_attenuation * distance
			+ light.quadratic_attenuation * distance * distance);
		
		if (light.spot_cutoff <= radians(90.0)) {
			// spotlight?
			float clamped_cos = max(0.0, dot(-light_dir, light.spot_direction));
			if (clamped_cos < cos(light.spot_cutoff)) {
				// outside of spotlight cone?
				attenuation = 0.0;
			} else {
				attenuation = attenuation * pow(clamped_cos, light.spot_exponent);   
			}
		}
	}
	
	vec3 ambientLighting = vec3(scene_ambient) * vec3(material.ambient);
	
	vec3 diffuseReflection = attenuation 
		* vec3(light.diffuse) * vec3(material.diffuse)
		* max(0.0, dot(normal_dir, light_dir));
	
	vec3 specular_reflection;
	if (dot(normal_dir, light_dir) < 0.0) {
		// light source on the wrong side?
		specular_reflection = vec3(0.0, 0.0, 0.0); // no specular reflection
	} else {
		// light source on the right side
		specular_reflection = attenuation * vec3(light.specular) * vec3(material.specular) 
			* pow(max(0.0, dot(reflect(-light_dir, normal_dir), view_dir)), material.shininess);
	}
	
	gl_FragColor = vec4(ambientLighting + diffuseReflection + specular_reflection, 1.0);
}
