// The vertex shader processes each incoming vertex. 
// It takes its attributes, like world position, color, 
// normal and texture coordinates as input.

// The output values will then be interpolated over the 
// fragments by the rasterizer to produce a smooth gradient.

// Shaders work on GLOBAL variables, not on passing arguments

// It's also possible to compile shaders directly from code. 
// The Vulkan SDK includes `libshaderc`, which is a library 
// to compile GLSL code to SPIR-V from within program.


#version 450

// dvec3 64 bit vectors, use multiple slots 
// That means that the index after it must be at least 2 higher

// layout(location = 0) in dvec3 inPosition;
// layout(location = 2) in vec3  inColor;


layout(location = 0) in vec2 inPosition;
layout(location = 1) in vec3 inColor;

layout(location = 0) out vec3 fragColor;


void main() {
    gl_Position = vec4(inPosition, 0.0, 1.0);
    fragColor = inColor;
}