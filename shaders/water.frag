#version 140

in vec2 v_tex_coords;
out vec4 color;
in vec2 pos;

uniform float time;
uniform sampler2D tex;

float rand(vec2 n) { 
	return fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453);
}

float mod289(float x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 mod289(vec4 x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 perm(vec4 x){return mod289(((x * 34.0) + 1.0) * x);}

float noise3d(vec3 p){
    vec3 a = floor(p);
    vec3 d = p - a;
    d = d * d * (3.0 - 2.0 * d);

    vec4 b = a.xxyy + vec4(0.0, 1.0, 0.0, 1.0);
    vec4 k1 = perm(b.xyxy);
    vec4 k2 = perm(k1.xyxy + b.zzww);

    vec4 c = k2 + a.zzzz;
    vec4 k3 = perm(c);
    vec4 k4 = perm(c + 1.0);

    vec4 o1 = fract(k3 * (1.0 / 41.0));
    vec4 o2 = fract(k4 * (1.0 / 41.0));

    vec4 o3 = o2 * d.z + o1 * (1.0 - d.z);
    vec2 o4 = o3.yw * d.x + o3.xz * (1.0 - d.x);

    return o4.y * d.y + o4.x * (1.0 - d.y);
}


//	<https://www.shadertoy.com/view/Xd23Dh>
//	by inigo quilez <http://iquilezles.org/www/articles/voronoise/voronoise.htm>
//

vec3 hash3( vec2 p ){
    vec3 q = vec3( dot(p,vec2(127.1,311.7)), 
				   dot(p,vec2(269.5,183.3)), 
				   dot(p,vec2(419.2,371.9)) );
	return fract(sin(q)*43758.5453);
}

float iqnoise( in vec2 x, float u, float v ){
    vec2 p = floor(x);
    vec2 f = fract(x);
		
	float k = 1.0+63.0*pow(1.0-v,4.0);
	
	float va = 0.0;
	float wt = 0.0;
    for( int j=-2; j<=2; j++ )
    for( int i=-2; i<=2; i++ )
    {
        vec2 g = vec2( float(i),float(j) );
		vec3 o = hash3( p + g )*vec3(u,u,1.0);
		vec2 r = g - f + o.xy;
		float d = dot(r,r);
		float ww = pow( 1.0-smoothstep(0.0,1.414,sqrt(d)), k );
		va += o.z*ww;
		wt += ww;
    }
	
    return va/wt;
}


void main() {
	if(v_tex_coords.y > 0.5) {
		color = texture(tex, vec2(v_tex_coords.x, v_tex_coords.y*2.0));
		return;
	}

	vec2 p = 40.0*pos.xy;
	float n1 = noise3d(vec3(p.xy, 0.8*time));
	float n2 = noise3d(vec3(p.y, 0.6*time, p.x));
	vec2 offset = vec2(n1, n2);

	vec2 res = vec2(0.65, 0.65) + 0.2*sin(time*1.3);
	float noiseval = iqnoise(p + offset + time, res.x+n1, res.y+n2);

	vec2 texcoords = vec2(v_tex_coords.x, 1.0-v_tex_coords.y*2.0) + 0.016 * offset;

	vec4 texcolor = texture(tex, texcoords);

	float specular = 3.0*pow(noiseval, 8.0);
	vec4 highlight = specular * vec4(1.0) + (1-specular) * texcolor;

	highlight = min(highlight, 0.98);

	vec4 waterbase = vec4(0.1, 0.1, 0.95, 1.0);

	float opacity = noiseval*0.55;
	vec4 watercolor = opacity * waterbase + (1 - opacity) * texcolor;

	vec4 blendcolor = (1/(5.0+5.0))*(5.0*highlight + 5.0*watercolor);

	color = blendcolor;	

	return;
}