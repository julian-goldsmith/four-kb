#version 150

in vec3 position;
in vec2 texcoord;

out vec2 Texcoord;
out vec3 Position_worldspace;
out vec3 EyeDirection_cameraspace;
out vec3 LightDirection_cameraspace;
out float dist;

uniform mat4 trans;
uniform mat4 proj;
uniform mat4 view;

void main() {
	mat4 mvp = proj * view * trans;
	
	vec3 LightPosition_worldspace = vec3(0, 0, 0);
	
	Position_worldspace = (trans * vec4(position, 1)).xyz;
	
	vec3 vertexPosition_cameraspace = (view * trans * vec4(position, 1)).xyz;
	EyeDirection_cameraspace = normalize(vec3(0, 0, 0) - vertexPosition_cameraspace);
	
	vec3 LightPosition_cameraspace = (view * vec4(LightPosition_worldspace, 1)).xyz;
	LightDirection_cameraspace = normalize(LightPosition_cameraspace + EyeDirection_cameraspace);
	
	dist = distance(Position_worldspace, LightPosition_worldspace);
	
	Texcoord = texcoord;
	gl_Position = mvp * vec4(position, 1.0);
}
