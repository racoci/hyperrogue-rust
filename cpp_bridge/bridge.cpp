// C++ FFI Bridge binding directly to the original HyperRogue repository
// (/home/racoci/Projects/mine/hyperrogue) without modifying any file in it.

#define SDLVER 2
#define CAP_AUDIO 0
#define CAP_SDLTTF 0
#define CAP_SDLGFX 0
#define CAP_GLEW 0
#define CAP_PNG 0
#define CAP_TRANS 0
#define CAP_TABFONT 1
#define NOMAIN

#include <SDL2/SDL.h>
#ifndef KMOD_SCROLL
#define KMOD_SCROLL 0x0040
#endif
#define mouseX x
#define mouseY y

#include <chrono>
#include "hyper.h"

namespace hr {
extern "C" {

// ---------------------------------------------------------------------------
// Geometry State Helpers
// ---------------------------------------------------------------------------

static void ensure_std_hyperbolic() {
    if (!cgip || !(cgi.state & 1) || geometry != gNormal || variation != eVariation::bitruncated) {
        geometry = gNormal;
        variation = eVariation::bitruncated;
        check_cgi();
        cgi.require_basics();
    }
}

static void ensure_nil_mode(double model_used) {
    if (!cgip || !(cgi.state & 1) || geometry != gNil) {
        geometry = gNil;
        variation = eVariation::pure;
        check_cgi();
        cgi.require_basics();
    }
    nilv::model_used = model_used;
}

// ---------------------------------------------------------------------------
// 1. Continuous Hyperbolic Geometry & Transformations (hyperpoint.cpp)
// ---------------------------------------------------------------------------

double cpp_squar(double x) {
    return squar(x);
}

double cpp_acosh_clamp(double x) {
    return acosh_clamp(x);
}

double cpp_acos_clamp(double x) {
    return acos_clamp(x);
}

double cpp_asin_clamp(double x) {
    return asin_clamp(x);
}

double cpp_edge_of_triangle_with_angles(double alpha, double beta, double gamma) {
    ensure_std_hyperbolic();
    return edge_of_triangle_with_angles(alpha, beta, gamma);
}

double cpp_geo_inner(double x1, double y1, double z1, double x2, double y2, double z2) {
    ensure_std_hyperbolic();
    return geo_inner(hyperpoint(x1, y1, z1, 0), hyperpoint(x2, y2, z2, 0));
}

double cpp_intval(double x1, double y1, double z1, double x2, double y2, double z2) {
    ensure_std_hyperbolic();
    return intval(hyperpoint(x1, y1, z1, 0), hyperpoint(x2, y2, z2, 0));
}

double cpp_hdist0(double x, double y, double z) {
    ensure_std_hyperbolic();
    return hdist0(hyperpoint(x, y, z, 0));
}

double cpp_hdist(double x1, double y1, double z1, double x2, double y2, double z2) {
    ensure_std_hyperbolic();
    return hdist(hyperpoint(x1, y1, z1, 0), hyperpoint(x2, y2, z2, 0));
}

double cpp_zlevel(double x, double y, double z) {
    ensure_std_hyperbolic();
    return zlevel(hyperpoint(x, y, z, 0));
}

void cpp_normalize(double x, double y, double z, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = normalize(hyperpoint(x, y, z, 0));
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_mid(double x1, double y1, double z1, double x2, double y2, double z2, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = mid(hyperpoint(x1, y1, z1, 0), hyperpoint(x2, y2, z2, 0));
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_cspin_apply(int a, int b, double alpha, double x, double y, double z, double w, double* out4) {
    ensure_std_hyperbolic();
    hyperpoint res = Hypc;
    res = cspin(a, b, alpha) * hyperpoint(x, y, z, w);
    out4[0] = res[0]; out4[1] = res[1]; out4[2] = res[2]; out4[3] = w;
}

void cpp_spin_apply(double alpha, double x, double y, double z, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = spin(alpha) * hyperpoint(x, y, z, 0);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_cpush_apply(int cid, double alpha, double x, double y, double z, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = cpush(cid, alpha) * hyperpoint(x, y, z, 0);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_xpush_apply(double alpha, double x, double y, double z, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = xpush(alpha) * hyperpoint(x, y, z, 0);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_ypush_apply(double alpha, double x, double y, double z, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = ypush(alpha) * hyperpoint(x, y, z, 0);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_xpush0(double alpha, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = xpush0(alpha);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_ypush0(double alpha, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = ypush0(alpha);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_xspinpush0(double alpha, double d, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = xspinpush0(alpha, d);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_spintox_apply(double hx, double hy, double hz, double px, double py, double pz, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = spintox(hyperpoint(hx, hy, hz, 0)) * hyperpoint(px, py, pz, 0);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_rspintox_apply(double hx, double hy, double hz, double px, double py, double pz, double* ox, double* oy, double* oz) {
    ensure_std_hyperbolic();
    hyperpoint res = rspintox(hyperpoint(hx, hy, hz, 0)) * hyperpoint(px, py, pz, 0);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

int cpp_eqmatrix(const double* a16, const double* b16, double eps) {
    ensure_std_hyperbolic();
    transmatrix A, B;
    for (int i = 0; i < 4; i++)
        for (int j = 0; j < 4; j++) {
            A[i][j] = a16[i * 4 + j];
            B[i][j] = b16[i * 4 + j];
        }
    return eqmatrix(A, B, eps) ? 1 : 0;
}

void cpp_kleinize(double x, double y, double z, double* ou, double* ov) {
    ensure_std_hyperbolic();
    hyperpoint res = kleinize(hyperpoint(x, y, z, 0));
    *ou = res[0]; *ov = res[1];
}

void cpp_poincare(double x, double y, double z, double* ou, double* ov) {
    ensure_std_hyperbolic();
    pmodel = mdDisk;
    pconf.alpha = 1.0;
    pconf.scale = 1.0;
    vid.camera = 0.0;
    vid.depth = 0.0;
    hyperpoint ret;
    applymodel(shiftless(hyperpoint(x, y, z, 0)), ret);
    *ou = ret[0]; *ov = ret[1];
}

void cpp_hybrid_disk(double x, double y, double z, double alpha, double* ou, double* ov) {
    ensure_std_hyperbolic();
    pmodel = mdDisk;
    pconf.alpha = alpha;
    pconf.scale = 1.0;
    vid.camera = 0.0;
    vid.depth = 0.0;
    hyperpoint ret;
    applymodel(shiftless(hyperpoint(x, y, z, 0)), ret);
    *ou = ret[0]; *ov = ret[1];
}

void cpp_compute_standard_geom_params(double* out8) {
    geometry = gNormal;
    variation = eVariation::bitruncated;
    check_cgi();
    cgi.state = 0;
    cgi.require_basics();
    out8[0] = cgi.tessf;
    out8[1] = cgi.hcrossf;
    out8[2] = cgi.crossf;
    out8[3] = cgi.hexf;
    out8[4] = cgi.rhexf;
    out8[5] = cgi.edgelen;
    out8[6] = cgi.hexvdist;
    out8[7] = cgi.hexhexdist;
}

// ---------------------------------------------------------------------------
// 2. Discrete Tiling Engine ({7,3} Heptagonal & Bitruncated)
// ---------------------------------------------------------------------------

int cpp_heptagon_bfs_count(int radius, int* dist_counts) {
    ensure_std_hyperbolic();
    hrmap_hyperbolic* m = new hrmap_hyperbolic();
    hrmap* old = currentmap;
    currentmap = m;

    std::map<heptagon*, int> dmap;
    std::queue<heptagon*> q;
    heptagon* orig = m->getOrigin();
    dmap[orig] = 0;
    q.push(orig);

    for (int i = 0; i <= radius; i++) dist_counts[i] = 0;

    while (!q.empty()) {
        heptagon* h = q.front();
        q.pop();
        int d = dmap[h];
        dist_counts[d]++;
        if (d < radius) {
            for (int dir = 0; dir < 7; dir++) {
                heptagon* n = createStep(h, dir);
                if (dmap.find(n) == dmap.end()) {
                    dmap[n] = d + 1;
                    q.push(n);
                }
            }
        }
    }

    int total = (int)dmap.size();
    currentmap = old;
    delete m;
    return total;
}

int cpp_bitrunc_disk_expand(int radius, int* dist_counts) {
    ensure_std_hyperbolic();
    hrmap_hyperbolic* m = new hrmap_hyperbolic();
    hrmap* old = currentmap;
    currentmap = m;

    std::map<cell*, int> dmap;
    std::queue<cell*> q;
    cell* orig = m->gamestart();
    dmap[orig] = 0;
    q.push(orig);

    for (int i = 0; i <= radius; i++) dist_counts[i] = 0;

    while (!q.empty()) {
        cell* c = q.front();
        q.pop();
        int d = dmap[c];
        dist_counts[d]++;
        if (d < radius) {
            for (int dir = 0; dir < c->type; dir++) {
                cell* n = createMov(c, dir);
                if (dmap.find(n) == dmap.end()) {
                    dmap[n] = d + 1;
                    q.push(n);
                }
            }
        }
    }

    int total = (int)dmap.size();
    currentmap = old;
    delete m;
    return total;
}

void cpp_cell_geometry_ops(int dir, double* spin_ang, double* space_d, double* adj_center3, double* corner3) {
    ensure_std_hyperbolic();
    hrmap_hyperbolic* m = new hrmap_hyperbolic();
    hrmap* old = currentmap;
    currentmap = m;

    cell* orig = m->gamestart();
    *spin_ang = m->spin_angle(orig, dir);
    *space_d = m->spacedist(orig, dir);

    transmatrix A = m->adj(orig, dir);
    hyperpoint ac = A * C0;
    adj_center3[0] = ac[0]; adj_center3[1] = ac[1]; adj_center3[2] = ac[2];

    hyperpoint co = m->get_corner(orig, dir, 3.0);
    corner3[0] = co[0]; corner3[1] = co[1]; corner3[2] = co[2];

    currentmap = old;
    delete m;
}

int cpp_walker_walk(const int* turns_and_steps, int num_ops, int* out_degree, int* out_spin) {
    ensure_std_hyperbolic();
    hrmap_hyperbolic* m = new hrmap_hyperbolic();
    hrmap* old = currentmap;
    currentmap = m;

    cellwalker cw(m->gamestart(), 0, false);
    for (int i = 0; i < num_ops; i++) {
        int op = turns_and_steps[i];
        if (op == 999) {
            cw += wstep;
        } else {
            cw += op;
        }
    }

    int returned_origin = (cw.at == m->gamestart()) ? 1 : 0;
    *out_degree = cw.at->type;
    *out_spin = cw.spin;

    currentmap = old;
    delete m;
    return returned_origin;
}

// ---------------------------------------------------------------------------
// 3. Nil Geometry & Heisenberg Group (nonisotropic.cpp)
// ---------------------------------------------------------------------------

double cpp_sym_to_heis_bonus(double x, double y) {
    return nilv::sym_to_heis_bonus(hyperpoint(x, y, 0, 1));
}

void cpp_convert_nil(double x, double y, double z, double from, double to, double* ox, double* oy, double* oz) {
    hyperpoint res = nilv::convert(hyperpoint(x, y, z, 1), from, to);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_mvec_heis_mul(int x1, int y1, int z1, int x2, int y2, int z2, int* ox, int* oy, int* oz) {
    nilv::nil_structure_index = 0;
    nilv::mvec a(x1, y1, z1);
    nilv::mvec b(x2, y2, z2);
    nilv::mvec r = a * b;
    *ox = r[0]; *oy = r[1]; *oz = r[2];
}

void cpp_mvec_heis_inv(int x, int y, int z, int* ox, int* oy, int* oz) {
    nilv::nil_structure_index = 0;
    nilv::mvec a(x, y, z);
    nilv::mvec r = a.inverse();
    *ox = r[0]; *oy = r[1]; *oz = r[2];
}

void cpp_mvec_hex_mul(int x1, int y1, int z1, int x2, int y2, int z2, int* ox, int* oy, int* oz) {
    nilv::nil_structure_index = 2;
    nilv::mvec a(x1, y1, z1);
    nilv::mvec b(x2, y2, z2);
    nilv::mvec r = a * b;
    *ox = r[0]; *oy = r[1]; *oz = r[2];
}

void cpp_mvec_hex_inv(int x, int y, int z, int* ox, int* oy, int* oz) {
    nilv::nil_structure_index = 2;
    nilv::mvec a(x, y, z);
    nilv::mvec r = a.inverse();
    *ox = r[0]; *oy = r[1]; *oz = r[2];
}

void cpp_nil_translate_apply(double hx, double hy, double hz, double px, double py, double pz, double model_used, double* ox, double* oy, double* oz) {
    ensure_nil_mode(model_used);
    transmatrix T = nisot::translate(hyperpoint(hx, hy, hz, 1));
    hyperpoint res = T * hyperpoint(px, py, pz, 1);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_nil_translate_inv_apply(double hx, double hy, double hz, double px, double py, double pz, double model_used, double* ox, double* oy, double* oz) {
    ensure_nil_mode(model_used);
    transmatrix T = nisot::translate(hyperpoint(hx, hy, hz, 1), -1);
    hyperpoint res = T * hyperpoint(px, py, pz, 1);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

double cpp_nil_dist0(double x, double y, double z) {
    ensure_nil_mode(0.0);
    return hdist0(hyperpoint(x, y, z, 1));
}

void cpp_nil_formula_exp(double vx, double vy, double vz, double model_used, double* ox, double* oy, double* oz) {
    ensure_nil_mode(model_used);
    hyperpoint res = nilv::formula_exp(hyperpoint(vx, vy, vz, 0));
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_nil_get_inverse_exp(double hx, double hy, double hz, double model_used, double* ox, double* oy, double* oz) {
    ensure_nil_mode(model_used);
    hyperpoint res = nilv::get_inverse_exp(hyperpoint(hx, hy, hz, 1), 0);
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_nil_on_geodesic(double x0, double y0, double z0, double x1, double y1, double z1, double t, double model_used, double* ox, double* oy, double* oz) {
    ensure_nil_mode(model_used);
    hyperpoint res = nilv::on_geodesic(
        hyperpoint(x0, y0, z0, 1),
        hyperpoint(x1, y1, z1, 1),
        t
    );
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

void cpp_nil_mvec_to_point(int mx, int my, int mz, double nilwidth, int ns_idx, double model_used, double* ox, double* oy, double* oz) {
    ensure_nil_mode(model_used);
    nilv::nilwidth = nilwidth;
    nilv::nil_structure_index = ns_idx;
    hyperpoint res = nilv::mvec_to_point(nilv::mvec(mx, my, mz));
    *ox = res[0]; *oy = res[1]; *oz = res[2];
}

int cpp_nil_map_expand(int ns_idx, int px, int py, int pz, int radius) {
    ensure_nil_mode(0.0);
    nilv::nil_structure_index = ns_idx;
    nilv::nilperiod = {px, py, pz};
    nilv::set_flags();

    hrmap* m = nisot::new_map();
    hrmap* old = currentmap;
    currentmap = m;

    std::map<heptagon*, int> dmap;
    std::queue<heptagon*> q;
    heptagon* orig = m->getOrigin();
    dmap[orig] = 0;
    q.push(orig);

    int deg = (int)nilv::current_ns().movevectors.size();
    while (!q.empty()) {
        heptagon* h = q.front();
        q.pop();
        int d = dmap[h];
        if (d < radius) {
            for (int i = 0; i < deg; i++) {
                heptagon* n = m->may_create_step(h, i);
                if (dmap.find(n) == dmap.end()) {
                    dmap[n] = d + 1;
                    q.push(n);
                }
            }
        }
    }

    int total = (int)dmap.size();
    currentmap = old;
    delete m;
    return total;
}

// ---------------------------------------------------------------------------
// 4. PRNG & Content Tables (game.cpp, content.cpp)
// ---------------------------------------------------------------------------

void cpp_rng_hrandpos_seq(unsigned int seed, int count, int* out) {
    shrand(seed);
    for (int i = 0; i < count; i++) out[i] = hrandpos();
}

void cpp_rng_hrand_seq(unsigned int seed, int mod, int count, int* out) {
    shrand(seed);
    for (int i = 0; i < count; i++) out[i] = hrand(mod);
}

void cpp_rng_hrandf_seq(unsigned int seed, int count, double* out) {
    shrand(seed);
    for (int i = 0; i < count; i++) out[i] = hrandf();
}

void cpp_rng_chance_seq(unsigned int seed, double p, int count, unsigned char* out) {
    shrand(seed);
    for (int i = 0; i < count; i++) out[i] = chance(p) ? 1 : 0;
}

// ---------------------------------------------------------------------------
// 5. High-Iteration Pure C++ Benchmarks (Zero FFI Overhead)
// ---------------------------------------------------------------------------

double cpp_bench_hdist_ns(int iters, double* sink) {
    ensure_std_hyperbolic();
    hyperpoint p1 = xpush0(0.75);
    hyperpoint p2 = xspinpush0(1.2, 1.35);
    double acc = 0.0;
    auto t0 = std::chrono::steady_clock::now();
    for (int i = 0; i < iters; i++) {
        p1[0] += 1e-12;
        acc += hdist(p1, p2);
    }
    auto t1 = std::chrono::steady_clock::now();
    *sink = acc;
    return std::chrono::duration<double, std::nano>(t1 - t0).count();
}

double cpp_bench_xpush_spin_ns(int iters, double* sink) {
    ensure_std_hyperbolic();
    hyperpoint p(0.5, 0.3, std::sqrt(1.0 + 0.25 + 0.09), 0.0);
    double acc = 0.0;
    auto t0 = std::chrono::steady_clock::now();
    for (int i = 0; i < iters; i++) {
        double a = 0.001 * (i & 255);
        hyperpoint q = xpush(a) * (spin(a) * p);
        acc += q[2];
    }
    auto t1 = std::chrono::steady_clock::now();
    *sink = acc;
    return std::chrono::duration<double, std::nano>(t1 - t0).count();
}

double cpp_bench_nil_exp_ns(int iters, double* sink) {
    ensure_nil_mode(0.0);
    double acc = 0.0;
    auto t0 = std::chrono::steady_clock::now();
    for (int i = 0; i < iters; i++) {
        double w = 0.2 + 0.001 * (i & 511);
        hyperpoint r = nilv::formula_exp(hyperpoint(0.5, 0.3, w, 0.0));
        acc += r[2];
    }
    auto t1 = std::chrono::steady_clock::now();
    *sink = acc;
    return std::chrono::duration<double, std::nano>(t1 - t0).count();
}

double cpp_bench_nil_inv_exp_ns(int iters, double* sink) {
    ensure_nil_mode(0.0);
    double acc = 0.0;
    auto t0 = std::chrono::steady_clock::now();
    for (int i = 0; i < iters; i++) {
        double z = 0.5 + 0.001 * (i & 255);
        hyperpoint r = nilv::get_inverse_exp(hyperpoint(0.35, 0.45, z, 1.0), 0);
        acc += r[2];
    }
    auto t1 = std::chrono::steady_clock::now();
    *sink = acc;
    return std::chrono::duration<double, std::nano>(t1 - t0).count();
}

double cpp_bench_mvec_mul_ns(int iters, int* sink) {
    nilv::nil_structure_index = 0;
    nilv::mvec acc(0, 0, 0);
    int acc_z = 0;
    auto t0 = std::chrono::steady_clock::now();
    for (int i = 0; i < iters; i++) {
        nilv::mvec a((i & 15) - 7, ((i >> 4) & 15) - 7, (i & 31));
        nilv::mvec b(((i >> 2) & 15) - 7, ((i >> 6) & 15) - 7, 1);
        nilv::mvec r = a * b;
        acc_z += r[2];
    }
    auto t1 = std::chrono::steady_clock::now();
    *sink = acc_z;
    return std::chrono::duration<double, std::nano>(t1 - t0).count();
}

double cpp_bench_bitrunc_expand_ns(int radius, int* sink) {
    ensure_std_hyperbolic();
    int counts[16] = {0};
    auto t0 = std::chrono::steady_clock::now();
    int total = cpp_bitrunc_disk_expand(radius, counts);
    auto t1 = std::chrono::steady_clock::now();
    *sink = total;
    return std::chrono::duration<double, std::nano>(t1 - t0).count();
}

double cpp_bench_nil_expand_ns(int radius, int* sink) {
    auto t0 = std::chrono::steady_clock::now();
    int total = cpp_nil_map_expand(0, 0, 0, 0, radius);
    auto t1 = std::chrono::steady_clock::now();
    *sink = total;
    return std::chrono::duration<double, std::nano>(t1 - t0).count();
}

// ---------------------------------------------------------------------------
// 6. Content Tables, Living Caves CA & Combat Parity (classes.cpp, complex.cpp, attack.cpp)
// ---------------------------------------------------------------------------

unsigned int cpp_land_color(int idx) {
    const eLand lands[] = {
        laCrossroads, laDesert, laIce, laCaves, laJungle, laAlchemist,
        laMirror, laGraveyard, laRlyeh, laHell, laCocytus, laMotion,
        laDryForest, laEmerald, laWineyard, laDeadCaves, laHive, laPower,
        laCamelot, laTemple
    };
    return linf[lands[idx]].color;
}

char cpp_monster_glyph(int idx) {
    const eMonster mons[] = {
        moYeti, moWolf, moRanger, moTroll, moGoblin, moWorm,
        moHedge, moDesertman, moIvyRoot, moMonkey, moSlime,
        moMimic, moGolem, moEagle, moSeep, moZombie, moGhost
    };
    return minf[mons[idx]].glyph;
}

unsigned int cpp_monster_color(int idx) {
    const eMonster mons[] = {
        moYeti, moWolf, moRanger, moTroll, moGoblin, moWorm,
        moHedge, moDesertman, moIvyRoot, moMonkey, moSlime,
        moMimic, moGolem, moEagle, moSeep, moZombie, moGhost
    };
    return minf[mons[idx]].color;
}

char cpp_item_glyph(int idx) {
    const eItem its[] = {
        itDiamond, itGold, itSpice, itRuby, itElixir, itShard,
        itBone, itHell, itStatue, itFeather, itSapphire, itHyperstone,
        itKey, itOrbYendor, itOrbLightning, itOrbFlash, itOrbWinter, itOrbSpeed
    };
    return iinf[its[idx]].glyph;
}

unsigned int cpp_item_color(int idx) {
    const eItem its[] = {
        itDiamond, itGold, itSpice, itRuby, itElixir, itShard,
        itBone, itHell, itStatue, itFeather, itSapphire, itHyperstone,
        itKey, itOrbYendor, itOrbLightning, itOrbFlash, itOrbWinter, itOrbSpeed
    };
    return iinf[its[idx]].color;
}

// Simulate Living Caves CA on origin cell surrounded by 7 neighbor configs:
// wall_code: 0 = waCavefloor, 1 = waCavewall, 2 = waDeadTroll
// item_code: 0 = itNone, 1 = itGold
// monst_code: 0 = moNone, 1 = moZombie
// Returns resulting center wall_code (0 = waCavefloor, 1 = waCavewall)
int cpp_living_caves_step(
    int center_wall_code,
    const int* neighbor_wall7,
    const int* neighbor_item7,
    const int* neighbor_monst7
) {
    ensure_std_hyperbolic();
    hrmap_hyperbolic* m = new hrmap_hyperbolic();
    hrmap* old = currentmap;
    currentmap = m;

    cell* center = m->gamestart();
    center->land = laCaves;
    center->wall = (center_wall_code == 1) ? waCavewall : waCavefloor;
    center->item = itNone;
    center->monst = moNone;
    center->cpdist = 2; // Not player cell (cpdist != 0)

    dcal.clear();
    dcal.push_back(center);

    for (int d = 0; d < 7; d++) {
        cell* n = createMov(center, d);
        n->land = laCaves;
        n->cpdist = 3;
        n->wall = (neighbor_wall7[d] == 2) ? waDeadTroll :
                  (neighbor_wall7[d] == 1) ? waCavewall : waCavefloor;
        n->item = (neighbor_item7[d] == 1) ? itGold : itNone;
        n->monst = (neighbor_monst7[d] == 1) ? moZombie : moNone;
        // Surround neighbors with stone so only center is evaluated cleanly
        dcal.push_back(n);
    }

    livecaves();
    int result = (center->wall == waCavewall) ? 1 : 0;

    dcal.clear();
    currentmap = old;
    delete m;
    return result;
}

// Execute killMonster(center, moPlayer) on a Rock Troll and return resulting walls:
// center_out: 2 = waDeadTroll
// neighbor_walls7: 0 = waCavefloor, 1 = waCavewall (converted from waDeadfloor/waDeadwall)
void cpp_kill_troll_effect(const int* init_neighbor_dead7, int* center_wall_out, int* out_neighbor7) {
    ensure_std_hyperbolic();
    hrmap_hyperbolic* m = new hrmap_hyperbolic();
    hrmap* old = currentmap;
    currentmap = m;

    cell* center = m->gamestart();
    center->land = laCaves;
    center->wall = waCavefloor;
    center->monst = moTroll;
    center->item = itGold;

    for (int d = 0; d < 7; d++) {
        cell* n = createMov(center, d);
        n->land = laCaves;
        n->monst = moNone;
        n->item = itGold;
        n->wall = (init_neighbor_dead7[d] == 1) ? waDeadwall : waDeadfloor;
    }

    cwt.at = createMov(center, 0);
    killMonster(center, moPlayer, 0);

    *center_wall_out = (center->wall == waDeadTroll && center->monst == moNone && center->item == itNone) ? 2 : -1;
    for (int d = 0; d < 7; d++) {
        cell* n = center->move(d);
        out_neighbor7[d] = (n->wall == waCavewall) ? 1 : (n->wall == waCavefloor ? 0 : -1);
    }

    currentmap = old;
    delete m;
}

double cpp_bench_living_caves_ns(int steps, int* sink) {
    ensure_std_hyperbolic();
    hrmap_hyperbolic* m = new hrmap_hyperbolic();
    hrmap* old = currentmap;
    currentmap = m;

    std::map<cell*, int> dmap;
    std::queue<cell*> q;
    cell* orig = m->gamestart();
    dmap[orig] = 0;
    q.push(orig);
    dcal.clear();

    while (!q.empty()) {
        cell* c = q.front();
        q.pop();
        int d = dmap[c];
        c->land = laCaves;
        c->cpdist = d + 1;
        c->wall = ((dcal.size() % 2) == 0) ? waCavewall : waCavefloor;
        c->item = itNone;
        c->monst = moNone;
        dcal.push_back(c);
        if (d < 4) {
            for (int dir = 0; dir < c->type; dir++) {
                cell* n = createMov(c, dir);
                if (dmap.find(n) == dmap.end()) {
                    dmap[n] = d + 1;
                    q.push(n);
                }
            }
        }
    }

    auto t0 = std::chrono::steady_clock::now();
    for (int s = 0; s < steps; s++) {
        livecaves();
    }
    auto t1 = std::chrono::steady_clock::now();

    int wall_cnt = 0;
    for (cell* c : dcal) if (c->wall == waCavewall) wall_cnt++;
    *sink = wall_cnt;

    dcal.clear();
    currentmap = old;
    delete m;
    return std::chrono::duration<double, std::nano>(t1 - t0).count();
}

} // extern "C"
} // namespace hr
