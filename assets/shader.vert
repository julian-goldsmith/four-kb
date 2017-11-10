#version 150

in vec3 position;
in vec2 texcoord;

out vec2 Texcoord;
out vec4 position_ws;
out vec4 eyedir_ws;
out vec4 lightdir_ws;
out mat4 normal_mat;

uniform mat4 trans;
uniform mat4 proj;
uniform mat4 view;

void main() {
	vec4 lightpos_ws = vec4(-10, 0, -5, 1.0);
	vec4 camerapos_ws = vec4(0, 0, 0, 1.0);	/* FIXME: make a uniform */
	
	position_ws = trans * vec4(position, 1.0);
	
	lightdir_ws = normalize(lightpos_ws - position_ws);
	eyedir_ws = normalize(camerapos_ws - position_ws);

	normal_mat = transpose(inverse(trans));
	
	Texcoord = texcoord;
	
	gl_Position = proj * view * position_ws;
}
