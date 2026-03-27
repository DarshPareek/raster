#version 330 core

#define MARKER_COLOR vec4(0.7, 0.0, 0.0, 1.0)
#define BEZIER_CURVE_THRESHOLD 10.0

uniform vec2 p1;
uniform vec2 p2;
uniform vec2 p3;
uniform float marker_radius;

void main() {
  if (length(gl_FragCoord.xy - p1) < marker_radius ||
      length(gl_FragCoord.xy - p2) < marker_radius ||
      length(gl_FragCoord.xy - p3) < marker_radius) {
    gl_FragColor = MARKER_COLOR;
  } else {
    vec2 p0 = gl_FragCoord.xy;
    float ax = p3.x  -2 * p2.x + p1.x;
    float bx = 2 * (p2.x - p1.x);
    float cx = p1.x - p0.x;
    float dx = bx * bx - 4.0 * ax * cx;
    if (dx > 0.0) {
        float t1 = (-bx + sqrt(dx)) / (2 * ax);
        float t2 = (-bx - sqrt(dx)) / (2 * ax);
        float y1 = p1.y + 2.0 * t1 * (p2.y - p1.y) + t1 * t1 * (p3.y - 2.0 * p2.y + p1.y);
        float y2 = p1.y + 2.0 * t2 * (p2.y - p1.y) + t2 * t2 * (p3.y - 2.0 * p2.y + p1.y);
        if ((0.0 <= t1 && t1 <= 1.0 && abs(p0.y - y1) < BEZIER_CURVE_THRESHOLD)
            || (0.0 <= t2 && t2 <= 1.0 && abs(p0.y - y2) < BEZIER_CURVE_THRESHOLD)){
                gl_FragColor = vec4(0.75, 0.75, 0.0, 1.0);
            } else {
                gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
            }
    } else {
    gl_FragColor = vec4(0.0);
    }
  }
}
