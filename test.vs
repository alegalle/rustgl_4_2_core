#version 330

uniform mat4 projectionMatrix;

in vec3 vertex;
in vec4 colour;

out vec4 fragColour;

void main()
{
    fragColour = colour;
    gl_Position = projectionMatrix*vec4(vertex,1.0f);
}
