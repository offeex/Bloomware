#version 400

in vec3 Position;
in vec4 Color;
in vec2 TextureCoords;

uniform mat4 ModelViewMat;
uniform mat4 ProjMat;

out vec4 vertexColor;
out vec2 texCoord;

void main() {
    gl_Position = ProjMat * ModelViewMat * vec4(Position, 1.0);

    vertexColor = Color;
    texCoord = vec2(TextureCoords);
}
