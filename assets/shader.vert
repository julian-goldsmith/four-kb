#version 430

in vec3 position;
in vec3 normal;
in vec2 texcoord;

out vec2 Texcoord;
out vec4 position_ws;
out vec4 eyedir_ws;
out vec4 lightdir_ws;
out float light_dist;
out vec3 normal_ws;

uniform mat4 trans;
uniform mat4 proj;
uniform mat4 view;

void main() {
	vec4 lightpos_ws = vec4(-1, 0, 0, 1.0);
	vec4 camerapos_ws = vec4(0, 0, 0, 1.0);	/* FIXME: make a uniform */
	
	position_ws = trans * vec4(position, 1.0);
	
	lightdir_ws = normalize(lightpos_ws - position_ws);
	light_dist = distance(lightpos_ws, position_ws);
	eyedir_ws = normalize(camerapos_ws - position_ws);

	normal_ws = normalize(trans * vec4(normal, 0.0)).xyz;
	
	Texcoord = texcoord;
	
	gl_Position = proj * view * position_ws;
}
