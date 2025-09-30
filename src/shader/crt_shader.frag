#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
// uniform vec2 u_resolution;

// https://www.shadertoy.com/view/XtlSD7

vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void DrawVignette(inout vec3 color, vec2 uv)
{
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}


void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
    color *= scanline * grille * 1.2;
}

vec4 addRotatedGrid(vec2 fragCoord, vec2 resolution) {

    // Center coordinates
    vec2 uv = fragCoord - resolution / 2.0;

    // Rotate 45 degrees
    float angle = 3.14159265 / 4.0;
    float cosA = cos(angle);
    float sinA = sin(angle);
    vec2 r = vec2(
        uv.x * cosA - uv.y * sinA,
        uv.x * sinA + uv.y * cosA
    );

    float gridSize = 5.0;

    // Distance to nearest grid line
    float dx = abs(mod(r.x, gridSize) - gridSize/2.0);
    float dy = abs(mod(r.y, gridSize) - gridSize/2.0);

    float thickness = 1.5;

    if(dx < thickness || dy < thickness) {
        return vec4(0.188, 0.302, 0.267, 0.5);
    } else {
        return vec4(0.0, 0.0, 0.0, 0.0);
    }
}

vec4 addHorizontalLines(vec2 fragCoord) {
    
    vec2 uv = fragCoord;
    float lineSpacing = 16.0;   
    float thickness = 5.0;     
    float dy = abs(mod(uv.y, lineSpacing) - lineSpacing / 2.0);

    if (dy < thickness / 2.0) {
        return vec4(0.184, 0.298, 0.259, 0.8);
    } else {
        return vec4(0.0, 0.0, 0.0, 0.0); 
    }

}

vec4 addCenterVignette(vec2 fragCoord, vec2 resolution) {

    vec2 center = resolution * 0.5;
    vec2 uv = fragCoord - center;

    float scale = min(resolution.x, resolution.y) * 0.5;
    float radius = length(uv) / scale;

    float vignetteRadius = 0.4; 
    float smoothness = 0.7;     

    float alpha = 1.0 - smoothstep(vignetteRadius - smoothness, vignetteRadius + smoothness, radius);

    vec3 color = vec3(0.184, 0.357, 0.188);

    return vec4(color, alpha);
}

void main() {
    vec2 resolution = vec2(800, 600);
    vec2 crtUV = CRTCurveUV(uv);
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;

    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    {
        res = vec3(0.0, 0.0, 0.0);
    }
    
    // Rotated grid overlay
    vec4 grid  = addRotatedGrid(gl_FragCoord.xy, vec2(100,100));
    vec4 lines = addHorizontalLines(gl_FragCoord.xy);
    vec4 vignette = addCenterVignette(gl_FragCoord.xy, resolution);

    res = mix(res, vignette.rgb, vignette.a);
    res = mix(res, grid.rgb, grid.a);   
    res = mix(res, lines.rgb, lines.a);
    
    DrawVignette(res, crtUV);

    gl_FragColor = vec4(res, 1.0);
}

