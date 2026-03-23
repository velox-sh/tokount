// 12 lines 7 code 2 comments 3 blanks

precision mediump float;

varying vec2 vTexCoord;
uniform sampler2D uTexture;

// sample and output the texture color
void main() {
    vec4 color = texture2D(uTexture, vTexCoord);
    gl_FragColor = color;
}
