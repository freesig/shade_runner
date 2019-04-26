#version 450

layout(location = 0) in vec4 cool;
layout(location = 1) in vec2 yep;
layout(location = 2) in float monkey;
layout(location = 0) out vec4 f_color;

void main() {
  vec4 t = cool;
  t.yw += yep;
  t.x -= monkey;
  f_color = t;
}
