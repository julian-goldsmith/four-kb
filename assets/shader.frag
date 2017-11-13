#version 430

in vec2 Texcoord;
in vec4 position_ws;
in vec4 eyedir_ws;
in vec4 lightdir_ws;
in float light_dist;
in vec3 normal_ws;

out vec4 out_color;

uniform sampler2D tex;
uniform sampler2D normal_tex;
uniform mat4 trans;
uniform mat4 proj;
uniform mat4 view;

void main() {
	vec4 light_color = vec4(0.8, 0.8, 0.8, 1.0);
	
	vec4 mat_color = texture(tex, Texcoord);

	vec3 normal_map = normalize(texture(normal_tex, Texcoord).rgb) * 2 - 1;
	vec3 normal_adj = normalize(normal_ws + normal_map);
	
	float diff = max(dot(lightdir_ws.xyz, normal_adj), 0.0) / light_dist;
	
	vec3 halfway_dir = normalize(lightdir_ws + eyedir_ws).xyz;
	float spec = pow(max(dot(normal_adj, halfway_dir), 0.0), 16.0) / light_dist;
	
	out_color = 
		vec4(spec, spec, spec, 1.0) +
		vec4(diff, diff, diff, 1.0);
}
