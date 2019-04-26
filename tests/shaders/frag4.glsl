#version 450

layout(location = 0) out vec4 f_color;

layout(push_constant) uniform PushConstantData {
    float time;
} pc;

void main() {
  f_color = vec4(pc.time, 0.5, 1.0, 1.0);
}
