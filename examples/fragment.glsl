// this is a simple fragment shader used as the default
// fragment shader in case a user-defined fragment
// shader is not passed as a commandline argument

in vec3 vertex_normal;
in vec3 texture_coordinate;

uniform float time;
uniform vec2 resolution;
uniform sampler2D image;

const float PI = 3.14159265359;

void main() {
    float sigma = 0.8;
    float constant = PI + sigma * (3.0 * PI - 4.0) / 6.0;
    float alpha = 1.0 / constant;
    float beta = sigma / constant;

    vec3 light_direction = vec3(0.0, 1.5, 1.25);
    vec3 view_direction = vec3(0.0, 5.0, 5.0);

    vec4 albedo = vec4(1.0, 1.0, 1.0, 1.0);

    if (textureSize(image, 0).x > 1 && textureSize(image, 0).y > 1) {
        albedo = texture(image, texture_coordinate.xy);
    }

    float nl = max(dot(vertex_normal, light_direction), 0.0);
    float nv = max(dot(vertex_normal, view_direction), 0.0);
    float lv = dot(light_direction, view_direction);

    float s = lv - nl * nv;
    float t = 1.0;

    if (s > 0.0) {
            t = max(nl, nv);
    }

    gl_FragColor = albedo * nl * (alpha + beta * s / t);
}
