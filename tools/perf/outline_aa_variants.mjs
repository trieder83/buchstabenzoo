// Outline anti-aliasing prototypes (Q-191, PERF-R-016) as shaderSource patches of the
// outline pass — input for `turn.mjs` (TURN_VARIANTS_FILE) and, via a written-out variant,
// for `look.mjs ab` (AB_INIT_B). Round 2026-09-29 (paused): `box4` = the edge averaged over a
// 2 × 2 pixel block (the recommended look; hot px −75…−84 %), `tent5` (calmer, dearer),
// `tri3` / `diag2` (cheaper, less calm), `box4d` / `box4n` (depth / normal edges only).
// Usage: TURN_VARIANTS=box4 TURN_VARIANTS_FILE=tools/perf/outline_aa_variants.mjs \
//        node tools/perf/turn.mjs flicker web/dist <out>
const patch = (body) => `{
  const src = WebGL2RenderingContext.prototype.shaderSource;
  const body = ${JSON.stringify(body)};
  WebGL2RenderingContext.prototype.shaderSource = function (s, text) {
    const re = /void main\\(\\) \\{\\n    vec2 o = u_texel \\* u_px;[\\s\\S]*?float edge = max\\(depth_edge, normal_edge\\);/;
    if (re.test(text)) { text = text.replace(re, body); window.__patched = (window.__patched || 0) + 1; }
    return src.call(this, s, text);
  };
}`;
const edgeFn = `
float q_edge(vec2 uv, vec2 o) {
    float c = inv_lin(uv);
    float l = inv_lin(uv - vec2(o.x, 0.0));
    float r = inv_lin(uv + vec2(o.x, 0.0));
    float b = inv_lin(uv - vec2(0.0, o.y));
    float t = inv_lin(uv + vec2(0.0, o.y));
    float lap = max(2.0 * c - l - r, 2.0 * c - b - t) / c;
    float depth_edge = smoothstep(0.012, 0.03, lap);
    vec4 nc = texture(u_normal, uv);
    vec4 nl = texture(u_normal, uv - vec2(o.x, 0.0));
    vec4 nb = texture(u_normal, uv - vec2(0.0, o.y));
    vec3 n0 = nc.xyz * 2.0 - 1.0;
    float nd = 1.0;
    nd = min(nd, mix(1.0, dot(n0, nl.xyz * 2.0 - 1.0), min(nc.a, nl.a)));
    nd = min(nd, mix(1.0, dot(n0, nb.xyz * 2.0 - 1.0), min(nc.a, nb.a)));
    float normal_edge = 1.0 - smoothstep(0.55, 0.75, nd);
    return max(depth_edge, normal_edge);
}
`;
// 2x2 box: the edge at this pixel and its right / upper / diagonal neighbours
const box4 = edgeFn + `
void main() {
    vec2 o = u_texel * u_px;
    vec2 t1 = u_texel;
    float edge = 0.25 * (q_edge(v_uv, o) + q_edge(v_uv + vec2(t1.x, 0.0), o)
        + q_edge(v_uv + vec2(0.0, t1.y), o) + q_edge(v_uv + t1, o));`;
// tent: centre 1/2, 4 neighbours 1/8
const tent5 = edgeFn + `
void main() {
    vec2 o = u_texel * u_px;
    vec2 t1 = u_texel;
    float edge = 0.5 * q_edge(v_uv, o) + 0.125 * (q_edge(v_uv + vec2(t1.x, 0.0), o)
        + q_edge(v_uv - vec2(t1.x, 0.0), o) + q_edge(v_uv + vec2(0.0, t1.y), o) + q_edge(v_uv - vec2(0.0, t1.y), o));`;
// 3-tap: centre 1/2, right and up 1/4 (cheaper, 3 evaluations)
const tri3 = edgeFn + `
void main() {
    vec2 o = u_texel * u_px;
    vec2 t1 = u_texel;
    float edge = 0.5 * q_edge(v_uv, o) + 0.25 * (q_edge(v_uv + vec2(t1.x, 0.0), o) + q_edge(v_uv + vec2(0.0, t1.y), o));`;

const splitFn = `
float q_depth(vec2 uv, vec2 o) {
    float c = inv_lin(uv);
    float l = inv_lin(uv - vec2(o.x, 0.0));
    float r = inv_lin(uv + vec2(o.x, 0.0));
    float b = inv_lin(uv - vec2(0.0, o.y));
    float t = inv_lin(uv + vec2(0.0, o.y));
    float lap = max(2.0 * c - l - r, 2.0 * c - b - t) / c;
    return smoothstep(0.012, 0.03, lap);
}
float q_norm(vec2 uv, vec2 o) {
    vec4 nc = texture(u_normal, uv);
    vec4 nl = texture(u_normal, uv - vec2(o.x, 0.0));
    vec4 nb = texture(u_normal, uv - vec2(0.0, o.y));
    vec3 n0 = nc.xyz * 2.0 - 1.0;
    float nd = 1.0;
    nd = min(nd, mix(1.0, dot(n0, nl.xyz * 2.0 - 1.0), min(nc.a, nl.a)));
    nd = min(nd, mix(1.0, dot(n0, nb.xyz * 2.0 - 1.0), min(nc.a, nb.a)));
    return 1.0 - smoothstep(0.55, 0.75, nd);
}
`;
const diag2 = edgeFn + `
void main() {
    vec2 o = u_texel * u_px;
    float edge = 0.5 * (q_edge(v_uv, o) + q_edge(v_uv + u_texel, o));`;
const box4d = splitFn + `
void main() {
    vec2 o = u_texel * u_px;
    vec2 t1 = u_texel;
    float de = 0.25 * (q_depth(v_uv, o) + q_depth(v_uv + vec2(t1.x, 0.0), o)
        + q_depth(v_uv + vec2(0.0, t1.y), o) + q_depth(v_uv + t1, o));
    float edge = max(de, q_norm(v_uv, o));`;
const box4n = splitFn + `
void main() {
    vec2 o = u_texel * u_px;
    vec2 t1 = u_texel;
    float ne = 0.25 * (q_norm(v_uv, o) + q_norm(v_uv + vec2(t1.x, 0.0), o)
        + q_norm(v_uv + vec2(0.0, t1.y), o) + q_norm(v_uv + t1, o));
    float edge = max(q_depth(v_uv, o), ne);`;
export default {
  diag2: patch(diag2),
  box4d: patch(box4d),
  box4n: patch(box4n),
  box4: patch(box4),
  tent5: patch(tent5),
  tri3: patch(tri3),
};
