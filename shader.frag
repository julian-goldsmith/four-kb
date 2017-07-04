#version 150

in vec2 Texcoord;
in vec3 Position_worldspace;
in vec3 EyeDirection_cameraspace;
in vec3 LightDirection_cameraspace;
in vec3 Normal_cameraspace;
in float dist;

out vec4 out_color;

uniform sampler2D tex;

void main() {
	float cosTheta = clamp(dot(Normal_cameraspace, LightDirection_cameraspace), 0, 1);
	vec4 light_color = vec4(0.6, 0.6, 0.6, 1.0);
	vec4 ambient_color = vec4(0.1, 0.1, 0.1, 0.1);
	
	vec3 reverse_normal = reflect(-LightDirection_cameraspace, Normal_cameraspace);
	float cosAlpha = clamp(dot(EyeDirection_cameraspace, reverse_normal), 0, 1);
	
	vec4 mat_color = texture(tex, Texcoord);
	
	out_color = 
		mat_color * ambient_color + 
		mat_color * light_color * cosTheta +
		mat_color * light_color * pow(cosAlpha, 8) / (dist * dist);
}