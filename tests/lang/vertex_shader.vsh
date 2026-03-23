// 12 lines 7 code 2 comments 3 blanks

attribute vec4 position;
attribute vec2 texCoord;

varying vec2 vTexCoord;

// pass through to fragment shader
void main() {
    vTexCoord = texCoord;
    gl_Position = position;
}
