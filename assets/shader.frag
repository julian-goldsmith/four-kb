#version 430

in vec2 Texcoord;
in vec4 position_ws;
in vec4 eyedir_ws;
in vec4 lightdir_ws;
in float light_dist;
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

	vec4 normal_val = vec4(normalize(texture(normal_tex, Texcoord).rgb), 0.0);
	vec3 normal_ws = normalize(normal_mat * normal_val).xyz;
	
	float diff = max(dot(lightdir_ws.xyz, normal_ws), 0.0);
	
	vec3 halfway_dir = normalize(lightdir_ws + eyedir_ws).xyz;
	float spec = pow(max(dot(normal_ws, halfway_dir), 0.0), 16.0);
	
	out_color = vec4(normal_ws, 1.0);
		//vec4(spec, spec, spec, 1.0) +
		//vec4(diff, diff, diff, 1.0);
}
