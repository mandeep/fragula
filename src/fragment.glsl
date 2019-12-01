in vec3 vertex_normal;

out vec3 frag_color;

void main() {
    vec3 object_color = vec3(0.6, 0.6, 0.6);
    vec3 light_direction = vec3(0.0, -1.0, -0.5);
    float kd = max(dot(vertex_normal, -light_direction), 0.0);

    frag_color = kd * object_color;
}
