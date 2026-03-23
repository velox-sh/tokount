// 20 lines 14 code 3 comments 3 blanks

// vertex shader input/output
struct VSInput {
    float4 position : POSITION;
    float2 uv       : TEXCOORD0;
};

struct PSInput {
    float4 position : SV_POSITION;
    float2 uv       : TEXCOORD0;
};

/* vertex entry point */
PSInput VSMain(VSInput input) {
    PSInput output;
    output.position = input.position;
    output.uv = input.uv;
    return output;
}
