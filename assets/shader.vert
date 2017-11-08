#version 150

in vec3 position;
in vec2 texcoord;

out vec2 Texcoord;
out vec3 position_ws;
out vec3 eyedir_ws;
out vec3 lightdir_ws;
out mat3 normal_mat;

uniform mat4 trans;
uniform mat4 proj;
uniform mat4 view;

void main() {
	vec3 lightpos_ws = vec3(0, 0, 0);
	vec3 camerapos_ws = vec3(0, 0, 0);			/* FIXME: make a uniform */
	
	position_ws = (trans * vec4(position, 1)).xyz;
	
	lightdir_ws = normalize(lightpos_ws - position_ws);
	eyedir_ws = normalize(camerapos_ws - position_ws);

	normal_mat = mat3(transpose(inverse(trans)));
	
	Texcoord = texcoord;
	
	gl_Position = proj * view * vec4(position_ws, 1.0);
}
