in vec3 position;
in vec3 texture;
in vec3 normal;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

out vec3 vertex_normal;
out vec3 texture_coordinate;

void main() {
    vertex_normal = normalize(view * model * vec4(normal, 0.0)).xyz;
    texture_coordinate = texture;
    gl_Position = projection * view * model * vec4(position, 1.0);
}
