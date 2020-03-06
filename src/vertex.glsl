in vec3 position;
in vec3 texture;
in vec3 normal;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 rotation;
uniform mat4 translation;

out vec3 vertex_normal;
out vec3 texture_coordinate;

void main() {
    vertex_normal = normal;
    texture_coordinate = texture;
    gl_Position = projection * view * translation * rotation * vec4(position, 1.0);
}
