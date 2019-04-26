#version 450

layout(location = 0) in vec2 position;
layout(location = 0) out vec4 cool;
layout(location = 1) out vec2 yep;
layout(location = 2) out float monkey;

void main() {
  cool = vec4(0.0, 0.5, 1.0, 1.0);
  yep = position;
  monkey = 0.9;
  gl_Position = vec4(yep, 0.0, 1.0);
}

