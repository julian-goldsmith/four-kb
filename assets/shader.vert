#version 430

uniform mat4 trans;
uniform mat4 proj;
uniform mat4 view;

in vec3 position;
in vec3 normal;
in vec3 tangent;
in vec2 texcoord;

out vec2 Texcoord;
out vec4 position_ws;
out vec3 Normal;
out vec3 Tangent;
out vec3 Bitangent;
out vec4 lightpos_ws;
out vec4 camerapos_ws;

void main() {
	lightpos_ws = vec4(-1, 0, 0, 1.0);
	camerapos_ws = vec4(0, 0, 0, 1.0);	/* FIXME: make a uniform */
	
	position_ws = vec4(position, 1.0);

	//normal_ws = normalize(trans * vec4(normal, 0.0)).xyz;
	Normal = normal;
	Tangent = tangent;
	Bitangent = cross(normal, tangent);
	
	Texcoord = texcoord;
	
	gl_Position = proj * view * trans * position_ws;
}
