#version 130

// Texture of the object
uniform sampler2D tex;

// Ambient lighting
uniform vec4 ambient;

uniform mat4 v_inv;

//struct Light {
	uniform vec4 light_pos;
	uniform vec4 light_diffuse;
	uniform vec4 light_specular;
	uniform float light_constant_attenuation, light_linear_attenuation, light_quadratic_attenuation;
	uniform float light_spot_cutoff, light_spot_exponent;
	uniform vec3 light_spot_direction;
//} light;

//struct Material {
	uniform vec4 material_ambient;
	uniform vec4 material_diffuse;
	uniform vec4 material_specular;
	uniform float material_shininess;
//} material;

in vec4 t_pos;
in vec3 t_normal;
in vec2 t_uv;

void main() {
	vec3 normal_dir = normalize(t_normal);
	vec3 view_dir = normalize(vec3(v_inv * vec4(0.0, 0.0, 0.0, 1.0) - t_pos));
	vec3 dir_light;
	float attenuation;
	
	if (light_pos.w == 0.0) { // Directional light?
		attenuation = 1.0; // no attenuation
		dir_light = normalize(vec3(light_pos));
	} else {
		// point light or spotlight (or other kind of light) 
		vec3 pos_to_light = vec3(light_pos - t_pos);
		float distance = length(pos_to_light);
		dir_light = normalize(pos_to_light);
		attenuation = 1.0 / (light_constant_attenuation
			+ light_linear_attenuation * distance
			+ light_quadratic_attenuation * distance * distance);
		
		if (light_spot_cutoff <= radians(90.0)) {
			// spotlight?
			float clamped_cos = max(0.0, dot(-dir_light, light_spot_direction));
			if (clamped_cos < cos(light_spot_cutoff)) {
				// outside of spotlight cone?
				attenuation = 0.0;
			} else {
				attenuation = attenuation * pow(clamped_cos, light_spot_exponent);   
			}
		}
	}
	
	vec4 ambient_lighting = ambient * material_ambient;
	
	vec4 diffuse_reflection = attenuation 
		* light_diffuse * material_diffuse
		* max(0.0, dot(normal_dir, dir_light));
	
	vec4 specular_reflection;
	if (dot(normal_dir, dir_light) < 0.0) {
		// light source on the wrong side?
		specular_reflection = vec4(0.0, 0.0, 0.0, 1.0); // no specular reflection
	} else {
		// light source on the right side
		specular_reflection = attenuation * light_specular * material_specular 
			* pow(max(0.0, dot(reflect(-dir_light, normal_dir), view_dir)), material_shininess);
	}
	
	gl_FragColor = (ambient_lighting + diffuse_reflection + specular_reflection) * texture(tex, t_uv);
}
