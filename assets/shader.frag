#version 150

in vec2 Texcoord;
in vec3 position_ws;
in vec3 eyedir_ws;
in vec3 lightdir_ws;
in mat4 normal_mat;

out vec4 out_color;

uniform sampler2D tex;
uniform sampler2D normal_tex;
uniform mat4 trans;
uniform mat4 proj;
uniform mat4 view;

void main() {
	vec4 light_color = vec4(0.8, 0.8, 0.8, 1.0);
	vec4 ambient_color = vec4(0.1, 0.1, 0.1, 0.1);
	
	vec4 mat_color = texture(tex, Texcoord);

	vec3 normal_val = texture(normal_tex, Texcoord).rgb;
	vec3 normal = normalize(normal_mat * vec4(normal_val, 0)).xyz;
	vec3 normal_ws = (normal * 2.0 - 1.0);
	
	float diff = max(dot(lightdir_ws, normal_ws), 0.0);
	
	vec3 halfway_dir = normalize(lightdir_ws + eyedir_ws);
	float spec = pow(max(dot(normal_ws, halfway_dir), 0.0), 32.0);
	
	out_color = vec4(normal_ws, 1.0);
		//vec4(spec, spec, spec, 1.0);
		//vec4(diff, diff, diff, 1.0);

}
