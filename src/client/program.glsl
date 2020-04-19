varying vec2 v_vt;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
uniform vec2 u_pos;
uniform vec2 u_size;
uniform float u_rotation;
uniform mat4 u_view_matrix;
void main()
{
    v_vt = (a_pos + vec2(1.0, 1.0)) / 2.0;
    v_vt.y = 1.0 - v_vt.y;
    float sn = sin(u_rotation), cs = cos(u_rotation);
    vec2 pos = a_pos * u_size;
    pos = vec2(pos.x * cs - pos.y * sn, pos.x * sn + pos.y * cs);
    gl_Position = u_view_matrix * vec4(u_pos + pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
uniform sampler2D u_texture;
void main()
{
    gl_FragColor = texture2D(u_texture, v_vt) * u_color;
}
#endif