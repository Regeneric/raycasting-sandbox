// The triangle that is formed by the positions from the 
// vertex shader fills an area on the screen with fragments.

#version 450

layout(location = 0) out vec4 outColor;
layout(location = 0) in  vec3 fragColor;

void main() {
    // outColor = vec4(1.0, 0.0, 0.0, 1.0);  // Color RED (R,G,B,A)
    outColor = vec4(fragColor, 1.0);         // Per-vertex colors (vec3(RGB),A)
}