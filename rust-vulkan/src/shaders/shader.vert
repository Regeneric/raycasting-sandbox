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

layout(location = 0) out vec3 fragColor;

vec2 positions[3] = vec2[](
    vec2( 0.0, -0.5),
    vec2( 0.5,  0.5),
    vec2(-0.5,  0.5)
);

// Per-vertex color
vec3 colors[3] = vec3[](
    vec3(1.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    vec3(0.0, 0.0, 1.0)
);

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 0.1);
    fragColor = colors[gl_VertexIndex];     // Pass per-vertex colors to the fragment shader so it can output their interpolated values to the framebuffer.
}