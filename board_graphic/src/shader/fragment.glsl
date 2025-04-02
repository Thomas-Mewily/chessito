#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec4 previousColor; // Uniform to store the previous color

/*
void main() {
    vec4 color = texture2D(Texture, uv);
    float gray = (color.r+color.r+color.b)/3.0;
    //gl_FragColor = vec4(gray, gray, gray, 1.0);
    gl_FragColor = vec4(gray, gray, gray, color.a);

    //gl_FragColor = color;
}*/

void main() 
{
    //vec2 nuv = uv; //vec2(1.0- uv.x, uv.y);
    //vec4 color = texture2D(Texture, nuv);
    // Perform blending with the previous color (destination color)
    /*
    if (color.a >= 0.5)
    {
        gl_FragColor = color;
    }else
    {
        gl_FragColor = previousColor;
    }*/
    //gl_FragColor = (previousColor + color) / 2.;
    //gl_FragColor = color / 2.;

    //vec2 c =  gl_Color;
    //vec4 color = gl_Color;



    gl_FragColor = texture2D(Texture, uv); // / 2.;
    
    gl_FragColor.rgb = vec3((gl_FragColor.r + gl_FragColor.g + gl_FragColor.b)/3.);
}