in vec3 position;
in vec3 texture;
in vec3 normal;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 rotation;
uniform mat4 translation;
uniform mat4 scale;

out vec3 vertex_normal;
out vec3 texture_coordinate;

void main() {
    mat4 model_view = view * translation * rotation * scale;
    vertex_normal = normalize(model_view * vec4(normal, 0.0)).xyz;
    texture_coordinate = texture;
    gl_Position = projection * model_view * vec4(position, 1.0);
}
