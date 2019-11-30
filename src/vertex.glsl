in vec3 position;
in vec3 normal;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 rotation;
uniform mat4 translation;

out vec3 vertex_normal;

void main() {
    vertex_normal = normal;
    gl_Position = projection * view * translation * rotation * vec4(position, 1.0);
}
