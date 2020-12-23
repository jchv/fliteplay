#[derive(Copy, Clone)]
pub struct CFFTPlanI {
    pub packplan: CFFTPPlan,
    pub blueplan: FFTBluePlan,
}
pub type FFTBluePlan = *mut FFTBluePlanI;
#[derive(Copy, Clone)]
pub struct FFTBluePlanI {
    pub n: usize,
    pub n2: usize,
    pub plan: CFFTPPlan,
    pub mem: *mut f64,
    pub bk: *mut f64,
    pub bkf: *mut f64,
}
pub type CFFTPPlan = *mut CFFTPPlanI;
#[derive(Copy, Clone)]
pub struct CFFTPPlanI {
    pub length: usize,
    pub nfct: usize,
    pub mem: *mut Complex,
    pub fct: [CFFTPFctData; 25],
}
#[derive(Copy, Clone)]
pub struct CFFTPFctData {
    pub fct: usize,
    pub tw: *mut Complex,
    pub tws: *mut Complex,
}
#[derive(Copy, Clone)]
pub struct Complex {
    pub r: f64,
    pub i: f64,
}
pub type CFFTPlan = *mut CFFTPlanI;
#[derive(Copy, Clone)]
pub struct RFFTPlanI {
    pub packplan: RFFTPPlan,
    pub blueplan: FFTBluePlan,
}
pub type RFFTPPlan = *mut RFFTPPlanI;
#[derive(Copy, Clone)]
pub struct RFFTPPlanI {
    pub length: usize,
    pub nfct: usize,
    pub mem: *mut f64,
    pub fct: [RFFTPFctData; 25],
}
#[derive(Copy, Clone)]
pub struct RFFTPFctData {
    pub fct: usize,
    pub tw: *mut f64,
    pub tws: *mut f64,
}
pub type RFFTPlan = *mut RFFTPlanI;
// adapted from https://stackoverflow.com/questions/42792939/
// CAUTION: this function only works for arguments in the range [-0.25; 0.25]!
unsafe fn my_sincosm1pi(a: f64, res: *mut f64) {
    let mut s: f64 = a * a;
    /* Approximate cos(pi*x)-1 for x in [-0.25,0.25] */
    let mut r: f64 = -1.0369917389758117e-4f64;
    r = f64::mul_add(r, s, 1.9294935641298806e-3f64);
    r = f64::mul_add(r, s, -2.5806887942825395e-2f64);
    r = f64::mul_add(r, s, 2.3533063028328211e-1f64);
    r = f64::mul_add(r, s, -1.3352627688538006e+0f64);
    r = f64::mul_add(r, s, 4.0587121264167623e+0f64);
    r = f64::mul_add(r, s, -4.9348022005446790e+0f64);
    let c: f64 = r * s;
    /* Approximate sin(pi*x) for x in [-0.25,0.25] */
    r = 4.6151442520157035e-4f64;
    r = f64::mul_add(r, s, -7.3700183130883555e-3f64);
    r = f64::mul_add(r, s, 8.2145868949323936e-2f64);
    r = f64::mul_add(r, s, -5.9926452893214921e-1f64);
    r = f64::mul_add(r, s, 2.5501640398732688e+0f64);
    r = f64::mul_add(r, s, -5.1677127800499516e+0f64);
    s = s * a;
    r = r * s;
    s = f64::mul_add(a, 3.1415926535897931e+0f64, r);
    *res.offset(0) = c;
    *res.offset(1) = s;
}
#[inline(never)]
unsafe fn calc_first_octant(den: usize, res: *mut f64) {
    let n: usize = den.wrapping_add(4 as libc::c_int as usize) >> 3 as libc::c_int;
    if n == 0 {
        return;
    }
    *res.offset(0) = 1.0f64;
    *res.offset(1) = 0.0f64;
    if n == 1 as libc::c_int as usize {
        return;
    }
    let l1: usize = f64::sqrt(n as f64) as usize;
    let mut i: usize = 1 as libc::c_int as usize;
    while i < l1 {
        my_sincosm1pi(
            2.0f64 * i as f64 / den as f64,
            &mut *res.offset((2usize).wrapping_mul(i) as isize),
        );
        i = i.wrapping_add(1)
    }
    let mut start: usize = l1;
    while start < n {
        let mut cs: [f64; 2] = [0.; 2];
        my_sincosm1pi(2.0f64 * start as f64 / den as f64, cs.as_mut_ptr());
        *res.offset((2usize).wrapping_mul(start) as isize) = cs[0] + 1.0f64;
        *res.offset(
            (2usize)
                .wrapping_mul(start)
                .wrapping_add(1 as libc::c_int as usize) as isize,
        ) = cs[1 as libc::c_int as usize];
        let mut end: usize = l1;
        if start.wrapping_add(end) > n {
            end = n.wrapping_sub(start)
        }
        let mut i_0: usize = 1 as libc::c_int as usize;
        while i_0 < end {
            let csx: [f64; 2] = [
                *res.offset((2usize).wrapping_mul(i_0) as isize),
                *res.offset(
                    (2usize)
                        .wrapping_mul(i_0)
                        .wrapping_add(1 as libc::c_int as usize) as isize,
                ),
            ];
            *res.offset((2usize).wrapping_mul(start.wrapping_add(i_0)) as isize) = cs[0] * csx[0]
                - cs[1 as libc::c_int as usize] * csx[1 as libc::c_int as usize]
                + cs[0]
                + csx[0]
                + 1.0f64;
            *res.offset(
                (2usize)
                    .wrapping_mul(start.wrapping_add(i_0))
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            ) = cs[0] * csx[1 as libc::c_int as usize]
                + cs[1 as libc::c_int as usize] * csx[0]
                + cs[1 as libc::c_int as usize]
                + csx[1 as libc::c_int as usize];
            i_0 = i_0.wrapping_add(1)
        }
        start = (start as usize).wrapping_add(l1) as usize as usize
    }
    let mut i_1: usize = 1 as libc::c_int as usize;
    while i_1 < l1 {
        *res.offset((2usize).wrapping_mul(i_1) as isize) += 1.0f64;
        i_1 = i_1.wrapping_add(1)
    }
}
#[inline(never)]
unsafe fn calc_first_quadrant(n: usize, res: *mut f64) {
    let p: *mut f64 = res.offset(n as isize);
    calc_first_octant(n << 1 as libc::c_int, p);
    let ndone: usize = n.wrapping_add(2usize) >> 2 as libc::c_int;
    let mut i: usize = 0;
    let mut idx1: usize = 0;
    let mut idx2: usize = (2usize).wrapping_mul(ndone).wrapping_sub(2usize);
    while i.wrapping_add(1 as libc::c_int as usize) < ndone {
        *res.offset(idx1 as isize) = *p.offset((2usize).wrapping_mul(i) as isize);
        *res.offset(idx1.wrapping_add(1 as libc::c_int as usize) as isize) = *p.offset(
            (2usize)
                .wrapping_mul(i)
                .wrapping_add(1 as libc::c_int as usize) as isize,
        );
        *res.offset(idx2 as isize) = *p.offset(
            (2usize)
                .wrapping_mul(i)
                .wrapping_add(3 as libc::c_int as usize) as isize,
        );
        *res.offset(idx2.wrapping_add(1 as libc::c_int as usize) as isize) =
            *p.offset((2usize).wrapping_mul(i).wrapping_add(2usize) as isize);
        i = (i as usize).wrapping_add(2usize) as usize as usize;
        idx1 = (idx1 as usize).wrapping_add(2usize) as usize as usize;
        idx2 = (idx2 as usize).wrapping_sub(2usize) as usize as usize
    }
    if i != ndone {
        *res.offset(idx1 as isize) = *p.offset((2usize).wrapping_mul(i) as isize);
        *res.offset(idx1.wrapping_add(1 as libc::c_int as usize) as isize) = *p.offset(
            (2usize)
                .wrapping_mul(i)
                .wrapping_add(1 as libc::c_int as usize) as isize,
        )
    };
}
#[inline(never)]
unsafe fn calc_first_half(n: usize, res: *mut f64) {
    let ndone: libc::c_int =
        (n.wrapping_add(1 as libc::c_int as usize) >> 1 as libc::c_int) as libc::c_int;
    let p: *mut f64 = res.offset(n as isize).offset(-(1 as libc::c_int as isize));
    calc_first_octant(n << 2 as libc::c_int, p);
    let mut i4: libc::c_int = 0 as libc::c_int;
    let in_0: libc::c_int = n as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i4 <= in_0 - i4 {
        // octant 0
        *res.offset((2 as libc::c_int * i) as isize) = *p.offset((2 as libc::c_int * i4) as isize);
        *res.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) =
            *p.offset((2 as libc::c_int * i4 + 1 as libc::c_int) as isize);
        i += 1;
        i4 += 4 as libc::c_int
    }
    while i4 - in_0 <= 0 as libc::c_int {
        // octant 1
        let xm: libc::c_int = in_0 - i4;
        *res.offset((2 as libc::c_int * i) as isize) =
            *p.offset((2 as libc::c_int * xm + 1 as libc::c_int) as isize);
        *res.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) =
            *p.offset((2 as libc::c_int * xm) as isize);
        i += 1;
        i4 += 4 as libc::c_int
    }
    while i4 <= 3 as libc::c_int * in_0 - i4 {
        // octant 2
        let xm_0: libc::c_int = i4 - in_0;
        *res.offset((2 as libc::c_int * i) as isize) =
            -*p.offset((2 as libc::c_int * xm_0 + 1 as libc::c_int) as isize);
        *res.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) =
            *p.offset((2 as libc::c_int * xm_0) as isize);
        i += 1;
        i4 += 4 as libc::c_int
    }
    while i < ndone {
        // octant 3
        let xm_1: libc::c_int = 2 as libc::c_int * in_0 - i4; // penalty for non-hardcoded larger factors
        *res.offset((2 as libc::c_int * i) as isize) =
            -*p.offset((2 as libc::c_int * xm_1) as isize); // penalize larger prime factors
        *res.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) =
            *p.offset((2 as libc::c_int * xm_1 + 1 as libc::c_int) as isize);
        i += 1;
        i4 += 4 as libc::c_int
    }
}
#[inline(never)]
unsafe fn fill_first_quadrant(n: usize, res: *mut f64) {
    let hsqt2: f64 = 0.707106781186547524400844362104849f64;
    let quart: usize = n >> 2 as libc::c_int;
    if n & 7 as libc::c_int as usize == 0 {
        let ref mut fresh0 = *res.offset(quart.wrapping_add(1 as libc::c_int as usize) as isize);
        *fresh0 = hsqt2;
        *res.offset(quart as isize) = *fresh0
    }
    let mut i: usize = 2usize;
    let mut j: usize = (2usize).wrapping_mul(quart).wrapping_sub(2usize);
    while i < quart {
        *res.offset(j as isize) = *res.offset(i.wrapping_add(1 as libc::c_int as usize) as isize);
        *res.offset(j.wrapping_add(1 as libc::c_int as usize) as isize) = *res.offset(i as isize);
        i = (i as usize).wrapping_add(2usize) as usize as usize;
        j = (j as usize).wrapping_sub(2usize) as usize as usize
    }
}
#[inline(never)]
unsafe fn fill_first_half(n: usize, res: *mut f64) {
    let half: usize = n >> 1 as libc::c_int;
    if n & 3 as libc::c_int as usize == 0 {
        let mut i: usize = 0;
        while i < half {
            *res.offset(i.wrapping_add(half) as isize) =
                -*res.offset(i.wrapping_add(1 as libc::c_int as usize) as isize);
            *res.offset(i.wrapping_add(half).wrapping_add(1 as libc::c_int as usize) as isize) =
                *res.offset(i as isize);
            i = (i as usize).wrapping_add(2usize) as usize as usize
        }
    } else {
        let mut i_0: usize = 2usize;
        let mut j: usize = (2usize).wrapping_mul(half).wrapping_sub(2usize);
        while i_0 < half {
            *res.offset(j as isize) = -*res.offset(i_0 as isize);
            *res.offset(j.wrapping_add(1 as libc::c_int as usize) as isize) =
                *res.offset(i_0.wrapping_add(1 as libc::c_int as usize) as isize);
            i_0 = (i_0 as usize).wrapping_add(2usize) as usize as usize;
            j = (j as usize).wrapping_sub(2usize) as usize as usize
        }
    };
}
#[inline(never)]
unsafe fn fill_second_half(n: usize, res: *mut f64) {
    if n & 1 as libc::c_int as usize == 0 {
        let mut i: usize = 0;
        while i < n {
            *res.offset(i.wrapping_add(n) as isize) = -*res.offset(i as isize);
            i = i.wrapping_add(1)
        }
    } else {
        let mut i_0: usize = 2usize;
        let mut j: usize = (2usize).wrapping_mul(n).wrapping_sub(2usize);
        while i_0 < n {
            *res.offset(j as isize) = *res.offset(i_0 as isize);
            *res.offset(j.wrapping_add(1 as libc::c_int as usize) as isize) =
                -*res.offset(i_0.wrapping_add(1 as libc::c_int as usize) as isize);
            i_0 = (i_0 as usize).wrapping_add(2usize) as usize as usize;
            j = (j as usize).wrapping_sub(2usize) as usize as usize
        }
    };
}
#[inline(never)]
unsafe fn sincos_2pibyn_half(n: usize, res: *mut f64) {
    if n & 3 as libc::c_int as usize == 0 {
        calc_first_octant(n, res);
        fill_first_quadrant(n, res);
        fill_first_half(n, res);
    } else if n & 1 as libc::c_int as usize == 0 {
        calc_first_quadrant(n, res);
        fill_first_half(n, res);
    } else {
        calc_first_half(n, res);
    };
}
#[inline(never)]
unsafe fn sincos_2pibyn(n: usize, res: *mut f64) {
    sincos_2pibyn_half(n, res);
    fill_second_half(n, res);
}
#[inline(never)]
unsafe fn largest_prime_factor(mut n: usize) -> usize {
    let mut res: usize = 1 as libc::c_int as usize;
    let mut tmp: usize;
    loop {
        tmp = n >> 1 as libc::c_int;
        if !(tmp << 1 as libc::c_int == n) {
            break;
        }
        res = 2usize;
        n = tmp
    }
    let mut limit: usize = f64::sqrt(n as f64 + 0.01f64) as usize;
    let mut x: usize = 3 as libc::c_int as usize;
    while x <= limit {
        loop {
            tmp = n.wrapping_div(x);
            if !(tmp.wrapping_mul(x) == n) {
                break;
            }
            res = x;
            n = tmp;
            limit = f64::sqrt(n as f64 + 0.01f64) as usize
        }
        x = (x as usize).wrapping_add(2usize) as usize as usize
    }
    if n > 1 as libc::c_int as usize {
        res = n
    }
    return res;
}
#[inline(never)]
unsafe fn cost_guess(mut n: usize) -> f64 {
    let lfp: f64 = 1.1f64;
    let ni: usize = n;
    let mut result: f64 = 0.0f64;
    let mut tmp: usize;
    loop {
        tmp = n >> 1 as libc::c_int;
        if !(tmp << 1 as libc::c_int == n) {
            break;
        }
        result += 2 as libc::c_int as f64;
        n = tmp
    }
    let mut limit: usize = f64::sqrt(n as f64 + 0.01f64) as usize;
    let mut x: usize = 3 as libc::c_int as usize;
    while x <= limit {
        loop {
            tmp = n.wrapping_div(x);
            if !(tmp.wrapping_mul(x) == n) {
                break;
            }
            result += if x <= 5 as libc::c_int as usize {
                x as f64
            } else {
                (lfp) * x as f64
            };
            n = tmp;
            limit = f64::sqrt(n as f64 + 0.01f64) as usize
        }
        x = (x as usize).wrapping_add(2usize) as usize as usize
    }
    if n > 1 as libc::c_int as usize {
        result += if n <= 5 as libc::c_int as usize {
            n as f64
        } else {
            (lfp) * n as f64
        }
    }
    return result * ni as f64;
}
/* returns the smallest composite of 2, 3, 5, 7 and 11 which is >= n */
#[inline(never)]
unsafe fn good_size(n: usize) -> usize {
    if n <= 6 as libc::c_int as usize {
        return n;
    }
    let mut bestfac: usize = (2usize).wrapping_mul(n);
    let mut f2: usize = 1 as libc::c_int as usize;
    while f2 < bestfac {
        let mut f23: usize = f2;
        while f23 < bestfac {
            let mut f235: usize = f23;
            while f235 < bestfac {
                let mut f2357: usize = f235;
                while f2357 < bestfac {
                    let mut f235711: usize = f2357;
                    while f235711 < bestfac {
                        if f235711 >= n {
                            bestfac = f235711
                        }
                        f235711 = (f235711 as usize).wrapping_mul(11 as libc::c_int as usize)
                            as usize as usize
                    }
                    f2357 =
                        (f2357 as usize).wrapping_mul(7 as libc::c_int as usize) as usize as usize
                }
                f235 = (f235 as usize).wrapping_mul(5 as libc::c_int as usize) as usize as usize
            }
            f23 = (f23 as usize).wrapping_mul(3 as libc::c_int as usize) as usize as usize
        }
        f2 = (f2 as usize).wrapping_mul(2usize) as usize as usize
    }
    return bestfac;
}
/* a = b*c */
/* a = conj(b)*c*/
/* a = b*c */
/* a *= b */
#[inline(never)]
unsafe fn pass2b(ido: usize, l1: usize, cc: *const Complex, ch: *mut Complex, wa: *const Complex) {
    let cdim: usize = 2usize;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let mut t: Complex = Complex { r: 0., i: 0. };
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize))
                .r * t.r
                    - (*wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize))
                    .i * t.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize))
                .r * t.i
                    + (*wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize))
                    .i * t.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn pass2f(ido: usize, l1: usize, cc: *const Complex, ch: *mut Complex, wa: *const Complex) {
    let cdim: usize = 2usize;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let mut t: Complex = Complex { r: 0., i: 0. };
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize))
                .r * t.r
                    + (*wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize))
                    .i * t.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize))
                .r * t.i
                    - (*wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize))
                    .i * t.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn pass3b(ido: usize, l1: usize, cc: *const Complex, ch: *mut Complex, wa: *const Complex) {
    let cdim: usize = 3 as libc::c_int as usize;
    let tw1r: f64 = -0.5f64;
    let tw1i: f64 = 0.86602540378443864676f64;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            let t0: Complex = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
            let mut t1: Complex = Complex { r: 0., i: 0. };
            let mut t2: Complex = Complex { r: 0., i: 0. };
            t1.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r;
            t1.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i;
            t2.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r;
            t2.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t0.r + t1.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t0.i + t1.i;
            let mut ca: Complex = Complex { r: 0., i: 0. };
            let mut cb: Complex = Complex { r: 0., i: 0. };
            ca.r = t0.r + tw1r * t1.r;
            ca.i = t0.i + tw1r * t1.i;
            cb.i = tw1i * t2.r;
            cb.r = -(tw1i * t2.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r + cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i + cb.i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca.r - cb.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca.i - cb.i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let t0_0: Complex = *cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            );
            let mut t1_0: Complex = Complex { r: 0., i: 0. };
            let mut t2_0: Complex = Complex { r: 0., i: 0. };
            t1_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r;
            t1_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i;
            t2_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r;
            t2_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t0_0.r + t1_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t0_0.i + t1_0.i;
            let mut ca_0: Complex = Complex { r: 0., i: 0. };
            let mut cb_0: Complex = Complex { r: 0., i: 0. };
            ca_0.r = t0_0.r + tw1r * t1_0.r;
            ca_0.i = t0_0.i + tw1r * t1_0.i;
            cb_0.i = tw1i * t2_0.r;
            cb_0.r = -(tw1i * t2_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca_0.r + cb_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca_0.i + cb_0.i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_0.r - cb_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_0.i - cb_0.i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let t0_1: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let mut t1_1: Complex = Complex { r: 0., i: 0. };
                let mut t2_1: Complex = Complex { r: 0., i: 0. };
                t1_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r;
                t1_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i;
                t2_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r;
                t2_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r = t0_1.r + t1_1.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i = t0_1.i + t1_1.i;
                let mut ca_1: Complex = Complex { r: 0., i: 0. };
                let mut cb_1: Complex = Complex { r: 0., i: 0. };
                let mut da: Complex = Complex { r: 0., i: 0. };
                let mut db: Complex = Complex { r: 0., i: 0. };
                ca_1.r = t0_1.r + tw1r * t1_1.r;
                ca_1.i = t0_1.i + tw1r * t1_1.i;
                cb_1.i = tw1i * t2_1.r;
                cb_1.r = -(tw1i * t2_1.i);
                da.r = ca_1.r + cb_1.r;
                da.i = ca_1.i + cb_1.i;
                db.r = ca_1.r - cb_1.r;
                db.i = ca_1.i - cb_1.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.r
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.i
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.r
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.i
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn pass3f(ido: usize, l1: usize, cc: *const Complex, ch: *mut Complex, wa: *const Complex) {
    let cdim: usize = 3 as libc::c_int as usize;
    let tw1r: f64 = -0.5f64;
    let tw1i: f64 = -0.86602540378443864676f64;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            let t0: Complex = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
            let mut t1: Complex = Complex { r: 0., i: 0. };
            let mut t2: Complex = Complex { r: 0., i: 0. };
            t1.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r;
            t1.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i;
            t2.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r;
            t2.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t0.r + t1.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t0.i + t1.i;
            let mut ca: Complex = Complex { r: 0., i: 0. };
            let mut cb: Complex = Complex { r: 0., i: 0. };
            ca.r = t0.r + tw1r * t1.r;
            ca.i = t0.i + tw1r * t1.i;
            cb.i = tw1i * t2.r;
            cb.r = -(tw1i * t2.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r + cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i + cb.i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca.r - cb.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca.i - cb.i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let t0_0: Complex = *cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            );
            let mut t1_0: Complex = Complex { r: 0., i: 0. };
            let mut t2_0: Complex = Complex { r: 0., i: 0. };
            t1_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r;
            t1_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i;
            t2_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r;
            t2_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t0_0.r + t1_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t0_0.i + t1_0.i;
            let mut ca_0: Complex = Complex { r: 0., i: 0. };
            let mut cb_0: Complex = Complex { r: 0., i: 0. };
            ca_0.r = t0_0.r + tw1r * t1_0.r;
            ca_0.i = t0_0.i + tw1r * t1_0.i;
            cb_0.i = tw1i * t2_0.r;
            cb_0.r = -(tw1i * t2_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca_0.r + cb_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca_0.i + cb_0.i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_0.r - cb_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_0.i - cb_0.i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let t0_1: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let mut t1_1: Complex = Complex { r: 0., i: 0. };
                let mut t2_1: Complex = Complex { r: 0., i: 0. };
                t1_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r;
                t1_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i;
                t2_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r;
                t2_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r = t0_1.r + t1_1.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i = t0_1.i + t1_1.i;
                let mut ca_1: Complex = Complex { r: 0., i: 0. };
                let mut cb_1: Complex = Complex { r: 0., i: 0. };
                let mut da: Complex = Complex { r: 0., i: 0. };
                let mut db: Complex = Complex { r: 0., i: 0. };
                ca_1.r = t0_1.r + tw1r * t1_1.r;
                ca_1.i = t0_1.i + tw1r * t1_1.i;
                cb_1.i = tw1i * t2_1.r;
                cb_1.r = -(tw1i * t2_1.i);
                da.r = ca_1.r + cb_1.r;
                da.i = ca_1.i + cb_1.i;
                db.r = ca_1.r - cb_1.r;
                db.i = ca_1.i - cb_1.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.r
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.i
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.r
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.i
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn pass4b(ido: usize, l1: usize, cc: *const Complex, ch: *mut Complex, wa: *const Complex) {
    let cdim: usize = 4 as libc::c_int as usize;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            let mut t1: Complex = Complex { r: 0., i: 0. };
            let mut t2: Complex = Complex { r: 0., i: 0. };
            let mut t3: Complex = Complex { r: 0., i: 0. };
            let mut t4: Complex = Complex { r: 0., i: 0. };
            t2.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r + (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r;
            t2.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i + (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i;
            t1.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r - (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r;
            t1.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i - (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i;
            t3.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t3.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t4.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t4.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            let tmp_: f64 = t4.r;
            t4.r = -t4.i;
            t4.i = tmp_;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t2.r + t3.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t2.i + t3.i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = t2.r - t3.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = t2.i - t3.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = t1.r + t4.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = t1.i + t4.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = t1.r - t4.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = t1.i - t4.i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let mut t1_0: Complex = Complex { r: 0., i: 0. };
            let mut t2_0: Complex = Complex { r: 0., i: 0. };
            let mut t3_0: Complex = Complex { r: 0., i: 0. };
            let mut t4_0: Complex = Complex { r: 0., i: 0. };
            t2_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r + (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r;
            t2_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i + (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i;
            t1_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r - (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r;
            t1_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i - (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i;
            t3_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t3_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t4_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t4_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            let tmp_0: f64 = t4_0.r;
            t4_0.r = -t4_0.i;
            t4_0.i = tmp_0;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t2_0.r + t3_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t2_0.i + t3_0.i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = t2_0.r - t3_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = t2_0.i - t3_0.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = t1_0.r + t4_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = t1_0.i + t4_0.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = t1_0.r - t4_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = t1_0.i - t4_0.i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let mut c2: Complex = Complex { r: 0., i: 0. };
                let mut c3: Complex = Complex { r: 0., i: 0. };
                let mut c4: Complex = Complex { r: 0., i: 0. };
                let mut t1_1: Complex = Complex { r: 0., i: 0. };
                let mut t2_1: Complex = Complex { r: 0., i: 0. };
                let mut t3_1: Complex = Complex { r: 0., i: 0. };
                let mut t4_1: Complex = Complex { r: 0., i: 0. };
                let cc0: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let cc1: Complex =
                    *cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize);
                let cc2: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let cc3: Complex =
                    *cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize);
                t2_1.r = cc0.r + cc2.r;
                t2_1.i = cc0.i + cc2.i;
                t1_1.r = cc0.r - cc2.r;
                t1_1.i = cc0.i - cc2.i;
                t3_1.r = cc1.r + cc3.r;
                t3_1.i = cc1.i + cc3.i;
                t4_1.r = cc1.r - cc3.r;
                t4_1.i = cc1.i - cc3.i;
                let tmp_1: f64 = t4_1.r;
                t4_1.r = -t4_1.i;
                t4_1.i = tmp_1;
                let wa0: Complex =
                    *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize);
                let wa1: Complex = *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                );
                let wa2: Complex =
                    *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize);
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r = t2_1.r + t3_1.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i = t2_1.i + t3_1.i;
                c3.r = t2_1.r - t3_1.r;
                c3.i = t2_1.i - t3_1.i;
                c2.r = t1_1.r + t4_1.r;
                c2.i = t1_1.i + t4_1.i;
                c4.r = t1_1.r - t4_1.r;
                c4.i = t1_1.i - t4_1.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = wa0.r * c2.r - wa0.i * c2.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = wa0.r * c2.i + wa0.i * c2.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .r = wa1.r * c3.r - wa1.i * c3.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .i = wa1.r * c3.i + wa1.i * c3.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .r = wa2.r * c4.r - wa2.i * c4.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .i = wa2.r * c4.i + wa2.i * c4.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn pass4f(ido: usize, l1: usize, cc: *const Complex, ch: *mut Complex, wa: *const Complex) {
    let cdim: usize = 4 as libc::c_int as usize;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            let mut t1: Complex = Complex { r: 0., i: 0. };
            let mut t2: Complex = Complex { r: 0., i: 0. };
            let mut t3: Complex = Complex { r: 0., i: 0. };
            let mut t4: Complex = Complex { r: 0., i: 0. };
            t2.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r + (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r;
            t2.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i + (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i;
            t1.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r - (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r;
            t1.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i - (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i;
            t3.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t3.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t4.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t4.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            let tmp_: f64 = -t4.r;
            t4.r = t4.i;
            t4.i = tmp_;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t2.r + t3.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t2.i + t3.i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = t2.r - t3.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = t2.i - t3.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = t1.r + t4.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = t1.i + t4.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = t1.r - t4.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = t1.i - t4.i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let mut t1_0: Complex = Complex { r: 0., i: 0. };
            let mut t2_0: Complex = Complex { r: 0., i: 0. };
            let mut t3_0: Complex = Complex { r: 0., i: 0. };
            let mut t4_0: Complex = Complex { r: 0., i: 0. };
            t2_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r + (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r;
            t2_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i + (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i;
            t1_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r - (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r;
            t1_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i - (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i;
            t3_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t3_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t4_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t4_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            let tmp_0: f64 = -t4_0.r;
            t4_0.r = t4_0.i;
            t4_0.i = tmp_0;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t2_0.r + t3_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t2_0.i + t3_0.i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = t2_0.r - t3_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = t2_0.i - t3_0.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = t1_0.r + t4_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = t1_0.i + t4_0.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = t1_0.r - t4_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = t1_0.i - t4_0.i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let mut c2: Complex = Complex { r: 0., i: 0. };
                let mut c3: Complex = Complex { r: 0., i: 0. };
                let mut c4: Complex = Complex { r: 0., i: 0. };
                let mut t1_1: Complex = Complex { r: 0., i: 0. };
                let mut t2_1: Complex = Complex { r: 0., i: 0. };
                let mut t3_1: Complex = Complex { r: 0., i: 0. };
                let mut t4_1: Complex = Complex { r: 0., i: 0. };
                let cc0: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let cc1: Complex =
                    *cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize);
                let cc2: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let cc3: Complex =
                    *cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize);
                t2_1.r = cc0.r + cc2.r;
                t2_1.i = cc0.i + cc2.i;
                t1_1.r = cc0.r - cc2.r;
                t1_1.i = cc0.i - cc2.i;
                t3_1.r = cc1.r + cc3.r;
                t3_1.i = cc1.i + cc3.i;
                t4_1.r = cc1.r - cc3.r;
                t4_1.i = cc1.i - cc3.i;
                let tmp_1: f64 = -t4_1.r;
                t4_1.r = t4_1.i;
                t4_1.i = tmp_1;
                let wa0: Complex =
                    *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize);
                let wa1: Complex = *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                );
                let wa2: Complex =
                    *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize);
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r = t2_1.r + t3_1.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i = t2_1.i + t3_1.i;
                c3.r = t2_1.r - t3_1.r;
                c3.i = t2_1.i - t3_1.i;
                c2.r = t1_1.r + t4_1.r;
                c2.i = t1_1.i + t4_1.i;
                c4.r = t1_1.r - t4_1.r;
                c4.i = t1_1.i - t4_1.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = wa0.r * c2.r + wa0.i * c2.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = wa0.r * c2.i - wa0.i * c2.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .r = wa1.r * c3.r + wa1.i * c3.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .i = wa1.r * c3.i - wa1.i * c3.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .r = wa2.r * c4.r + wa2.i * c4.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .i = wa2.r * c4.i - wa2.i * c4.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn pass5b(ido: usize, l1: usize, cc: *const Complex, ch: *mut Complex, wa: *const Complex) {
    let cdim: usize = 5 as libc::c_int as usize;
    let tw1r: f64 = 0.3090169943749474241f64;
    let tw1i: f64 = 0.95105651629515357212f64;
    let tw2r: f64 = -0.8090169943749474241f64;
    let tw2i: f64 = 0.58778525229247312917f64;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            let t0: Complex = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
            let mut t1: Complex = Complex { r: 0., i: 0. };
            let mut t2: Complex = Complex { r: 0., i: 0. };
            let mut t3: Complex = Complex { r: 0., i: 0. };
            let mut t4: Complex = Complex { r: 0., i: 0. };
            t1.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t1.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t4.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t4.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t2.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t2.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t3.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t3.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t0.r + t1.r + t2.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t0.i + t1.i + t2.i;
            let mut ca: Complex = Complex { r: 0., i: 0. };
            let mut cb: Complex = Complex { r: 0., i: 0. };
            ca.r = t0.r + tw1r * t1.r + tw2r * t2.r;
            ca.i = t0.i + tw1r * t1.i + tw2r * t2.i;
            cb.i = tw1i * t4.r + tw2i * t3.r;
            cb.r = -(tw1i * t4.i + tw2i * t3.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r + cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i + cb.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r - cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i - cb.i;
            let mut ca_0: Complex = Complex { r: 0., i: 0. };
            let mut cb_0: Complex = Complex { r: 0., i: 0. };
            ca_0.r = t0.r + tw2r * t1.r + tw1r * t2.r;
            ca_0.i = t0.i + tw2r * t1.i + tw1r * t2.i;
            cb_0.i = tw2i * t4.r - tw1i * t3.r;
            cb_0.r = -(tw2i * t4.i - tw1i * t3.i);
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_0.r + cb_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_0.i + cb_0.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = ca_0.r - cb_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = ca_0.i - cb_0.i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let t0_0: Complex = *cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            );
            let mut t1_0: Complex = Complex { r: 0., i: 0. };
            let mut t2_0: Complex = Complex { r: 0., i: 0. };
            let mut t3_0: Complex = Complex { r: 0., i: 0. };
            let mut t4_0: Complex = Complex { r: 0., i: 0. };
            t1_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t1_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t4_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t4_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t2_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t2_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t3_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t3_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t0_0.r + t1_0.r + t2_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t0_0.i + t1_0.i + t2_0.i;
            let mut ca_1: Complex = Complex { r: 0., i: 0. };
            let mut cb_1: Complex = Complex { r: 0., i: 0. };
            ca_1.r = t0_0.r + tw1r * t1_0.r + tw2r * t2_0.r;
            ca_1.i = t0_0.i + tw1r * t1_0.i + tw2r * t2_0.i;
            cb_1.i = tw1i * t4_0.r + tw2i * t3_0.r;
            cb_1.r = -(tw1i * t4_0.i + tw2i * t3_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca_1.r + cb_1.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca_1.i + cb_1.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .r = ca_1.r - cb_1.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .i = ca_1.i - cb_1.i;
            let mut ca_2: Complex = Complex { r: 0., i: 0. };
            let mut cb_2: Complex = Complex { r: 0., i: 0. };
            ca_2.r = t0_0.r + tw2r * t1_0.r + tw1r * t2_0.r;
            ca_2.i = t0_0.i + tw2r * t1_0.i + tw1r * t2_0.i;
            cb_2.i = tw2i * t4_0.r - tw1i * t3_0.r;
            cb_2.r = -(tw2i * t4_0.i - tw1i * t3_0.i);
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_2.r + cb_2.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_2.i + cb_2.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = ca_2.r - cb_2.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = ca_2.i - cb_2.i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let t0_1: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let mut t1_1: Complex = Complex { r: 0., i: 0. };
                let mut t2_1: Complex = Complex { r: 0., i: 0. };
                let mut t3_1: Complex = Complex { r: 0., i: 0. };
                let mut t4_1: Complex = Complex { r: 0., i: 0. };
                t1_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t1_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t4_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t4_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t2_1.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t2_1.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t3_1.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t3_1.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r = t0_1.r + t1_1.r + t2_1.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i = t0_1.i + t1_1.i + t2_1.i;
                let mut ca_3: Complex = Complex { r: 0., i: 0. };
                let mut cb_3: Complex = Complex { r: 0., i: 0. };
                let mut da: Complex = Complex { r: 0., i: 0. };
                let mut db: Complex = Complex { r: 0., i: 0. };
                ca_3.r = t0_1.r + tw1r * t1_1.r + tw2r * t2_1.r;
                ca_3.i = t0_1.i + tw1r * t1_1.i + tw2r * t2_1.i;
                cb_3.i = tw1i * t4_1.r + tw2i * t3_1.r;
                cb_3.r = -(tw1i * t4_1.i + tw2i * t3_1.i);
                da.r = ca_3.r + cb_3.r;
                da.i = ca_3.i + cb_3.i;
                db.r = ca_3.r - cb_3.r;
                db.i = ca_3.i - cb_3.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.r
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.i
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((4 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.r
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((4 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((4 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.i
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((4 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db.r;
                let mut ca_4: Complex = Complex { r: 0., i: 0. };
                let mut cb_4: Complex = Complex { r: 0., i: 0. };
                let mut da_0: Complex = Complex { r: 0., i: 0. };
                let mut db_0: Complex = Complex { r: 0., i: 0. };
                ca_4.r = t0_1.r + tw2r * t1_1.r + tw1r * t2_1.r;
                ca_4.i = t0_1.i + tw2r * t1_1.i + tw1r * t2_1.i;
                cb_4.i = tw2i * t4_1.r - tw1i * t3_1.r;
                cb_4.r = -(tw2i * t4_1.i - tw1i * t3_1.i);
                da_0.r = ca_4.r + cb_4.r;
                da_0.i = ca_4.i + cb_4.i;
                db_0.r = ca_4.r - cb_4.r;
                db_0.i = ca_4.i - cb_4.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_0.r
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da_0.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_0.i
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da_0.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((3 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_0.r
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((3 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db_0.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((3 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_0.i
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((3 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db_0.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn pass5f(ido: usize, l1: usize, cc: *const Complex, ch: *mut Complex, wa: *const Complex) {
    let cdim: usize = 5 as libc::c_int as usize;
    let tw1r: f64 = 0.3090169943749474241f64;
    let tw1i: f64 = -0.95105651629515357212f64;
    let tw2r: f64 = -0.8090169943749474241f64;
    let tw2i: f64 = -0.58778525229247312917f64;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            let t0: Complex = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
            let mut t1: Complex = Complex { r: 0., i: 0. };
            let mut t2: Complex = Complex { r: 0., i: 0. };
            let mut t3: Complex = Complex { r: 0., i: 0. };
            let mut t4: Complex = Complex { r: 0., i: 0. };
            t1.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t1.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t4.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t4.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t2.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t2.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t3.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t3.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t0.r + t1.r + t2.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t0.i + t1.i + t2.i;
            let mut ca: Complex = Complex { r: 0., i: 0. };
            let mut cb: Complex = Complex { r: 0., i: 0. };
            ca.r = t0.r + tw1r * t1.r + tw2r * t2.r;
            ca.i = t0.i + tw1r * t1.i + tw2r * t2.i;
            cb.i = tw1i * t4.r + tw2i * t3.r;
            cb.r = -(tw1i * t4.i + tw2i * t3.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r + cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i + cb.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r - cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i - cb.i;
            let mut ca_0: Complex = Complex { r: 0., i: 0. };
            let mut cb_0: Complex = Complex { r: 0., i: 0. };
            ca_0.r = t0.r + tw2r * t1.r + tw1r * t2.r;
            ca_0.i = t0.i + tw2r * t1.i + tw1r * t2.i;
            cb_0.i = tw2i * t4.r - tw1i * t3.r;
            cb_0.r = -(tw2i * t4.i - tw1i * t3.i);
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_0.r + cb_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_0.i + cb_0.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = ca_0.r - cb_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = ca_0.i - cb_0.i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let t0_0: Complex = *cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            );
            let mut t1_0: Complex = Complex { r: 0., i: 0. };
            let mut t2_0: Complex = Complex { r: 0., i: 0. };
            let mut t3_0: Complex = Complex { r: 0., i: 0. };
            let mut t4_0: Complex = Complex { r: 0., i: 0. };
            t1_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t1_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t4_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t4_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t2_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t2_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t3_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t3_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t0_0.r + t1_0.r + t2_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t0_0.i + t1_0.i + t2_0.i;
            let mut ca_1: Complex = Complex { r: 0., i: 0. };
            let mut cb_1: Complex = Complex { r: 0., i: 0. };
            ca_1.r = t0_0.r + tw1r * t1_0.r + tw2r * t2_0.r;
            ca_1.i = t0_0.i + tw1r * t1_0.i + tw2r * t2_0.i;
            cb_1.i = tw1i * t4_0.r + tw2i * t3_0.r;
            cb_1.r = -(tw1i * t4_0.i + tw2i * t3_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca_1.r + cb_1.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca_1.i + cb_1.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .r = ca_1.r - cb_1.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .i = ca_1.i - cb_1.i;
            let mut ca_2: Complex = Complex { r: 0., i: 0. };
            let mut cb_2: Complex = Complex { r: 0., i: 0. };
            ca_2.r = t0_0.r + tw2r * t1_0.r + tw1r * t2_0.r;
            ca_2.i = t0_0.i + tw2r * t1_0.i + tw1r * t2_0.i;
            cb_2.i = tw2i * t4_0.r - tw1i * t3_0.r;
            cb_2.r = -(tw2i * t4_0.i - tw1i * t3_0.i);
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_2.r + cb_2.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_2.i + cb_2.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = ca_2.r - cb_2.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = ca_2.i - cb_2.i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let t0_1: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let mut t1_1: Complex = Complex { r: 0., i: 0. };
                let mut t2_1: Complex = Complex { r: 0., i: 0. };
                let mut t3_1: Complex = Complex { r: 0., i: 0. };
                let mut t4_1: Complex = Complex { r: 0., i: 0. };
                t1_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t1_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t4_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t4_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t2_1.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t2_1.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t3_1.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t3_1.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r = t0_1.r + t1_1.r + t2_1.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i = t0_1.i + t1_1.i + t2_1.i;
                let mut ca_3: Complex = Complex { r: 0., i: 0. };
                let mut cb_3: Complex = Complex { r: 0., i: 0. };
                let mut da: Complex = Complex { r: 0., i: 0. };
                let mut db: Complex = Complex { r: 0., i: 0. };
                ca_3.r = t0_1.r + tw1r * t1_1.r + tw2r * t2_1.r;
                ca_3.i = t0_1.i + tw1r * t1_1.i + tw2r * t2_1.i;
                cb_3.i = tw1i * t4_1.r + tw2i * t3_1.r;
                cb_3.r = -(tw1i * t4_1.i + tw2i * t3_1.i);
                da.r = ca_3.r + cb_3.r;
                da.i = ca_3.i + cb_3.i;
                db.r = ca_3.r - cb_3.r;
                db.i = ca_3.i - cb_3.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.r
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.i
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((4 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.r
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((4 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((4 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.i
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((4 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db.r;
                let mut ca_4: Complex = Complex { r: 0., i: 0. };
                let mut cb_4: Complex = Complex { r: 0., i: 0. };
                let mut da_0: Complex = Complex { r: 0., i: 0. };
                let mut db_0: Complex = Complex { r: 0., i: 0. };
                ca_4.r = t0_1.r + tw2r * t1_1.r + tw1r * t2_1.r;
                ca_4.i = t0_1.i + tw2r * t1_1.i + tw1r * t2_1.i;
                cb_4.i = tw2i * t4_1.r - tw1i * t3_1.r;
                cb_4.r = -(tw2i * t4_1.i - tw1i * t3_1.i);
                da_0.r = ca_4.r + cb_4.r;
                da_0.i = ca_4.i + cb_4.i;
                db_0.r = ca_4.r - cb_4.r;
                db_0.i = ca_4.i - cb_4.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_0.r
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da_0.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_0.i
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * da_0.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((3 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_0.r
                    + (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((3 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db_0.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((3 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_0.i
                    - (*wa.offset(
                        i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                            ((3 as libc::c_int - 1 as libc::c_int) as usize)
                                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize,
                    ))
                    .i * db_0.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn pass7(
    ido: usize,
    l1: usize,
    cc: *const Complex,
    ch: *mut Complex,
    wa: *const Complex,
    sign: libc::c_int,
) {
    let cdim: usize = 7 as libc::c_int as usize;
    let tw1r: f64 = 0.623489801858733530525f64;
    let tw1i: f64 = sign as f64 * 0.7818314824680298087084f64;
    let tw2r: f64 = -0.222520933956314404289f64;
    let tw2i: f64 = sign as f64 * 0.9749279121818236070181f64;
    let tw3r: f64 = -0.9009688679024191262361f64;
    let tw3i: f64 = sign as f64 * 0.4338837391175581204758f64;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            let t1: Complex = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
            let mut t2: Complex = Complex { r: 0., i: 0. };
            let mut t3: Complex = Complex { r: 0., i: 0. };
            let mut t4: Complex = Complex { r: 0., i: 0. };
            let mut t5: Complex = Complex { r: 0., i: 0. };
            let mut t6: Complex = Complex { r: 0., i: 0. };
            let mut t7: Complex = Complex { r: 0., i: 0. };
            t2.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t2.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t7.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t7.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t3.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t3.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t6.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t6.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t4.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t4.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t5.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t5.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t1.r + t2.r + t3.r + t4.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t1.i + t2.i + t3.i + t4.i;
            let mut ca: Complex = Complex { r: 0., i: 0. };
            let mut cb: Complex = Complex { r: 0., i: 0. };
            ca.r = t1.r + tw1r * t2.r + tw2r * t3.r + tw3r * t4.r;
            ca.i = t1.i + tw1r * t2.i + tw2r * t3.i + tw3r * t4.i;
            cb.i = tw1i * t7.r + tw2i * t6.r + tw3i * t5.r;
            cb.r = -(tw1i * t7.i + tw2i * t6.i + tw3i * t5.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r + cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i + cb.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r - cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i - cb.i;
            let mut ca_0: Complex = Complex { r: 0., i: 0. };
            let mut cb_0: Complex = Complex { r: 0., i: 0. };
            ca_0.r = t1.r + tw2r * t2.r + tw3r * t3.r + tw1r * t4.r;
            ca_0.i = t1.i + tw2r * t2.i + tw3r * t3.i + tw1r * t4.i;
            cb_0.i = tw2i * t7.r - tw3i * t6.r - tw1i * t5.r;
            cb_0.r = -(tw2i * t7.i - tw3i * t6.i - tw1i * t5.i);
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_0.r + cb_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_0.i + cb_0.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
            ) as isize))
            .r = ca_0.r - cb_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
            ) as isize))
            .i = ca_0.i - cb_0.i;
            let mut ca_1: Complex = Complex { r: 0., i: 0. };
            let mut cb_1: Complex = Complex { r: 0., i: 0. };
            ca_1.r = t1.r + tw3r * t2.r + tw1r * t3.r + tw2r * t4.r;
            ca_1.i = t1.i + tw3r * t2.i + tw1r * t3.i + tw2r * t4.i;
            cb_1.i = tw3i * t7.r - tw1i * t6.r + tw2i * t5.r;
            cb_1.r = -(tw3i * t7.i - tw1i * t6.i + tw2i * t5.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = ca_1.r + cb_1.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = ca_1.i + cb_1.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .r = ca_1.r - cb_1.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .i = ca_1.i - cb_1.i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let t1_0: Complex = *cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            );
            let mut t2_0: Complex = Complex { r: 0., i: 0. };
            let mut t3_0: Complex = Complex { r: 0., i: 0. };
            let mut t4_0: Complex = Complex { r: 0., i: 0. };
            let mut t5_0: Complex = Complex { r: 0., i: 0. };
            let mut t6_0: Complex = Complex { r: 0., i: 0. };
            let mut t7_0: Complex = Complex { r: 0., i: 0. };
            t2_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t2_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t7_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t7_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t3_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t3_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t6_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t6_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t4_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t4_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t5_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t5_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t1_0.r + t2_0.r + t3_0.r + t4_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t1_0.i + t2_0.i + t3_0.i + t4_0.i;
            let mut ca_2: Complex = Complex { r: 0., i: 0. };
            let mut cb_2: Complex = Complex { r: 0., i: 0. };
            ca_2.r = t1_0.r + tw1r * t2_0.r + tw2r * t3_0.r + tw3r * t4_0.r;
            ca_2.i = t1_0.i + tw1r * t2_0.i + tw2r * t3_0.i + tw3r * t4_0.i;
            cb_2.i = tw1i * t7_0.r + tw2i * t6_0.r + tw3i * t5_0.r;
            cb_2.r = -(tw1i * t7_0.i + tw2i * t6_0.i + tw3i * t5_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca_2.r + cb_2.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca_2.i + cb_2.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
            ) as isize))
            .r = ca_2.r - cb_2.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
            ) as isize))
            .i = ca_2.i - cb_2.i;
            let mut ca_3: Complex = Complex { r: 0., i: 0. };
            let mut cb_3: Complex = Complex { r: 0., i: 0. };
            ca_3.r = t1_0.r + tw2r * t2_0.r + tw3r * t3_0.r + tw1r * t4_0.r;
            ca_3.i = t1_0.i + tw2r * t2_0.i + tw3r * t3_0.i + tw1r * t4_0.i;
            cb_3.i = tw2i * t7_0.r - tw3i * t6_0.r - tw1i * t5_0.r;
            cb_3.r = -(tw2i * t7_0.i - tw3i * t6_0.i - tw1i * t5_0.i);
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_3.r + cb_3.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_3.i + cb_3.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
            ) as isize))
            .r = ca_3.r - cb_3.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
            ) as isize))
            .i = ca_3.i - cb_3.i;
            let mut ca_4: Complex = Complex { r: 0., i: 0. };
            let mut cb_4: Complex = Complex { r: 0., i: 0. };
            ca_4.r = t1_0.r + tw3r * t2_0.r + tw1r * t3_0.r + tw2r * t4_0.r;
            ca_4.i = t1_0.i + tw3r * t2_0.i + tw1r * t3_0.i + tw2r * t4_0.i;
            cb_4.i = tw3i * t7_0.r - tw1i * t6_0.r + tw2i * t5_0.r;
            cb_4.r = -(tw3i * t7_0.i - tw1i * t6_0.i + tw2i * t5_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = ca_4.r + cb_4.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = ca_4.i + cb_4.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .r = ca_4.r - cb_4.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .i = ca_4.i - cb_4.i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let t1_1: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let mut t2_1: Complex = Complex { r: 0., i: 0. };
                let mut t3_1: Complex = Complex { r: 0., i: 0. };
                let mut t4_1: Complex = Complex { r: 0., i: 0. };
                let mut t5_1: Complex = Complex { r: 0., i: 0. };
                let mut t6_1: Complex = Complex { r: 0., i: 0. };
                let mut t7_1: Complex = Complex { r: 0., i: 0. };
                t2_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t2_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t7_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t7_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t3_1.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t3_1.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t6_1.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t6_1.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t4_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t4_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t5_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t5_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r = t1_1.r + t2_1.r + t3_1.r + t4_1.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i = t1_1.i + t2_1.i + t3_1.i + t4_1.i;
                let mut da: Complex = Complex { r: 0., i: 0. };
                let mut db: Complex = Complex { r: 0., i: 0. };
                let mut ca_5: Complex = Complex { r: 0., i: 0. };
                let mut cb_5: Complex = Complex { r: 0., i: 0. };
                ca_5.r = t1_1.r + tw1r * t2_1.r + tw2r * t3_1.r + tw3r * t4_1.r;
                ca_5.i = t1_1.i + tw1r * t2_1.i + tw2r * t3_1.i + tw3r * t4_1.i;
                cb_5.i = tw1i * t7_1.r + tw2i * t6_1.r + tw3i * t5_1.r;
                cb_5.r = -(tw1i * t7_1.i + tw2i * t6_1.i + tw3i * t5_1.i);
                da.r = ca_5.r + cb_5.r;
                da.i = ca_5.i + cb_5.i;
                db.r = ca_5.r - cb_5.r;
                db.i = ca_5.i - cb_5.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((6 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((6 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((6 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((6 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db.r;
                let mut da_0: Complex = Complex { r: 0., i: 0. };
                let mut db_0: Complex = Complex { r: 0., i: 0. };
                let mut ca_6: Complex = Complex { r: 0., i: 0. };
                let mut cb_6: Complex = Complex { r: 0., i: 0. };
                ca_6.r = t1_1.r + tw2r * t2_1.r + tw3r * t3_1.r + tw1r * t4_1.r;
                ca_6.i = t1_1.i + tw2r * t2_1.i + tw3r * t3_1.i + tw1r * t4_1.i;
                cb_6.i = tw2i * t7_1.r - tw3i * t6_1.r - tw1i * t5_1.r;
                cb_6.r = -(tw2i * t7_1.i - tw3i * t6_1.i - tw1i * t5_1.i);
                da_0.r = ca_6.r + cb_6.r;
                da_0.i = ca_6.i + cb_6.i;
                db_0.r = ca_6.r - cb_6.r;
                db_0.i = ca_6.i - cb_6.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_0.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_0.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_0.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_0.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((5 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_0.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((5 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_0.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((5 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_0.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((5 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_0.r;
                let mut da_1: Complex = Complex { r: 0., i: 0. };
                let mut db_1: Complex = Complex { r: 0., i: 0. };
                let mut ca_7: Complex = Complex { r: 0., i: 0. };
                let mut cb_7: Complex = Complex { r: 0., i: 0. };
                ca_7.r = t1_1.r + tw3r * t2_1.r + tw1r * t3_1.r + tw2r * t4_1.r;
                ca_7.i = t1_1.i + tw3r * t2_1.i + tw1r * t3_1.i + tw2r * t4_1.i;
                cb_7.i = tw3i * t7_1.r - tw1i * t6_1.r + tw2i * t5_1.r;
                cb_7.r = -(tw3i * t7_1.i - tw1i * t6_1.i + tw2i * t5_1.i);
                da_1.r = ca_7.r + cb_7.r;
                da_1.i = ca_7.i + cb_7.i;
                db_1.r = ca_7.r - cb_7.r;
                db_1.i = ca_7.i - cb_7.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((3 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_1.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((3 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_1.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((3 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_1.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((3 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_1.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((4 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_1.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((4 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_1.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((4 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_1.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((4 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_1.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn pass11(
    ido: usize,
    l1: usize,
    cc: *const Complex,
    ch: *mut Complex,
    wa: *const Complex,
    sign: libc::c_int,
) {
    let cdim: usize = 11 as libc::c_int as usize;
    let tw1r: f64 = 0.8412535328311811688618f64;
    let tw1i: f64 = sign as f64 * 0.5406408174555975821076f64;
    let tw2r: f64 = 0.4154150130018864255293f64;
    let tw2i: f64 = sign as f64 * 0.9096319953545183714117f64;
    let tw3r: f64 = -0.1423148382732851404438f64;
    let tw3i: f64 = sign as f64 * 0.9898214418809327323761f64;
    let tw4r: f64 = -0.6548607339452850640569f64;
    let tw4i: f64 = sign as f64 * 0.755749574354258283774f64;
    let tw5r: f64 = -0.9594929736144973898904f64;
    let tw5i: f64 = sign as f64 * 0.2817325568414296977114f64;
    if ido == 1 as libc::c_int as usize {
        let mut k: usize = 0;
        while k < l1 {
            let t1: Complex = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
            let mut t2: Complex = Complex { r: 0., i: 0. };
            let mut t3: Complex = Complex { r: 0., i: 0. };
            let mut t4: Complex = Complex { r: 0., i: 0. };
            let mut t5: Complex = Complex { r: 0., i: 0. };
            let mut t6: Complex = Complex { r: 0., i: 0. };
            let mut t7: Complex = Complex { r: 0., i: 0. };
            let mut t8: Complex = Complex { r: 0., i: 0. };
            let mut t9: Complex = Complex { r: 0., i: 0. };
            let mut t10: Complex = Complex { r: 0., i: 0. };
            let mut t11: Complex = Complex { r: 0., i: 0. };
            t2.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t2.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t11.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t11.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t3.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t3.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t10.r = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t10.i = (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t4.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t4.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t9.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t9.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t5.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t5.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t8.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t8.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t6.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t6.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            t7.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .r;
            t7.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t1.r + t2.r + t3.r + t4.r + t5.r + t6.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t1.i + t2.i + t3.i + t4.i + t5.i + t6.i;
            let mut ca: Complex = Complex { r: 0., i: 0. };
            let mut cb: Complex = Complex { r: 0., i: 0. };
            ca.r = t1.r + tw1r * t2.r + tw2r * t3.r + tw3r * t4.r + tw4r * t5.r + tw5r * t6.r;
            ca.i = t1.i + tw1r * t2.i + tw2r * t3.i + tw3r * t4.i + tw4r * t5.i + tw5r * t6.i;
            cb.i = tw1i * t11.r + tw2i * t10.r + tw3i * t9.r + tw4i * t8.r + tw5i * t7.r;
            cb.r = -(tw1i * t11.i + tw2i * t10.i + tw3i * t9.i + tw4i * t8.i + tw5i * t7.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r + cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i + cb.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(10 as libc::c_int as usize))),
            ) as isize))
            .r = ca.r - cb.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(10 as libc::c_int as usize))),
            ) as isize))
            .i = ca.i - cb.i;
            let mut ca_0: Complex = Complex { r: 0., i: 0. };
            let mut cb_0: Complex = Complex { r: 0., i: 0. };
            ca_0.r = t1.r + tw2r * t2.r + tw4r * t3.r + tw5r * t4.r + tw3r * t5.r + tw1r * t6.r;
            ca_0.i = t1.i + tw2r * t2.i + tw4r * t3.i + tw5r * t4.i + tw3r * t5.i + tw1r * t6.i;
            cb_0.i = tw2i * t11.r + tw4i * t10.r - tw5i * t9.r - tw3i * t8.r - tw1i * t7.r;
            cb_0.r = -(tw2i * t11.i + tw4i * t10.i - tw5i * t9.i - tw3i * t8.i - tw1i * t7.i);
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_0.r + cb_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_0.i + cb_0.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(9 as libc::c_int as usize))),
            ) as isize))
            .r = ca_0.r - cb_0.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(9 as libc::c_int as usize))),
            ) as isize))
            .i = ca_0.i - cb_0.i;
            let mut ca_1: Complex = Complex { r: 0., i: 0. };
            let mut cb_1: Complex = Complex { r: 0., i: 0. };
            ca_1.r = t1.r + tw3r * t2.r + tw5r * t3.r + tw2r * t4.r + tw1r * t5.r + tw4r * t6.r;
            ca_1.i = t1.i + tw3r * t2.i + tw5r * t3.i + tw2r * t4.i + tw1r * t5.i + tw4r * t6.i;
            cb_1.i = tw3i * t11.r - tw5i * t10.r - tw2i * t9.r + tw1i * t8.r + tw4i * t7.r;
            cb_1.r = -(tw3i * t11.i - tw5i * t10.i - tw2i * t9.i + tw1i * t8.i + tw4i * t7.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = ca_1.r + cb_1.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = ca_1.i + cb_1.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(8 as libc::c_int as usize))),
            ) as isize))
            .r = ca_1.r - cb_1.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(8 as libc::c_int as usize))),
            ) as isize))
            .i = ca_1.i - cb_1.i;
            let mut ca_2: Complex = Complex { r: 0., i: 0. };
            let mut cb_2: Complex = Complex { r: 0., i: 0. };
            ca_2.r = t1.r + tw4r * t2.r + tw3r * t3.r + tw1r * t4.r + tw5r * t5.r + tw2r * t6.r;
            ca_2.i = t1.i + tw4r * t2.i + tw3r * t3.i + tw1r * t4.i + tw5r * t5.i + tw2r * t6.i;
            cb_2.i = tw4i * t11.r - tw3i * t10.r + tw1i * t9.r + tw5i * t8.r - tw2i * t7.r;
            cb_2.r = -(tw4i * t11.i - tw3i * t10.i + tw1i * t9.i + tw5i * t8.i - tw2i * t7.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .r = ca_2.r + cb_2.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .i = ca_2.i + cb_2.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(7 as libc::c_int as usize))),
            ) as isize))
            .r = ca_2.r - cb_2.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(7 as libc::c_int as usize))),
            ) as isize))
            .i = ca_2.i - cb_2.i;
            let mut ca_3: Complex = Complex { r: 0., i: 0. };
            let mut cb_3: Complex = Complex { r: 0., i: 0. };
            ca_3.r = t1.r + tw5r * t2.r + tw1r * t3.r + tw4r * t4.r + tw2r * t5.r + tw3r * t6.r;
            ca_3.i = t1.i + tw5r * t2.i + tw1r * t3.i + tw4r * t4.i + tw2r * t5.i + tw3r * t6.i;
            cb_3.i = tw5i * t11.r - tw1i * t10.r + tw4i * t9.r - tw2i * t8.r + tw3i * t7.r;
            cb_3.r = -(tw5i * t11.i - tw1i * t10.i + tw4i * t9.i - tw2i * t8.i + tw3i * t7.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
            ) as isize))
            .r = ca_3.r + cb_3.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
            ) as isize))
            .i = ca_3.i + cb_3.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
            ) as isize))
            .r = ca_3.r - cb_3.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
            ) as isize))
            .i = ca_3.i - cb_3.i;
            k = k.wrapping_add(1)
        }
    } else {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let t1_0: Complex = *cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            );
            let mut t2_0: Complex = Complex { r: 0., i: 0. };
            let mut t3_0: Complex = Complex { r: 0., i: 0. };
            let mut t4_0: Complex = Complex { r: 0., i: 0. };
            let mut t5_0: Complex = Complex { r: 0., i: 0. };
            let mut t6_0: Complex = Complex { r: 0., i: 0. };
            let mut t7_0: Complex = Complex { r: 0., i: 0. };
            let mut t8_0: Complex = Complex { r: 0., i: 0. };
            let mut t9_0: Complex = Complex { r: 0., i: 0. };
            let mut t10_0: Complex = Complex { r: 0., i: 0. };
            let mut t11_0: Complex = Complex { r: 0., i: 0. };
            t2_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t2_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t11_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t11_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t3_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t3_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t10_0.r = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t10_0.i = (*cc.offset(
                (0usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t4_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t4_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t9_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t9_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t5_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t5_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t8_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t8_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t6_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t6_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i + (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            t7_0.r = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .r;
            t7_0.i = (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i - (*cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize))
            .i;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .r = t1_0.r + t2_0.r + t3_0.r + t4_0.r + t5_0.r + t6_0.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ))
            .i = t1_0.i + t2_0.i + t3_0.i + t4_0.i + t5_0.i + t6_0.i;
            let mut ca_4: Complex = Complex { r: 0., i: 0. };
            let mut cb_4: Complex = Complex { r: 0., i: 0. };
            ca_4.r = t1_0.r
                + tw1r * t2_0.r
                + tw2r * t3_0.r
                + tw3r * t4_0.r
                + tw4r * t5_0.r
                + tw5r * t6_0.r;
            ca_4.i = t1_0.i
                + tw1r * t2_0.i
                + tw2r * t3_0.i
                + tw3r * t4_0.i
                + tw4r * t5_0.i
                + tw5r * t6_0.i;
            cb_4.i =
                tw1i * t11_0.r + tw2i * t10_0.r + tw3i * t9_0.r + tw4i * t8_0.r + tw5i * t7_0.r;
            cb_4.r =
                -(tw1i * t11_0.i + tw2i * t10_0.i + tw3i * t9_0.i + tw4i * t8_0.i + tw5i * t7_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .r = ca_4.r + cb_4.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize))
            .i = ca_4.i + cb_4.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(10 as libc::c_int as usize))),
            ) as isize))
            .r = ca_4.r - cb_4.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(10 as libc::c_int as usize))),
            ) as isize))
            .i = ca_4.i - cb_4.i;
            let mut ca_5: Complex = Complex { r: 0., i: 0. };
            let mut cb_5: Complex = Complex { r: 0., i: 0. };
            ca_5.r = t1_0.r
                + tw2r * t2_0.r
                + tw4r * t3_0.r
                + tw5r * t4_0.r
                + tw3r * t5_0.r
                + tw1r * t6_0.r;
            ca_5.i = t1_0.i
                + tw2r * t2_0.i
                + tw4r * t3_0.i
                + tw5r * t4_0.i
                + tw3r * t5_0.i
                + tw1r * t6_0.i;
            cb_5.i =
                tw2i * t11_0.r + tw4i * t10_0.r - tw5i * t9_0.r - tw3i * t8_0.r - tw1i * t7_0.r;
            cb_5.r =
                -(tw2i * t11_0.i + tw4i * t10_0.i - tw5i * t9_0.i - tw3i * t8_0.i - tw1i * t7_0.i);
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .r = ca_5.r + cb_5.r;
            (*ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ))
            .i = ca_5.i + cb_5.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(9 as libc::c_int as usize))),
            ) as isize))
            .r = ca_5.r - cb_5.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(9 as libc::c_int as usize))),
            ) as isize))
            .i = ca_5.i - cb_5.i;
            let mut ca_6: Complex = Complex { r: 0., i: 0. };
            let mut cb_6: Complex = Complex { r: 0., i: 0. };
            ca_6.r = t1_0.r
                + tw3r * t2_0.r
                + tw5r * t3_0.r
                + tw2r * t4_0.r
                + tw1r * t5_0.r
                + tw4r * t6_0.r;
            ca_6.i = t1_0.i
                + tw3r * t2_0.i
                + tw5r * t3_0.i
                + tw2r * t4_0.i
                + tw1r * t5_0.i
                + tw4r * t6_0.i;
            cb_6.i =
                tw3i * t11_0.r - tw5i * t10_0.r - tw2i * t9_0.r + tw1i * t8_0.r + tw4i * t7_0.r;
            cb_6.r =
                -(tw3i * t11_0.i - tw5i * t10_0.i - tw2i * t9_0.i + tw1i * t8_0.i + tw4i * t7_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .r = ca_6.r + cb_6.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize))
            .i = ca_6.i + cb_6.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(8 as libc::c_int as usize))),
            ) as isize))
            .r = ca_6.r - cb_6.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(8 as libc::c_int as usize))),
            ) as isize))
            .i = ca_6.i - cb_6.i;
            let mut ca_7: Complex = Complex { r: 0., i: 0. };
            let mut cb_7: Complex = Complex { r: 0., i: 0. };
            ca_7.r = t1_0.r
                + tw4r * t2_0.r
                + tw3r * t3_0.r
                + tw1r * t4_0.r
                + tw5r * t5_0.r
                + tw2r * t6_0.r;
            ca_7.i = t1_0.i
                + tw4r * t2_0.i
                + tw3r * t3_0.i
                + tw1r * t4_0.i
                + tw5r * t5_0.i
                + tw2r * t6_0.i;
            cb_7.i =
                tw4i * t11_0.r - tw3i * t10_0.r + tw1i * t9_0.r + tw5i * t8_0.r - tw2i * t7_0.r;
            cb_7.r =
                -(tw4i * t11_0.i - tw3i * t10_0.i + tw1i * t9_0.i + tw5i * t8_0.i - tw2i * t7_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .r = ca_7.r + cb_7.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize))
            .i = ca_7.i + cb_7.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(7 as libc::c_int as usize))),
            ) as isize))
            .r = ca_7.r - cb_7.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(7 as libc::c_int as usize))),
            ) as isize))
            .i = ca_7.i - cb_7.i;
            let mut ca_8: Complex = Complex { r: 0., i: 0. };
            let mut cb_8: Complex = Complex { r: 0., i: 0. };
            ca_8.r = t1_0.r
                + tw5r * t2_0.r
                + tw1r * t3_0.r
                + tw4r * t4_0.r
                + tw2r * t5_0.r
                + tw3r * t6_0.r;
            ca_8.i = t1_0.i
                + tw5r * t2_0.i
                + tw1r * t3_0.i
                + tw4r * t4_0.i
                + tw2r * t5_0.i
                + tw3r * t6_0.i;
            cb_8.i =
                tw5i * t11_0.r - tw1i * t10_0.r + tw4i * t9_0.r - tw2i * t8_0.r + tw3i * t7_0.r;
            cb_8.r =
                -(tw5i * t11_0.i - tw1i * t10_0.i + tw4i * t9_0.i - tw2i * t8_0.i + tw3i * t7_0.i);
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
            ) as isize))
            .r = ca_8.r + cb_8.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
            ) as isize))
            .i = ca_8.i + cb_8.i;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
            ) as isize))
            .r = ca_8.r - cb_8.r;
            (*ch.offset((0usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
            ) as isize))
            .i = ca_8.i - cb_8.i;
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                let t1_1: Complex = *cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
                let mut t2_1: Complex = Complex { r: 0., i: 0. };
                let mut t3_1: Complex = Complex { r: 0., i: 0. };
                let mut t4_1: Complex = Complex { r: 0., i: 0. };
                let mut t5_1: Complex = Complex { r: 0., i: 0. };
                let mut t6_1: Complex = Complex { r: 0., i: 0. };
                let mut t7_1: Complex = Complex { r: 0., i: 0. };
                let mut t8_1: Complex = Complex { r: 0., i: 0. };
                let mut t9_1: Complex = Complex { r: 0., i: 0. };
                let mut t10_1: Complex = Complex { r: 0., i: 0. };
                let mut t11_1: Complex = Complex { r: 0., i: 0. };
                t2_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t2_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t11_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t11_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (10 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t3_1.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t3_1.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t10_1.r =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t10_1.i =
                    (*cc.offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (9 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t4_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t4_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t9_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t9_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (8 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t5_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t5_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t8_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t8_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (7 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t6_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t6_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i + (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                t7_1.r =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .r;
                t7_1.i =
                    (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (5 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i - (*cc.offset(i.wrapping_add(ido.wrapping_mul(
                        (6 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize))
                    .i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .r = t1_1.r + t2_1.r + t3_1.r + t4_1.r + t5_1.r + t6_1.r;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
                ))
                .i = t1_1.i + t2_1.i + t3_1.i + t4_1.i + t5_1.i + t6_1.i;
                let mut da: Complex = Complex { r: 0., i: 0. };
                let mut db: Complex = Complex { r: 0., i: 0. };
                let mut ca_9: Complex = Complex { r: 0., i: 0. };
                let mut cb_9: Complex = Complex { r: 0., i: 0. };
                ca_9.r = t1_1.r
                    + tw1r * t2_1.r
                    + tw2r * t3_1.r
                    + tw3r * t4_1.r
                    + tw4r * t5_1.r
                    + tw5r * t6_1.r;
                ca_9.i = t1_1.i
                    + tw1r * t2_1.i
                    + tw2r * t3_1.i
                    + tw3r * t4_1.i
                    + tw4r * t5_1.i
                    + tw5r * t6_1.i;
                cb_9.i =
                    tw1i * t11_1.r + tw2i * t10_1.r + tw3i * t9_1.r + tw4i * t8_1.r + tw5i * t7_1.r;
                cb_9.r = -(tw1i * t11_1.i
                    + tw2i * t10_1.i
                    + tw3i * t9_1.i
                    + tw4i * t8_1.i
                    + tw5i * t7_1.i);
                da.r = ca_9.r + cb_9.r;
                da.i = ca_9.i + cb_9.i;
                db.r = ca_9.r - cb_9.r;
                db.i = ca_9.i - cb_9.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((1 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((1 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(10 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((10 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((10 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(10 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((10 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((10 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db.r;
                let mut da_0: Complex = Complex { r: 0., i: 0. };
                let mut db_0: Complex = Complex { r: 0., i: 0. };
                let mut ca_10: Complex = Complex { r: 0., i: 0. };
                let mut cb_10: Complex = Complex { r: 0., i: 0. };
                ca_10.r = t1_1.r
                    + tw2r * t2_1.r
                    + tw4r * t3_1.r
                    + tw5r * t4_1.r
                    + tw3r * t5_1.r
                    + tw1r * t6_1.r;
                ca_10.i = t1_1.i
                    + tw2r * t2_1.i
                    + tw4r * t3_1.i
                    + tw5r * t4_1.i
                    + tw3r * t5_1.i
                    + tw1r * t6_1.i;
                cb_10.i =
                    tw2i * t11_1.r + tw4i * t10_1.r - tw5i * t9_1.r - tw3i * t8_1.r - tw1i * t7_1.r;
                cb_10.r = -(tw2i * t11_1.i + tw4i * t10_1.i
                    - tw5i * t9_1.i
                    - tw3i * t8_1.i
                    - tw1i * t7_1.i);
                da_0.r = ca_10.r + cb_10.r;
                da_0.i = ca_10.i + cb_10.i;
                db_0.r = ca_10.r - cb_10.r;
                db_0.i = ca_10.i - cb_10.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_0.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_0.i;
                (*ch.offset(
                    i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                ))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((2 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_0.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((2 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_0.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(9 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((9 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_0.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((9 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_0.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(9 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((9 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_0.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((9 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_0.r;
                let mut da_1: Complex = Complex { r: 0., i: 0. };
                let mut db_1: Complex = Complex { r: 0., i: 0. };
                let mut ca_11: Complex = Complex { r: 0., i: 0. };
                let mut cb_11: Complex = Complex { r: 0., i: 0. };
                ca_11.r = t1_1.r
                    + tw3r * t2_1.r
                    + tw5r * t3_1.r
                    + tw2r * t4_1.r
                    + tw1r * t5_1.r
                    + tw4r * t6_1.r;
                ca_11.i = t1_1.i
                    + tw3r * t2_1.i
                    + tw5r * t3_1.i
                    + tw2r * t4_1.i
                    + tw1r * t5_1.i
                    + tw4r * t6_1.i;
                cb_11.i =
                    tw3i * t11_1.r - tw5i * t10_1.r - tw2i * t9_1.r + tw1i * t8_1.r + tw4i * t7_1.r;
                cb_11.r = -(tw3i * t11_1.i - tw5i * t10_1.i - tw2i * t9_1.i
                    + tw1i * t8_1.i
                    + tw4i * t7_1.i);
                da_1.r = ca_11.r + cb_11.r;
                da_1.i = ca_11.i + cb_11.i;
                db_1.r = ca_11.r - cb_11.r;
                db_1.i = ca_11.i - cb_11.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((3 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_1.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((3 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_1.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((3 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_1.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((3 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_1.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(8 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((8 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_1.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((8 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_1.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(8 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((8 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_1.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((8 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_1.r;
                let mut da_2: Complex = Complex { r: 0., i: 0. };
                let mut db_2: Complex = Complex { r: 0., i: 0. };
                let mut ca_12: Complex = Complex { r: 0., i: 0. };
                let mut cb_12: Complex = Complex { r: 0., i: 0. };
                ca_12.r = t1_1.r
                    + tw4r * t2_1.r
                    + tw3r * t3_1.r
                    + tw1r * t4_1.r
                    + tw5r * t5_1.r
                    + tw2r * t6_1.r;
                ca_12.i = t1_1.i
                    + tw4r * t2_1.i
                    + tw3r * t3_1.i
                    + tw1r * t4_1.i
                    + tw5r * t5_1.i
                    + tw2r * t6_1.i;
                cb_12.i =
                    tw4i * t11_1.r - tw3i * t10_1.r + tw1i * t9_1.r + tw5i * t8_1.r - tw2i * t7_1.r;
                cb_12.r = -(tw4i * t11_1.i - tw3i * t10_1.i + tw1i * t9_1.i + tw5i * t8_1.i
                    - tw2i * t7_1.i);
                da_2.r = ca_12.r + cb_12.r;
                da_2.i = ca_12.i + cb_12.i;
                db_2.r = ca_12.r - cb_12.r;
                db_2.i = ca_12.i - cb_12.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((4 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_2.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((4 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_2.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((4 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_2.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((4 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_2.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(7 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((7 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_2.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((7 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_2.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(7 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((7 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_2.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((7 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_2.r;
                let mut da_3: Complex = Complex { r: 0., i: 0. };
                let mut db_3: Complex = Complex { r: 0., i: 0. };
                let mut ca_13: Complex = Complex { r: 0., i: 0. };
                let mut cb_13: Complex = Complex { r: 0., i: 0. };
                ca_13.r = t1_1.r
                    + tw5r * t2_1.r
                    + tw1r * t3_1.r
                    + tw4r * t4_1.r
                    + tw2r * t5_1.r
                    + tw3r * t6_1.r;
                ca_13.i = t1_1.i
                    + tw5r * t2_1.i
                    + tw1r * t3_1.i
                    + tw4r * t4_1.i
                    + tw2r * t5_1.i
                    + tw3r * t6_1.i;
                cb_13.i =
                    tw5i * t11_1.r - tw1i * t10_1.r + tw4i * t9_1.r - tw2i * t8_1.r + tw3i * t7_1.r;
                cb_13.r = -(tw5i * t11_1.i - tw1i * t10_1.i + tw4i * t9_1.i - tw2i * t8_1.i
                    + tw3i * t7_1.i);
                da_3.r = ca_13.r + cb_13.r;
                da_3.i = ca_13.i + cb_13.i;
                db_3.r = ca_13.r - cb_13.r;
                db_3.i = ca_13.i - cb_13.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((5 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_3.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((5 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_3.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(5 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((5 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * da_3.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((5 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * da_3.r;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
                ) as isize))
                .r = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((6 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_3.r
                    - sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((6 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_3.i;
                (*ch.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(6 as libc::c_int as usize))),
                ) as isize))
                .i = (*wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ((6 as libc::c_int - 1 as libc::c_int) as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ))
                .r * db_3.i
                    + sign as f64
                        * (*wa.offset(
                            i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                                ((6 as libc::c_int - 1 as libc::c_int) as usize)
                                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                            ) as isize,
                        ))
                        .i
                        * db_3.r;
                i = i.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
    };
}
#[inline(never)]
unsafe fn passg(
    ido: usize,
    ip: usize,
    l1: usize,
    cc: *mut Complex,
    ch: *mut Complex,
    wa: *const Complex,
    csarr: *const Complex,
    sign: libc::c_int,
) -> libc::c_int {
    let cdim: usize = ip;
    let ipph: usize = ip
        .wrapping_add(1 as libc::c_int as usize)
        .wrapping_div(2usize);
    let idl1: usize = ido.wrapping_mul(l1);
    let wal: *mut Complex =
        libc::malloc(ip.wrapping_mul(::std::mem::size_of::<Complex>() as usize) as libc::size_t)
            as *mut Complex;
    if wal.is_null() {
        return -(1 as libc::c_int);
    }
    *wal.offset(0) = {
        let init = Complex {
            r: 1.0f64,
            i: 0.0f64,
        };
        init
    };
    let mut i: usize = 1 as libc::c_int as usize;
    while i < ip {
        *wal.offset(i as isize) = Complex {
            r: (*csarr.offset(i as isize)).r,
            i: sign as f64 * (*csarr.offset(i as isize)).i,
        };
        i = i.wrapping_add(1)
    }
    let mut k: usize = 0;
    while k < l1 {
        let mut i_0: usize = 0;
        while i_0 < ido {
            *ch.offset(
                i_0.wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) = *cc.offset(
                i_0.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
            i_0 = i_0.wrapping_add(1)
        }
        k = k.wrapping_add(1)
    }
    let mut j: usize = 1 as libc::c_int as usize;
    let mut jc: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while j < ipph {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let mut i_1: usize = 0;
            while i_1 < ido {
                (*ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(j))))
                        as isize,
                ))
                .r = (*cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(j.wrapping_add(cdim.wrapping_mul(k_0))))
                        as isize,
                ))
                .r + (*cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(jc.wrapping_add(cdim.wrapping_mul(k_0))))
                        as isize,
                ))
                .r;
                (*ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(j))))
                        as isize,
                ))
                .i = (*cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(j.wrapping_add(cdim.wrapping_mul(k_0))))
                        as isize,
                ))
                .i + (*cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(jc.wrapping_add(cdim.wrapping_mul(k_0))))
                        as isize,
                ))
                .i;
                (*ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(jc))))
                        as isize,
                ))
                .r = (*cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(j.wrapping_add(cdim.wrapping_mul(k_0))))
                        as isize,
                ))
                .r - (*cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(jc.wrapping_add(cdim.wrapping_mul(k_0))))
                        as isize,
                ))
                .r;
                (*ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(jc))))
                        as isize,
                ))
                .i = (*cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(j.wrapping_add(cdim.wrapping_mul(k_0))))
                        as isize,
                ))
                .i - (*cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(jc.wrapping_add(cdim.wrapping_mul(k_0))))
                        as isize,
                ))
                .i;
                i_1 = i_1.wrapping_add(1)
            }
            k_0 = k_0.wrapping_add(1)
        }
        j = j.wrapping_add(1);
        jc = jc.wrapping_sub(1)
    }
    let mut k_1: usize = 0;
    while k_1 < l1 {
        let mut i_2: usize = 0;
        while i_2 < ido {
            let mut tmp: Complex = *ch.offset(
                i_2.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0)))) as isize,
            );
            let mut j_0: usize = 1 as libc::c_int as usize;
            while j_0 < ipph {
                tmp.r = tmp.r
                    + (*ch.offset(
                        i_2.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(j_0))))
                            as isize,
                    ))
                    .r;
                tmp.i = tmp.i
                    + (*ch.offset(
                        i_2.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(j_0))))
                            as isize,
                    ))
                    .i;
                j_0 = j_0.wrapping_add(1)
            }
            *cc.offset(
                i_2.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) = tmp;
            i_2 = i_2.wrapping_add(1)
        }
        k_1 = k_1.wrapping_add(1)
    }
    let mut l: usize = 1 as libc::c_int as usize;
    let mut lc: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while l < ipph {
        // j=0
        let mut ik: usize = 0;
        while ik < idl1 {
            (*cc.offset(ik.wrapping_add(idl1.wrapping_mul(l)) as isize)).r =
                (*ch.offset(ik.wrapping_add(idl1.wrapping_mul(0)) as isize)).r
                    + (*wal.offset(l as isize)).r
                        * (*ch.offset(
                            ik.wrapping_add(idl1.wrapping_mul(1 as libc::c_int as usize)) as isize,
                        ))
                        .r
                    + (*wal.offset((2usize).wrapping_mul(l) as isize)).r
                        * (*ch.offset(ik.wrapping_add(idl1.wrapping_mul(2usize)) as isize)).r;
            (*cc.offset(ik.wrapping_add(idl1.wrapping_mul(l)) as isize)).i =
                (*ch.offset(ik.wrapping_add(idl1.wrapping_mul(0)) as isize)).i
                    + (*wal.offset(l as isize)).r
                        * (*ch.offset(
                            ik.wrapping_add(idl1.wrapping_mul(1 as libc::c_int as usize)) as isize,
                        ))
                        .i
                    + (*wal.offset((2usize).wrapping_mul(l) as isize)).r
                        * (*ch.offset(ik.wrapping_add(idl1.wrapping_mul(2usize)) as isize)).i;
            (*cc.offset(ik.wrapping_add(idl1.wrapping_mul(lc)) as isize)).r = -(*wal
                .offset(l as isize))
            .i * (*ch.offset(
                ik.wrapping_add(idl1.wrapping_mul(ip.wrapping_sub(1 as libc::c_int as usize)))
                    as isize,
            ))
            .i - (*wal
                .offset((2usize).wrapping_mul(l) as isize))
            .i * (*ch
                .offset(ik.wrapping_add(idl1.wrapping_mul(ip.wrapping_sub(2usize))) as isize))
            .i;
            (*cc.offset(ik.wrapping_add(idl1.wrapping_mul(lc)) as isize)).i = (*wal
                .offset(l as isize))
            .i * (*ch.offset(
                ik.wrapping_add(idl1.wrapping_mul(ip.wrapping_sub(1 as libc::c_int as usize)))
                    as isize,
            ))
            .r + (*wal
                .offset((2usize).wrapping_mul(l) as isize))
            .i * (*ch
                .offset(ik.wrapping_add(idl1.wrapping_mul(ip.wrapping_sub(2usize))) as isize))
            .r;
            ik = ik.wrapping_add(1)
        }
        let mut iwal: usize = (2usize).wrapping_mul(l);
        let mut j_1: usize = 3 as libc::c_int as usize;
        let mut jc_0: usize = ip.wrapping_sub(3 as libc::c_int as usize);
        while j_1 < ipph.wrapping_sub(1 as libc::c_int as usize) {
            iwal = (iwal as usize).wrapping_add(l) as usize as usize;
            if iwal > ip {
                iwal = (iwal as usize).wrapping_sub(ip) as usize as usize
            }
            let xwal: Complex = *wal.offset(iwal as isize);
            iwal = (iwal as usize).wrapping_add(l) as usize as usize;
            if iwal > ip {
                iwal = (iwal as usize).wrapping_sub(ip) as usize as usize
            }
            let xwal2: Complex = *wal.offset(iwal as isize);
            let mut ik_0: usize = 0;
            while ik_0 < idl1 {
                (*cc.offset(ik_0.wrapping_add(idl1.wrapping_mul(l)) as isize)).r +=
                    (*ch.offset(ik_0.wrapping_add(idl1.wrapping_mul(j_1)) as isize)).r * xwal.r
                        + (*ch.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(j_1.wrapping_add(1 as libc::c_int as usize)),
                        ) as isize))
                        .r * xwal2.r;
                (*cc.offset(ik_0.wrapping_add(idl1.wrapping_mul(l)) as isize)).i +=
                    (*ch.offset(ik_0.wrapping_add(idl1.wrapping_mul(j_1)) as isize)).i * xwal.r
                        + (*ch.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(j_1.wrapping_add(1 as libc::c_int as usize)),
                        ) as isize))
                        .i * xwal2.r;
                (*cc.offset(ik_0.wrapping_add(idl1.wrapping_mul(lc)) as isize)).r -=
                    (*ch.offset(ik_0.wrapping_add(idl1.wrapping_mul(jc_0)) as isize)).i * xwal.i
                        + (*ch.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(jc_0.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize))
                        .i * xwal2.i;
                (*cc.offset(ik_0.wrapping_add(idl1.wrapping_mul(lc)) as isize)).i +=
                    (*ch.offset(ik_0.wrapping_add(idl1.wrapping_mul(jc_0)) as isize)).r * xwal.i
                        + (*ch.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(jc_0.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize))
                        .r * xwal2.i;
                ik_0 = ik_0.wrapping_add(1)
            }
            j_1 = (j_1 as usize).wrapping_add(2usize) as usize as usize;
            jc_0 = (jc_0 as usize).wrapping_sub(2usize) as usize as usize
        }
        while j_1 < ipph {
            iwal = (iwal as usize).wrapping_add(l) as usize as usize;
            if iwal > ip {
                iwal = (iwal as usize).wrapping_sub(ip) as usize as usize
            }
            let xwal_0: Complex = *wal.offset(iwal as isize);
            let mut ik_1: usize = 0;
            while ik_1 < idl1 {
                (*cc.offset(ik_1.wrapping_add(idl1.wrapping_mul(l)) as isize)).r +=
                    (*ch.offset(ik_1.wrapping_add(idl1.wrapping_mul(j_1)) as isize)).r * xwal_0.r;
                (*cc.offset(ik_1.wrapping_add(idl1.wrapping_mul(l)) as isize)).i +=
                    (*ch.offset(ik_1.wrapping_add(idl1.wrapping_mul(j_1)) as isize)).i * xwal_0.r;
                (*cc.offset(ik_1.wrapping_add(idl1.wrapping_mul(lc)) as isize)).r -=
                    (*ch.offset(ik_1.wrapping_add(idl1.wrapping_mul(jc_0)) as isize)).i * xwal_0.i;
                (*cc.offset(ik_1.wrapping_add(idl1.wrapping_mul(lc)) as isize)).i +=
                    (*ch.offset(ik_1.wrapping_add(idl1.wrapping_mul(jc_0)) as isize)).r * xwal_0.i;
                ik_1 = ik_1.wrapping_add(1)
            }
            j_1 = j_1.wrapping_add(1);
            jc_0 = jc_0.wrapping_sub(1)
        }
        l = l.wrapping_add(1);
        lc = lc.wrapping_sub(1)
    }
    libc::free(wal as *mut libc::c_void);
    // shuffling and twiddling
    if ido == 1 as libc::c_int as usize {
        let mut j_2: usize = 1 as libc::c_int as usize;
        let mut jc_1: usize = ip.wrapping_sub(1 as libc::c_int as usize);
        while j_2 < ipph {
            let mut ik_2: usize = 0;
            while ik_2 < idl1 {
                let t1: Complex = *cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(j_2)) as isize);
                let t2: Complex = *cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(jc_1)) as isize);
                (*cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(j_2)) as isize)).r = t1.r + t2.r;
                (*cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(j_2)) as isize)).i = t1.i + t2.i;
                (*cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(jc_1)) as isize)).r = t1.r - t2.r;
                (*cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(jc_1)) as isize)).i = t1.i - t2.i;
                ik_2 = ik_2.wrapping_add(1)
            }
            j_2 = j_2.wrapping_add(1);
            jc_1 = jc_1.wrapping_sub(1)
        }
    } else {
        let mut j_3: usize = 1 as libc::c_int as usize;
        let mut jc_2: usize = ip.wrapping_sub(1 as libc::c_int as usize);
        while j_3 < ipph {
            let mut k_2: usize = 0;
            while k_2 < l1 {
                let t1_0: Complex = *cc.offset(
                    (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                        as isize,
                );
                let t2_0: Complex = *cc.offset(
                    (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                        as isize,
                );
                (*cc.offset(
                    (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                        as isize,
                ))
                .r = t1_0.r + t2_0.r;
                (*cc.offset(
                    (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                        as isize,
                ))
                .i = t1_0.i + t2_0.i;
                (*cc.offset(
                    (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                        as isize,
                ))
                .r = t1_0.r - t2_0.r;
                (*cc.offset(
                    (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                        as isize,
                ))
                .i = t1_0.i - t2_0.i;
                let mut i_3: usize = 1 as libc::c_int as usize;
                while i_3 < ido {
                    let mut x1: Complex = Complex { r: 0., i: 0. };
                    let mut x2: Complex = Complex { r: 0., i: 0. };
                    x1.r = (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                            as isize,
                    ))
                    .r + (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                            as isize,
                    ))
                    .r;
                    x1.i = (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                            as isize,
                    ))
                    .i + (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                            as isize,
                    ))
                    .i;
                    x2.r = (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                            as isize,
                    ))
                    .r - (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                            as isize,
                    ))
                    .r;
                    x2.i = (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                            as isize,
                    ))
                    .i - (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                            as isize,
                    ))
                    .i;
                    let mut idij: usize = j_3
                        .wrapping_sub(1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize))
                        .wrapping_add(i_3)
                        .wrapping_sub(1 as libc::c_int as usize);
                    (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                            as isize,
                    ))
                    .r = (*wa.offset(idij as isize)).r * x1.r
                        - sign as f64 * (*wa.offset(idij as isize)).i * x1.i;
                    (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                            as isize,
                    ))
                    .i = (*wa.offset(idij as isize)).r * x1.i
                        + sign as f64 * (*wa.offset(idij as isize)).i * x1.r;
                    idij = jc_2
                        .wrapping_sub(1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize))
                        .wrapping_add(i_3)
                        .wrapping_sub(1 as libc::c_int as usize);
                    (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                            as isize,
                    ))
                    .r = (*wa.offset(idij as isize)).r * x2.r
                        - sign as f64 * (*wa.offset(idij as isize)).i * x2.i;
                    (*cc.offset(
                        i_3.wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                            as isize,
                    ))
                    .i = (*wa.offset(idij as isize)).r * x2.i
                        + sign as f64 * (*wa.offset(idij as isize)).i * x2.r;
                    i_3 = i_3.wrapping_add(1)
                }
                k_2 = k_2.wrapping_add(1)
            }
            j_3 = j_3.wrapping_add(1);
            jc_2 = jc_2.wrapping_sub(1)
        }
    }
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe fn pass_all(plan: CFFTPPlan, c: *mut Complex, fct: f64, sign: libc::c_int) -> libc::c_int {
    if (*plan).length == 1 as libc::c_int as usize {
        return 0 as libc::c_int;
    }
    let len: usize = (*plan).length;
    let mut l1: usize = 1 as libc::c_int as usize;
    let nf: usize = (*plan).nfct;
    let ch: *mut Complex =
        libc::malloc(len.wrapping_mul(::std::mem::size_of::<Complex>() as usize) as libc::size_t)
            as *mut Complex;
    if ch.is_null() {
        return -(1 as libc::c_int);
    }
    let mut p1: *mut Complex = c;
    let mut p2: *mut Complex = ch;
    let mut k1: usize = 0;
    while k1 < nf {
        let ip: usize = (*plan).fct[k1 as usize].fct;
        let l2: usize = ip.wrapping_mul(l1);
        let ido: usize = len.wrapping_div(l2);
        if ip == 4 as libc::c_int as usize {
            if sign > 0 as libc::c_int {
                pass4b(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw);
            } else {
                pass4f(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw);
            };
        } else if ip == 2usize {
            if sign > 0 as libc::c_int {
                pass2b(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw);
            } else {
                pass2f(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw);
            };
        } else if ip == 3 as libc::c_int as usize {
            if sign > 0 as libc::c_int {
                pass3b(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw);
            } else {
                pass3f(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw);
            };
        } else if ip == 5 as libc::c_int as usize {
            if sign > 0 as libc::c_int {
                pass5b(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw);
            } else {
                pass5f(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw);
            };
        } else if ip == 7 as libc::c_int as usize {
            pass7(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw, sign);
        } else if ip == 11 as libc::c_int as usize {
            pass11(ido, l1, p1, p2, (*plan).fct[k1 as usize].tw, sign);
        } else {
            if passg(
                ido,
                ip,
                l1,
                p1,
                p2,
                (*plan).fct[k1 as usize].tw,
                (*plan).fct[k1 as usize].tws,
                sign,
            ) != 0
            {
                libc::free(ch as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            let tmp_: *mut Complex = p1;
            p1 = p2;
            p2 = tmp_
        }
        let tmp_0: *mut Complex = p1;
        p1 = p2;
        p2 = tmp_0;
        l1 = l2;
        k1 = k1.wrapping_add(1)
    }
    if p1 != c {
        if fct != 1.0f64 {
            let mut i: usize = 0;
            while i < len {
                (*c.offset(i as isize)).r = (*ch.offset(i as isize)).r * fct;
                (*c.offset(i as isize)).i = (*ch.offset(i as isize)).i * fct;
                i = i.wrapping_add(1)
            }
        } else {
            libc::memcpy(
                c as *mut libc::c_void,
                p1 as *const libc::c_void,
                len.wrapping_mul(::std::mem::size_of::<Complex>() as usize) as libc::size_t,
            );
        }
    } else if fct != 1.0f64 {
        let mut i_0: usize = 0;
        while i_0 < len {
            (*c.offset(i_0 as isize)).r *= fct;
            (*c.offset(i_0 as isize)).i *= fct;
            i_0 = i_0.wrapping_add(1)
        }
    }
    libc::free(ch as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe fn cfftp_forward(plan: CFFTPPlan, c: *mut f64, fct: f64) -> libc::c_int {
    return pass_all(plan, c as *mut Complex, fct, -(1 as libc::c_int));
}
#[inline(never)]
unsafe fn cfftp_backward(plan: CFFTPPlan, c: *mut f64, fct: f64) -> libc::c_int {
    return pass_all(plan, c as *mut Complex, fct, 1 as libc::c_int);
}
#[inline(never)]
unsafe fn cfftp_factorize(mut plan: CFFTPPlan) -> libc::c_int {
    let mut length: usize = (*plan).length;
    let mut nfct: usize = 0;
    while length.wrapping_rem(4 as libc::c_int as usize) == 0 {
        if nfct >= 25 as libc::c_int as usize {
            return -(1 as libc::c_int);
        }
        let fresh1 = nfct;
        nfct = nfct.wrapping_add(1);
        (*plan).fct[fresh1 as usize].fct = 4 as libc::c_int as usize;
        length >>= 2 as libc::c_int
    }
    if length.wrapping_rem(2usize) == 0 {
        length >>= 1 as libc::c_int;
        // factor 2 should be at the front of the factor list
        if nfct >= 25 as libc::c_int as usize {
            return -(1 as libc::c_int);
        }
        let fresh2 = nfct;
        nfct = nfct.wrapping_add(1);
        (*plan).fct[fresh2 as usize].fct = 2usize;
        let tmp_: usize = (*plan).fct[0].fct;
        (*plan).fct[0].fct = (*plan).fct[nfct.wrapping_sub(1 as libc::c_int as usize) as usize].fct;
        (*plan).fct[nfct.wrapping_sub(1 as libc::c_int as usize) as usize].fct = tmp_
    }
    let mut maxl: usize =
        (f64::sqrt(length as f64) as usize).wrapping_add(1 as libc::c_int as usize);
    let mut divisor: usize = 3 as libc::c_int as usize;
    while length > 1 as libc::c_int as usize && divisor < maxl {
        if length.wrapping_rem(divisor) == 0 {
            while length.wrapping_rem(divisor) == 0 {
                if nfct >= 25 as libc::c_int as usize {
                    return -(1 as libc::c_int);
                }
                let fresh3 = nfct;
                nfct = nfct.wrapping_add(1);
                (*plan).fct[fresh3 as usize].fct = divisor;
                length = (length as usize).wrapping_div(divisor) as usize as usize
            }
            maxl = (f64::sqrt(length as f64) as usize).wrapping_add(1 as libc::c_int as usize)
        }
        divisor = (divisor as usize).wrapping_add(2usize) as usize as usize
    }
    if length > 1 as libc::c_int as usize {
        let fresh4 = nfct;
        nfct = nfct.wrapping_add(1);
        (*plan).fct[fresh4 as usize].fct = length
    }
    (*plan).nfct = nfct;
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe fn cfftp_twsize(plan: CFFTPPlan) -> usize {
    let mut twsize: usize = 0;
    let mut l1: usize = 1 as libc::c_int as usize;
    let mut k: usize = 0;
    while k < (*plan).nfct {
        let ip: usize = (*plan).fct[k as usize].fct;
        let ido: usize = (*plan).length.wrapping_div(l1.wrapping_mul(ip));
        twsize = (twsize as usize).wrapping_add(
            ip.wrapping_sub(1 as libc::c_int as usize)
                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
        ) as usize as usize;
        if ip > 11 as libc::c_int as usize {
            twsize = (twsize as usize).wrapping_add(ip) as usize as usize
        }
        l1 = (l1 as usize).wrapping_mul(ip) as usize as usize;
        k = k.wrapping_add(1)
    }
    return twsize;
}
#[inline(never)]
unsafe fn cfftp_comp_twiddle(mut plan: CFFTPPlan) -> libc::c_int {
    let length = (*plan).length;
    let twid: *mut f64 =
        libc::malloc(2 * length as libc::size_t * std::mem::size_of::<f64>()) as *mut f64;
    if twid.is_null() {
        return -(1 as libc::c_int);
    }
    sincos_2pibyn(length, twid);
    let mut l1: usize = 1 as libc::c_int as usize;
    let mut memofs: usize = 0;
    let mut k: usize = 0;
    while k < (*plan).nfct {
        let ip: usize = (*plan).fct[k as usize].fct;
        let ido: usize = length.wrapping_div(l1.wrapping_mul(ip));
        (*plan).fct[k as usize].tw = (*plan).mem.offset(memofs as isize);
        memofs = (memofs as usize).wrapping_add(
            ip.wrapping_sub(1 as libc::c_int as usize)
                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
        ) as usize as usize;
        let mut j: usize = 1 as libc::c_int as usize;
        while j < ip {
            let mut i: usize = 1 as libc::c_int as usize;
            while i < ido {
                (*(*plan).fct[k as usize].tw.offset(
                    j.wrapping_sub(1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize))
                        .wrapping_add(i)
                        .wrapping_sub(1 as libc::c_int as usize) as isize,
                ))
                .r = *twid
                    .offset((2usize).wrapping_mul(j).wrapping_mul(l1).wrapping_mul(i) as isize);
                (*(*plan).fct[k as usize].tw.offset(
                    j.wrapping_sub(1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize))
                        .wrapping_add(i)
                        .wrapping_sub(1 as libc::c_int as usize) as isize,
                ))
                .i = *twid.offset(
                    (2usize)
                        .wrapping_mul(j)
                        .wrapping_mul(l1)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as usize) as isize,
                );
                i = i.wrapping_add(1)
            }
            j = j.wrapping_add(1)
        }
        if ip > 11 as libc::c_int as usize {
            (*plan).fct[k as usize].tws = (*plan).mem.offset(memofs as isize);
            memofs = (memofs as usize).wrapping_add(ip) as usize as usize;
            let mut j_0: usize = 0;
            while j_0 < ip {
                (*(*plan).fct[k as usize].tws.offset(j_0 as isize)).r = *twid.offset(
                    (2usize)
                        .wrapping_mul(j_0)
                        .wrapping_mul(l1)
                        .wrapping_mul(ido) as isize,
                );
                (*(*plan).fct[k as usize].tws.offset(j_0 as isize)).i = *twid.offset(
                    (2usize)
                        .wrapping_mul(j_0)
                        .wrapping_mul(l1)
                        .wrapping_mul(ido)
                        .wrapping_add(1 as libc::c_int as usize) as isize,
                );
                j_0 = j_0.wrapping_add(1)
            }
        }
        l1 = (l1 as usize).wrapping_mul(ip) as usize as usize;
        k = k.wrapping_add(1)
    }
    libc::free(twid as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe fn make_cfftp_plan(length: usize) -> CFFTPPlan {
    if length == 0 {
        return 0 as CFFTPPlan;
    }
    let mut plan: CFFTPPlan = libc::malloc(::std::mem::size_of::<CFFTPPlanI>()) as *mut CFFTPPlanI;
    if plan.is_null() {
        return 0 as CFFTPPlan;
    }
    (*plan).length = length;
    (*plan).nfct = 0;
    let mut i: usize = 0;
    while i < 25 as libc::c_int as usize {
        (*plan).fct[i as usize] = CFFTPFctData {
            fct: 0,
            tw: 0 as *mut Complex,
            tws: 0 as *mut Complex,
        };
        i = i.wrapping_add(1)
    }
    (*plan).mem = 0 as *mut Complex;
    if length == 1 as libc::c_int as usize {
        return plan;
    }
    if cfftp_factorize(plan) != 0 as libc::c_int {
        libc::free(plan as *mut libc::c_void);
        return 0 as CFFTPPlan;
    }
    let tws: usize = cfftp_twsize(plan);
    (*plan).mem =
        libc::malloc(tws.wrapping_mul(::std::mem::size_of::<Complex>() as usize) as libc::size_t)
            as *mut Complex;
    if (*plan).mem.is_null() {
        libc::free(plan as *mut libc::c_void);
        return 0 as CFFTPPlan;
    }
    if cfftp_comp_twiddle(plan) != 0 as libc::c_int {
        libc::free((*plan).mem as *mut libc::c_void);
        (*plan).mem = 0 as *mut Complex;
        libc::free(plan as *mut libc::c_void);
        return 0 as CFFTPPlan;
    }
    return plan;
}
unsafe fn destroy_cfftp_plan(plan: CFFTPPlan) {
    libc::free((*plan).mem as *mut libc::c_void);
    libc::free(plan as *mut libc::c_void);
}
#[inline(never)]
unsafe fn radf2(ido: usize, l1: usize, cc: *const f64, ch: *mut f64, wa: *const f64) {
    let cdim: usize = 2usize;
    let mut k: usize = 0;
    while k < l1 {
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) + *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
        ) as isize);
        *ch.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) - *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
        ) as isize);
        k = k.wrapping_add(1)
    }
    if ido & 1 as libc::c_int as usize == 0 {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            *ch.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = -*cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize);
            *ch.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = *cc.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            );
            k_0 = k_0.wrapping_add(1)
        }
    }
    if ido <= 2usize {
        return;
    }
    let mut k_1: usize = 0;
    while k_1 < l1 {
        let mut i: usize = 2usize;
        while i < ido {
            let ic: usize = ido.wrapping_sub(i);
            let tr2: f64;
            let ti2: f64;
            tr2 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize)
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_add(
                        ido.wrapping_mul(
                            k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize)),
                        ),
                    ) as isize);
            ti2 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize)
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul(
                            k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize)),
                        ),
                    ) as isize);
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) + tr2;
            *ch.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize) = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) - tr2;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) = ti2
                + *cc.offset(
                    i.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0)))) as isize,
                );
            *ch.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize) = ti2
                - *cc.offset(
                    i.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0)))) as isize,
                );
            i = (i as usize).wrapping_add(2usize) as usize as usize
        }
        k_1 = k_1.wrapping_add(1)
    }
}
#[inline(never)]
unsafe fn radf3(ido: usize, l1: usize, cc: *const f64, ch: *mut f64, wa: *const f64) {
    let cdim: usize = 3 as libc::c_int as usize;
    static mut TAUR: f64 = -0.5f64;
    static mut TAUI: f64 = 0.86602540378443864676f64;
    let mut k: usize = 0;
    while k < l1 {
        let cr2: f64 = *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
        ) as isize)
            + *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            );
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) + cr2;
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) = TAUI
            * (*cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) - *cc.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize));
        *ch.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) + TAUR * cr2;
        k = k.wrapping_add(1)
    }
    if ido == 1 as libc::c_int as usize {
        return;
    }
    let mut k_0: usize = 0;
    while k_0 < l1 {
        let mut i: usize = 2usize;
        while i < ido {
            let ic: usize = ido.wrapping_sub(i);
            let di2: f64;
            let di3: f64;
            let dr2: f64;
            let dr3: f64;
            // PM(ic) = conj(t2-t3)
            dr2 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize)
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_add(
                        ido.wrapping_mul(
                            k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize)),
                        ),
                    ) as isize); // d2=conj(WA0)*CC1
            di2 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize)
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul(
                            k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize)),
                        ),
                    ) as isize); // d3=conj(WA1)*CC2
            dr3 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) + *wa.offset(
                i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ); // c add
            di3 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) - *wa.offset(
                i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ); // c add
            let cr2_0: f64 = dr2 + dr3; // c add
            let ci2: f64 = di2 + di3; // t3 = taui*i*(d3-d2)?
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) + cr2_0; // PM(i) = t2+t3
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) + ci2;
            let tr2: f64 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) + TAUR * cr2_0;
            let ti2: f64 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) + TAUR * ci2;
            let tr3: f64 = TAUI * (di2 - di3);
            let ti3: f64 = TAUI * (dr3 - dr2);
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = tr2 + tr3;
            *ch.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = tr2 - tr3;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = ti3 + ti2;
            *ch.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = ti3 - ti2;
            i = (i as usize).wrapping_add(2usize) as usize as usize
        }
        k_0 = k_0.wrapping_add(1)
    }
}
#[inline(never)]
unsafe fn radf4(ido: usize, l1: usize, cc: *const f64, ch: *mut f64, wa: *const f64) {
    let cdim: usize = 4 as libc::c_int as usize;
    static mut HSQT2: f64 = 0.70710678118654752440f64;
    let mut k: usize = 0;
    while k < l1 {
        let tr1: f64;
        let tr2: f64;
        tr1 = *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
        ) as isize)
            + *cc.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize);
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) = *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
        ) as isize)
            - *cc.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize);
        tr2 = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) + *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                as isize,
        );
        *ch.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) - *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                as isize,
        );
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) = tr2 + tr1;
        *ch.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize) = tr2 - tr1;
        k = k.wrapping_add(1)
    }
    if ido & 1 as libc::c_int as usize == 0 {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let ti1: f64 = -HSQT2
                * (*cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize)
                    + *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul(
                            k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize)),
                        ),
                    ) as isize));
            let tr1_0: f64 = HSQT2
                * (*cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
                ) as isize)
                    - *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul(
                            k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize)),
                        ),
                    ) as isize));
            *ch.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = *cc.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) + tr1_0;
            *ch.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = *cc.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) - tr1_0;
            *ch.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = ti1
                + *cc.offset(
                    ido.wrapping_sub(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                );
            *ch.offset((0usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = ti1
                - *cc.offset(
                    ido.wrapping_sub(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                        as isize,
                );
            k_0 = k_0.wrapping_add(1)
        }
    }
    if ido <= 2usize {
        return;
    }
    let mut k_1: usize = 0;
    while k_1 < l1 {
        let mut i: usize = 2usize;
        while i < ido {
            let ic: usize = ido.wrapping_sub(i);
            let ci2: f64;
            let ci3: f64;
            let ci4: f64;
            let cr2: f64;
            let cr3: f64;
            let cr4: f64;
            let ti1_0: f64;
            let ti2: f64;
            let ti3: f64;
            let ti4: f64;
            let tr1_1: f64;
            let tr2_0: f64;
            let tr3: f64;
            let tr4: f64;
            cr2 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize)
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_add(
                        ido.wrapping_mul(
                            k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize)),
                        ),
                    ) as isize);
            ci2 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize)
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul(
                            k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize)),
                        ),
                    ) as isize);
            cr3 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) + *wa.offset(
                i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            );
            ci3 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) - *wa.offset(
                i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            );
            cr4 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize)
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_add(
                        ido.wrapping_mul(
                            k_1.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize)),
                        ),
                    ) as isize);
            ci4 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize)
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul(
                            k_1.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize)),
                        ),
                    ) as isize);
            tr1_1 = cr4 + cr2;
            tr4 = cr4 - cr2;
            ti1_0 = ci2 + ci4;
            ti4 = ci2 - ci4;
            tr2_0 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) + cr3;
            tr3 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) - cr3;
            ti2 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) + ci3;
            ti3 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) - ci3;
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) = tr2_0 + tr1_1;
            *ch.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize) = tr2_0 - tr1_1;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) = ti1_0 + ti2;
            *ch.offset(ic.wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize) = ti1_0 - ti2;
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) = tr3 + ti4;
            *ch.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize) = tr3 - ti4;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) = tr4 + ti3;
            *ch.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize) = tr4 - ti3;
            i = (i as usize).wrapping_add(2usize) as usize as usize
        }
        k_1 = k_1.wrapping_add(1)
    }
}
#[inline(never)]
unsafe fn radf5(ido: usize, l1: usize, cc: *const f64, ch: *mut f64, wa: *const f64) {
    let cdim: usize = 5 as libc::c_int as usize;
    static mut TR11: f64 = 0.3090169943749474241f64;
    static mut TI11: f64 = 0.95105651629515357212f64;
    static mut TR12: f64 = -0.8090169943749474241f64;
    static mut TI12: f64 = 0.58778525229247312917f64;
    let mut k: usize = 0;
    while k < l1 {
        let cr2: f64;
        let cr3: f64;
        let ci4: f64;
        let ci5: f64;
        cr2 = *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
        ) as isize)
            + *cc.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize);
        ci5 = *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
        ) as isize)
            - *cc.offset((0usize).wrapping_add(
                ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize);
        cr3 = *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
        ) as isize)
            + *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            );
        ci4 = *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
        ) as isize)
            - *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            );
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) + cr2
            + cr3;
        *ch.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) + TR11 * cr2
            + TR12 * cr3;
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) = TI11 * ci5 + TI12 * ci4;
        *ch.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) + TR12 * cr2
            + TR11 * cr3;
        *ch.offset((0usize).wrapping_add(
            ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize) = TI12 * ci5 - TI11 * ci4;
        k = k.wrapping_add(1)
    }
    if ido == 1 as libc::c_int as usize {
        return;
    }
    let mut k_0: usize = 0;
    while k_0 < l1 {
        let mut i: usize = 2usize;
        while i < ido {
            let ci2: f64;
            let di2: f64;
            let ci4_0: f64;
            let ci5_0: f64;
            let di3: f64;
            let di4: f64;
            let di5: f64;
            let ci3: f64;
            let cr2_0: f64;
            let cr3_0: f64;
            let dr2: f64;
            let dr3: f64;
            let dr4: f64;
            let dr5: f64;
            let cr5: f64;
            let cr4: f64;
            let ti2: f64;
            let ti3: f64;
            let ti5: f64;
            let ti4: f64;
            let tr2: f64;
            let tr3: f64;
            let tr4: f64;
            let tr5: f64;
            let ic: usize = ido.wrapping_sub(i);
            dr2 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize)
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_add(
                        ido.wrapping_mul(
                            k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize)),
                        ),
                    ) as isize);
            di2 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize)
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul(
                            k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize)),
                        ),
                    ) as isize);
            dr3 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) + *wa.offset(
                i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            );
            di3 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) - *wa.offset(
                i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            );
            dr4 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize)
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_add(
                        ido.wrapping_mul(
                            k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize)),
                        ),
                    ) as isize);
            di4 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize)
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul(
                            k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize)),
                        ),
                    ) as isize);
            dr5 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (3 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize)
                + *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (3 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * *cc.offset(i.wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize);
            di5 = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (3 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * *cc.offset(i.wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize)
                - *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (3 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
                ) as isize);
            cr2_0 = dr5 + dr2;
            ci5_0 = dr5 - dr2;
            ci2 = di2 + di5;
            cr5 = di2 - di5;
            cr3_0 = dr4 + dr3;
            ci4_0 = dr4 - dr3;
            ci3 = di3 + di4;
            cr4 = di3 - di4;
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) + cr2_0
                + cr3_0;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) + ci2
                + ci3;
            tr2 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) + TR11 * cr2_0
                + TR12 * cr3_0;
            ti2 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) + TR11 * ci2
                + TR12 * ci3;
            tr3 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) + TR12 * cr2_0
                + TR11 * cr3_0;
            ti3 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) + TR12 * ci2
                + TR11 * ci3;
            tr5 = cr5 * TI11 + cr4 * TI12;
            tr4 = cr5 * TI12 - cr4 * TI11;
            ti5 = ci5_0 * TI11 + ci4_0 * TI12;
            ti4 = ci5_0 * TI12 - ci4_0 * TI11;
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = tr2 + tr5;
            *ch.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = tr2 - tr5;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) = ti5 + ti2;
            *ch.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = ti5 - ti2;
            *ch.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = tr3 + tr4;
            *ch.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = tr3 - tr4;
            *ch.offset(i.wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = ti4 + ti3;
            *ch.offset(ic.wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize) = ti4 - ti3;
            i = (i as usize).wrapping_add(2usize) as usize as usize
        }
        k_0 = k_0.wrapping_add(1)
    }
}
#[inline(never)]
unsafe fn radfg(
    ido: usize,
    ip: usize,
    l1: usize,
    cc: *mut f64,
    ch: *mut f64,
    wa: *const f64,
    csarr: *const f64,
) {
    let cdim: usize = ip;
    let ipph: usize = ip
        .wrapping_add(1 as libc::c_int as usize)
        .wrapping_div(2usize);
    let idl1: usize = ido.wrapping_mul(l1);
    if ido > 1 as libc::c_int as usize {
        let mut j: usize = 1 as libc::c_int as usize;
        let mut jc: usize = ip.wrapping_sub(1 as libc::c_int as usize);
        while j < ipph {
            // 114
            let is: usize = j
                .wrapping_sub(1 as libc::c_int as usize)
                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize));
            let is2: usize = jc
                .wrapping_sub(1 as libc::c_int as usize)
                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize));
            let mut k: usize = 0;
            while k < l1 {
                // 113
                let mut idij: usize = is;
                let mut idij2: usize = is2;
                let mut i: usize = 1 as libc::c_int as usize;
                while i <= ido.wrapping_sub(2usize) {
                    // 112
                    let t1: f64 = *cc.offset(
                        i.wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(j))))
                            as isize,
                    );
                    let t2: f64 = *cc.offset(
                        i.wrapping_add(1 as libc::c_int as usize)
                            .wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(j))))
                            as isize,
                    );
                    let t3: f64 = *cc.offset(
                        i.wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(jc))))
                            as isize,
                    );
                    let t4: f64 = *cc.offset(
                        i.wrapping_add(1 as libc::c_int as usize)
                            .wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(jc))))
                            as isize,
                    );
                    let x1: f64 = *wa.offset(idij as isize) * t1
                        + *wa.offset(idij.wrapping_add(1 as libc::c_int as usize) as isize) * t2;
                    let x2: f64 = *wa.offset(idij as isize) * t2
                        - *wa.offset(idij.wrapping_add(1 as libc::c_int as usize) as isize) * t1;
                    let x3: f64 = *wa.offset(idij2 as isize) * t3
                        + *wa.offset(idij2.wrapping_add(1 as libc::c_int as usize) as isize) * t4;
                    let x4: f64 = *wa.offset(idij2 as isize) * t4
                        - *wa.offset(idij2.wrapping_add(1 as libc::c_int as usize) as isize) * t3;
                    *cc.offset(
                        i.wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(j))))
                            as isize,
                    ) = x1 + x3;
                    *cc.offset(
                        i.wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(jc))))
                            as isize,
                    ) = x2 - x4;
                    *cc.offset(
                        i.wrapping_add(1 as libc::c_int as usize)
                            .wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(j))))
                            as isize,
                    ) = x2 + x4;
                    *cc.offset(
                        i.wrapping_add(1 as libc::c_int as usize)
                            .wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(jc))))
                            as isize,
                    ) = x3 - x1;
                    idij = (idij as usize).wrapping_add(2usize) as usize as usize;
                    idij2 = (idij2 as usize).wrapping_add(2usize) as usize as usize;
                    i = (i as usize).wrapping_add(2usize) as usize as usize
                }
                k = k.wrapping_add(1)
            }
            j = j.wrapping_add(1);
            jc = jc.wrapping_sub(1)
        }
    }
    let mut j_0: usize = 1 as libc::c_int as usize;
    let mut jc_0: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while j_0 < ipph {
        // 123
        let mut k_0: usize = 0;
        while k_0 < l1 {
            // 122
            let t1_0: f64 = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(j_0))))
                    as isize,
            );
            let t2_0: f64 = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(jc_0))))
                    as isize,
            );
            *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(j_0))))
                    as isize,
            ) = t1_0 + t2_0;
            *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(jc_0))))
                    as isize,
            ) = t2_0 - t1_0;
            k_0 = k_0.wrapping_add(1)
        }
        j_0 = j_0.wrapping_add(1);
        jc_0 = jc_0.wrapping_sub(1)
    }
    //everything in C
    //memset(ch,0,ip*l1*ido*sizeof(double));
    let mut l: usize = 1 as libc::c_int as usize;
    let mut lc: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while l < ipph {
        // 127
        let mut ik: usize = 0;
        while ik < idl1 {
            // 124
            *ch.offset(ik.wrapping_add(idl1.wrapping_mul(l)) as isize) = *cc
                .offset(ik.wrapping_add(idl1.wrapping_mul(0)) as isize)
                + *csarr.offset((2usize).wrapping_mul(l) as isize)
                    * *cc.offset(
                        ik.wrapping_add(idl1.wrapping_mul(1 as libc::c_int as usize)) as isize,
                    )
                + *csarr.offset((4 as libc::c_int as usize).wrapping_mul(l) as isize)
                    * *cc.offset(ik.wrapping_add(idl1.wrapping_mul(2usize)) as isize);
            *ch.offset(ik.wrapping_add(idl1.wrapping_mul(lc)) as isize) = *csarr.offset(
                (2usize)
                    .wrapping_mul(l)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            ) * *cc.offset(
                ik.wrapping_add(idl1.wrapping_mul(ip.wrapping_sub(1 as libc::c_int as usize)))
                    as isize,
            ) + *csarr.offset(
                (4 as libc::c_int as usize)
                    .wrapping_mul(l)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            ) * *cc
                .offset(ik.wrapping_add(idl1.wrapping_mul(ip.wrapping_sub(2usize))) as isize);
            ik = ik.wrapping_add(1)
        }
        let mut iang: usize = (2usize).wrapping_mul(l);
        let mut j_1: usize = 3 as libc::c_int as usize;
        let mut jc_1: usize = ip.wrapping_sub(3 as libc::c_int as usize);
        while j_1 < ipph.wrapping_sub(3 as libc::c_int as usize) {
            // 126
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang >= ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar1: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai1: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang >= ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar2: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai2: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang >= ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar3: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai3: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang >= ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar4: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai4: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            let mut ik_0: usize = 0;
            while ik_0 < idl1 {
                // 125
                *ch.offset(ik_0.wrapping_add(idl1.wrapping_mul(l)) as isize) += ar1
                    * *cc.offset(ik_0.wrapping_add(idl1.wrapping_mul(j_1)) as isize)
                    + ar2
                        * *cc.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(j_1.wrapping_add(1 as libc::c_int as usize)),
                        ) as isize)
                    + ar3
                        * *cc.offset(
                            ik_0.wrapping_add(idl1.wrapping_mul(j_1.wrapping_add(2usize))) as isize,
                        )
                    + ar4
                        * *cc.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(j_1.wrapping_add(3 as libc::c_int as usize)),
                        ) as isize);
                *ch.offset(ik_0.wrapping_add(idl1.wrapping_mul(lc)) as isize) += ai1
                    * *cc.offset(ik_0.wrapping_add(idl1.wrapping_mul(jc_1)) as isize)
                    + ai2
                        * *cc.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(jc_1.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize)
                    + ai3
                        * *cc.offset(
                            ik_0.wrapping_add(idl1.wrapping_mul(jc_1.wrapping_sub(2usize)))
                                as isize,
                        )
                    + ai4
                        * *cc.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(jc_1.wrapping_sub(3 as libc::c_int as usize)),
                        ) as isize);
                ik_0 = ik_0.wrapping_add(1)
            }
            j_1 = (j_1 as usize).wrapping_add(4 as libc::c_int as usize) as usize as usize;
            jc_1 = (jc_1 as usize).wrapping_sub(4 as libc::c_int as usize) as usize as usize
        }
        while j_1 < ipph.wrapping_sub(1 as libc::c_int as usize) {
            // 126
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang >= ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar1_0: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai1_0: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang >= ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar2_0: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai2_0: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            let mut ik_1: usize = 0;
            while ik_1 < idl1 {
                // 125
                *ch.offset(ik_1.wrapping_add(idl1.wrapping_mul(l)) as isize) += ar1_0
                    * *cc.offset(ik_1.wrapping_add(idl1.wrapping_mul(j_1)) as isize)
                    + ar2_0
                        * *cc.offset(ik_1.wrapping_add(
                            idl1.wrapping_mul(j_1.wrapping_add(1 as libc::c_int as usize)),
                        ) as isize);
                *ch.offset(ik_1.wrapping_add(idl1.wrapping_mul(lc)) as isize) += ai1_0
                    * *cc.offset(ik_1.wrapping_add(idl1.wrapping_mul(jc_1)) as isize)
                    + ai2_0
                        * *cc.offset(ik_1.wrapping_add(
                            idl1.wrapping_mul(jc_1.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize);
                ik_1 = ik_1.wrapping_add(1)
            }
            j_1 = (j_1 as usize).wrapping_add(2usize) as usize as usize;
            jc_1 = (jc_1 as usize).wrapping_sub(2usize) as usize as usize
        }
        while j_1 < ipph {
            // 126
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang >= ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            let mut ik_2: usize = 0;
            while ik_2 < idl1 {
                // 125
                *ch.offset(ik_2.wrapping_add(idl1.wrapping_mul(l)) as isize) +=
                    ar * *cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(j_1)) as isize);
                *ch.offset(ik_2.wrapping_add(idl1.wrapping_mul(lc)) as isize) +=
                    ai * *cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(jc_1)) as isize);
                ik_2 = ik_2.wrapping_add(1)
            }
            j_1 = j_1.wrapping_add(1);
            jc_1 = jc_1.wrapping_sub(1)
        }
        l = l.wrapping_add(1);
        lc = lc.wrapping_sub(1)
    }
    let mut ik_3: usize = 0;
    while ik_3 < idl1 {
        // 101
        *ch.offset(ik_3.wrapping_add(idl1.wrapping_mul(0)) as isize) =
            *cc.offset(ik_3.wrapping_add(idl1.wrapping_mul(0)) as isize);
        ik_3 = ik_3.wrapping_add(1)
    }
    let mut j_2: usize = 1 as libc::c_int as usize;
    while j_2 < ipph {
        // 129
        let mut ik_4: usize = 0;
        while ik_4 < idl1 {
            // 128
            *ch.offset(ik_4.wrapping_add(idl1.wrapping_mul(0)) as isize) +=
                *cc.offset(ik_4.wrapping_add(idl1.wrapping_mul(j_2)) as isize);
            ik_4 = ik_4.wrapping_add(1)
        }
        j_2 = j_2.wrapping_add(1)
    }
    // everything in CH at this point!
    //memset(cc,0,ip*l1*ido*sizeof(double));
    let mut k_1: usize = 0;
    while k_1 < l1 {
        // 131
        let mut i_0: usize = 0;
        while i_0 < ido {
            // 130
            *cc.offset(
                i_0.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) = *ch.offset(
                i_0.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0)))) as isize,
            );
            i_0 = i_0.wrapping_add(1)
        }
        k_1 = k_1.wrapping_add(1)
    }
    let mut j_3: usize = 1 as libc::c_int as usize;
    let mut jc_2: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while j_3 < ipph {
        // 137
        let j2: usize = (2usize)
            .wrapping_mul(j_3)
            .wrapping_sub(1 as libc::c_int as usize);
        let mut k_2: usize = 0;
        while k_2 < l1 {
            // 136
            *cc.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(j2.wrapping_add(cdim.wrapping_mul(k_2))))
                    as isize,
            ) = *ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                    as isize,
            );
            *cc.offset(
                (0usize).wrapping_add(
                    ido.wrapping_mul(
                        j2.wrapping_add(1 as libc::c_int as usize)
                            .wrapping_add(cdim.wrapping_mul(k_2)),
                    ),
                ) as isize,
            ) = *ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                    as isize,
            );
            k_2 = k_2.wrapping_add(1)
        }
        j_3 = j_3.wrapping_add(1);
        jc_2 = jc_2.wrapping_sub(1)
    }
    if ido == 1 as libc::c_int as usize {
        return;
    }
    let mut j_4: usize = 1 as libc::c_int as usize;
    let mut jc_3: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while j_4 < ipph {
        // 140
        let j2_0: usize = (2usize)
            .wrapping_mul(j_4)
            .wrapping_sub(1 as libc::c_int as usize);
        let mut k_3: usize = 0;
        while k_3 < l1 {
            // 139
            let mut i_1: usize = 1 as libc::c_int as usize;
            let mut ic: usize = ido.wrapping_sub(i_1).wrapping_sub(2usize);
            while i_1 <= ido.wrapping_sub(2usize) {
                // 138
                *cc.offset(
                    i_1.wrapping_add(
                        ido.wrapping_mul(
                            j2_0.wrapping_add(1 as libc::c_int as usize)
                                .wrapping_add(cdim.wrapping_mul(k_3)),
                        ),
                    ) as isize,
                ) = *ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ) + *ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                ); // t2=CC(I) + conj(CC(ic))
                *cc.offset(
                    ic.wrapping_add(ido.wrapping_mul(j2_0.wrapping_add(cdim.wrapping_mul(k_3))))
                        as isize,
                ) = *ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ) - *ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                ); // c2=CC +taur*t2
                *cc.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul(
                            j2_0.wrapping_add(1 as libc::c_int as usize)
                                .wrapping_add(cdim.wrapping_mul(k_3)),
                        ),
                    ) as isize,
                ) = *ch.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ) + *ch.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                ); // CH=CC+t2
                *cc.offset(
                    ic.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(j2_0.wrapping_add(cdim.wrapping_mul(k_3))))
                        as isize,
                ) = *ch.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                ) - *ch.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ); // c3=taui*(CC(i)-conj(CC(ic)))
                i_1 = (i_1 as usize).wrapping_add(2usize) as usize as usize; // d2= (cr2-ci3, ci2+cr3) = c2+i*c3
                ic = (ic as usize).wrapping_sub(2usize) as usize as usize
            } // d3= (cr2+ci3, ci2-cr3) = c2-i*c3
            k_3 = k_3.wrapping_add(1)
        } // ch = WA*d2
        j_4 = j_4.wrapping_add(1);
        jc_3 = jc_3.wrapping_sub(1)
    }
}
#[inline(never)]
unsafe fn radb2(ido: usize, l1: usize, cc: *const f64, ch: *mut f64, wa: *const f64) {
    let cdim: usize = 2usize;
    let mut k: usize = 0;
    while k < l1 {
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) + *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize);
        *ch.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
        ) as isize) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) - *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize);
        k = k.wrapping_add(1)
    }
    if ido & 1 as libc::c_int as usize == 0 {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            *ch.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) =
                2.0f64
                    * *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize);
            *ch.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) =
                -2.0f64
                    * *cc.offset((0usize).wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize);
            k_0 = k_0.wrapping_add(1)
        }
    }
    if ido <= 2usize {
        return;
    }
    let mut k_1: usize = 0;
    while k_1 < l1 {
        let mut i: usize = 2usize;
        while i < ido {
            let ic: usize = ido.wrapping_sub(i);
            let ti2: f64;
            let tr2: f64;
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) + *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            tr2 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) - *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            ti2 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) + *cc.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) - *cc.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            *ch.offset(i.wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * ti2
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * tr2;
            *ch.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * tr2
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * ti2;
            i = (i as usize).wrapping_add(2usize) as usize as usize
        }
        k_1 = k_1.wrapping_add(1)
    }
}
#[inline(never)]
unsafe fn radb3(ido: usize, l1: usize, cc: *const f64, ch: *mut f64, wa: *const f64) {
    let cdim: usize = 3 as libc::c_int as usize;
    static mut TAUR: f64 = -0.5f64;
    static mut TAUI: f64 = 0.86602540378443864676f64;
    let mut k: usize = 0;
    while k < l1 {
        let tr2: f64 = 2.0f64
            * *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize);
        let cr2: f64 = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) + TAUR * tr2;
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) + tr2;
        let ci3: f64 = 2.0f64
            * TAUI
            * *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                as isize,
        ) = cr2 + ci3;
        *ch.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
        ) as isize) = cr2 - ci3;
        k = k.wrapping_add(1)
    }
    if ido == 1 as libc::c_int as usize {
        return;
    }
    let mut k_0: usize = 0;
    while k_0 < l1 {
        let mut i: usize = 2usize;
        while i < ido {
            let ic: usize = ido.wrapping_sub(i);
            let tr2_0: f64 =
                *cc.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize,
                ) + *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    ),
                ) as isize);
            let ti2: f64 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) - *cc.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize);
            let cr2_0: f64 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + TAUR * tr2_0;
            let ci2: f64 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + TAUR * ti2;
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + tr2_0;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + ti2;
            let cr3: f64 = TAUI
                * (*cc.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize,
                ) - *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    ),
                ) as isize));
            let ci3_0: f64 = TAUI
                * (*cc
                    .offset(i.wrapping_add(
                        ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))),
                    ) as isize)
                    + *cc.offset(ic.wrapping_add(ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    )) as isize));
            let di2: f64;
            let di3: f64;
            let dr2: f64;
            let dr3: f64;
            dr3 = cr2_0 + ci3_0;
            dr2 = cr2_0 - ci3_0;
            di2 = ci2 + cr3;
            di3 = ci2 - cr3;
            *ch.offset(i.wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * di2
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * dr2;
            *ch.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * dr2
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * di2;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * di3
                + *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * dr3;
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * dr3
                - *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * di3;
            i = (i as usize).wrapping_add(2usize) as usize as usize
        }
        k_0 = k_0.wrapping_add(1)
    }
}
#[inline(never)]
unsafe fn radb4(ido: usize, l1: usize, cc: *const f64, ch: *mut f64, wa: *const f64) {
    let cdim: usize = 4 as libc::c_int as usize;
    static mut SQRT2: f64 = 1.41421356237309504880f64;
    let mut k: usize = 0;
    while k < l1 {
        let tr1: f64;
        let tr2: f64;
        tr2 = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) + *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize);
        tr1 = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) - *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize);
        let tr3: f64 = 2.0f64
            * *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize);
        let tr4: f64 = 2.0f64
            * *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) = tr2 + tr3;
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                as isize,
        ) = tr2 - tr3;
        *ch.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
        ) as isize) = tr1 + tr4;
        *ch.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
        ) as isize) = tr1 - tr4;
        k = k.wrapping_add(1)
    }
    if ido & 1 as libc::c_int as usize == 0 {
        let mut k_0: usize = 0;
        while k_0 < l1 {
            let tr1_0: f64;
            let tr2_0: f64;
            let ti1: f64;
            let ti2: f64;
            ti1 = *cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize)
                + *cc.offset((0usize).wrapping_add(
                    ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    ),
                ) as isize);
            ti2 = *cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize)
                - *cc.offset((0usize).wrapping_add(
                    ido.wrapping_mul(
                        (1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    ),
                ) as isize);
            tr2_0 = *cc.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + *cc.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            );
            tr1_0 = *cc.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) - *cc.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            );
            *ch.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) = tr2_0 + tr2_0;
            *ch.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) = SQRT2 * (tr1_0 - ti1);
            *ch.offset(
                ido.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) = ti2 + ti2;
            *ch.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize) = -SQRT2 * (tr1_0 + ti1);
            k_0 = k_0.wrapping_add(1)
        }
    }
    if ido <= 2usize {
        return;
    }
    let mut k_1: usize = 0;
    while k_1 < l1 {
        let mut i: usize = 2usize;
        while i < ido {
            let ci2: f64;
            let ci3: f64;
            let ci4: f64;
            let cr2: f64;
            let cr3: f64;
            let cr4: f64;
            let ti1_0: f64;
            let ti2_0: f64;
            let ti3: f64;
            let ti4: f64;
            let tr1_1: f64;
            let tr2_1: f64;
            let tr3_0: f64;
            let tr4_0: f64;
            let ic: usize = ido.wrapping_sub(i);
            tr2_1 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) + *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            tr1_1 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) - *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            ti1_0 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) + *cc.offset(ic.wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            ti2_0 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) - *cc.offset(ic.wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            tr4_0 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) + *cc.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            ti3 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) - *cc.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            tr3_0 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) + *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            ti4 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_1))))
                    as isize,
            ) - *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_1))),
            ) as isize);
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) = tr2_1 + tr3_0;
            cr3 = tr2_1 - tr3_0;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) = ti2_0 + ti3;
            ci3 = ti2_0 - ti3;
            cr4 = tr1_1 + tr4_0;
            cr2 = tr1_1 - tr4_0;
            ci2 = ti1_0 + ti4;
            ci4 = ti1_0 - ti4;
            *ch.offset(i.wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * ci2
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * cr2;
            *ch.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * cr2
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * ci2;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * ci3
                + *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * cr3;
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * cr3
                - *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * ci3;
            *ch.offset(i.wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * ci4
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * cr4;
            *ch.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * cr4
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * ci4;
            i = (i as usize).wrapping_add(2usize) as usize as usize
        }
        k_1 = k_1.wrapping_add(1)
    }
}
#[inline(never)]
unsafe fn radb5(ido: usize, l1: usize, cc: *const f64, ch: *mut f64, wa: *const f64) {
    let cdim: usize = 5 as libc::c_int as usize;
    static mut TR11: f64 = 0.3090169943749474241f64;
    static mut TI11: f64 = 0.95105651629515357212f64;
    static mut TR12: f64 = -0.8090169943749474241f64;
    static mut TI12: f64 = 0.58778525229247312917f64;
    let mut k: usize = 0;
    while k < l1 {
        let ti5: f64 = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) + *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        );
        let ti4: f64 = *cc.offset((0usize).wrapping_add(
            ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize)
            + *cc.offset((0usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize);
        let tr2: f64 = *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize)
            + *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize);
        let tr3: f64 = *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
            ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
        ) as isize)
            + *cc.offset(ido.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k))),
            ) as isize);
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
        ) = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) + tr2
            + tr3;
        let cr2: f64 = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) + TR11 * tr2
            + TR12 * tr3;
        let cr3: f64 = *cc.offset(
            (0usize).wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                as isize,
        ) + TR12 * tr2
            + TR11 * tr3;
        let ci4: f64;
        let ci5: f64;
        ci5 = ti5 * TI11 + ti4 * TI12;
        ci4 = ti5 * TI12 - ti4 * TI11;
        *ch.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
        ) as isize) = cr2 + ci5;
        *ch.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
        ) as isize) = cr2 - ci5;
        *ch.offset((0usize).wrapping_add(
            ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
        ) as isize) = cr3 + ci4;
        *ch.offset(
            (0usize).wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(2usize))))
                as isize,
        ) = cr3 - ci4;
        k = k.wrapping_add(1)
    }
    if ido == 1 as libc::c_int as usize {
        return;
    }
    let mut k_0: usize = 0;
    while k_0 < l1 {
        let mut i: usize = 2usize;
        while i < ido {
            let ic: usize = ido.wrapping_sub(i);
            let tr2_0: f64;
            let tr3_0: f64;
            let tr4: f64;
            let tr5: f64;
            let ti2: f64;
            let ti3: f64;
            let ti4_0: f64;
            let ti5_0: f64;
            tr2_0 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize);
            tr5 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) - *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize);
            ti5_0 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + *cc.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize);
            ti2 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((2usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) - *cc.offset(ic.wrapping_add(
                ido.wrapping_mul((1 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize);
            tr3_0 = *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize)
                + *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    ),
                ) as isize);
            tr4 = *cc.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize)
                - *cc.offset(ic.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    ),
                ) as isize);
            ti4_0 = *cc.offset(i.wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize)
                + *cc.offset(ic.wrapping_add(
                    ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    ),
                ) as isize);
            ti3 = *cc.offset(i.wrapping_add(
                ido.wrapping_mul((4 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0))),
            ) as isize)
                - *cc.offset(ic.wrapping_add(
                    ido.wrapping_mul(
                        (3 as libc::c_int as usize).wrapping_add(cdim.wrapping_mul(k_0)),
                    ),
                ) as isize);
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0))))
                    as isize,
            ) = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + tr2_0
                + tr3_0;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + ti2
                + ti3;
            let cr2_0: f64 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + TR11 * tr2_0
                + TR12 * tr3_0;
            let ci2: f64 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + TR11 * ti2
                + TR12 * ti3;
            let cr3_0: f64 = *cc.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + TR12 * tr2_0
                + TR11 * tr3_0;
            let ci3: f64 = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k_0))))
                    as isize,
            ) + TR12 * ti2
                + TR11 * ti3;
            let ci4_0: f64;
            let ci5_0: f64;
            let cr5: f64;
            let cr4: f64;
            cr5 = tr5 * TI11 + tr4 * TI12;
            cr4 = tr5 * TI12 - tr4 * TI11;
            ci5_0 = ti5_0 * TI11 + ti4_0 * TI12;
            ci4_0 = ti5_0 * TI12 - ti4_0 * TI11;
            let dr2: f64;
            let dr3: f64;
            let dr4: f64;
            let dr5: f64;
            let di2: f64;
            let di3: f64;
            let di4: f64;
            let di5: f64;
            dr4 = cr3_0 + ci4_0;
            dr3 = cr3_0 - ci4_0;
            di3 = ci3 + cr4;
            di4 = ci3 - cr4;
            dr5 = cr2_0 + ci5_0;
            dr2 = cr2_0 - ci5_0;
            di2 = ci2 + cr5;
            di5 = ci2 - cr5;
            *ch.offset(i.wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * di2
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * dr2;
            *ch.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(1 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * dr2
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (0usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * di2;
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * di3
                + *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * dr3;
            *ch.offset(
                i.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(2usize))))
                    as isize,
            ) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (1 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * dr3
                - *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * di3;
            *ch.offset(i.wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * di4
                + *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * dr4;
            *ch.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(3 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * dr4
                - *wa.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                    (2usize).wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize)
                    * di4;
            *ch.offset(i.wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (3 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * di5
                + *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (3 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * dr5;
            *ch.offset(i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(4 as libc::c_int as usize))),
            ) as isize) = *wa.offset(
                i.wrapping_sub(2usize).wrapping_add(
                    (3 as libc::c_int as usize)
                        .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                ) as isize,
            ) * dr5
                - *wa.offset(
                    i.wrapping_sub(1 as libc::c_int as usize).wrapping_add(
                        (3 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
                    ) as isize,
                ) * di5;
            i = (i as usize).wrapping_add(2usize) as usize as usize
        }
        k_0 = k_0.wrapping_add(1)
    }
}
#[inline(never)]
unsafe fn radbg(
    ido: usize,
    ip: usize,
    l1: usize,
    cc: *mut f64,
    ch: *mut f64,
    wa: *const f64,
    csarr: *const f64,
) {
    let cdim: usize = ip;
    let ipph: usize = ip
        .wrapping_add(1 as libc::c_int as usize)
        .wrapping_div(2usize);
    let idl1: usize = ido.wrapping_mul(l1);
    let mut k: usize = 0;
    while k < l1 {
        // 102
        let mut i: usize = 0;
        while i < ido {
            // 101
            *ch.offset(
                i.wrapping_add(ido.wrapping_mul(k.wrapping_add(l1.wrapping_mul(0)))) as isize,
            ) = *cc.offset(
                i.wrapping_add(ido.wrapping_mul((0usize).wrapping_add(cdim.wrapping_mul(k))))
                    as isize,
            );
            i = i.wrapping_add(1)
        }
        k = k.wrapping_add(1)
    }
    let mut j: usize = 1 as libc::c_int as usize;
    let mut jc: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while j < ipph {
        // 108
        let j2: usize = (2usize)
            .wrapping_mul(j)
            .wrapping_sub(1 as libc::c_int as usize);
        let mut k_0: usize = 0;
        while k_0 < l1 {
            *ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(j))))
                    as isize,
            ) = 2 as libc::c_int as f64
                * *cc.offset(
                    ido.wrapping_sub(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(j2.wrapping_add(cdim.wrapping_mul(k_0))))
                        as isize,
                );
            *ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_0.wrapping_add(l1.wrapping_mul(jc))))
                    as isize,
            ) = 2 as libc::c_int as f64
                * *cc.offset(
                    (0usize).wrapping_add(
                        ido.wrapping_mul(
                            j2.wrapping_add(1 as libc::c_int as usize)
                                .wrapping_add(cdim.wrapping_mul(k_0)),
                        ),
                    ) as isize,
                );
            k_0 = k_0.wrapping_add(1)
        }
        j = j.wrapping_add(1);
        jc = jc.wrapping_sub(1)
    }
    if ido != 1 as libc::c_int as usize {
        let mut j_0: usize = 1 as libc::c_int as usize;
        let mut jc_0: usize = ip.wrapping_sub(1 as libc::c_int as usize);
        while j_0 < ipph {
            // 111
            let j2_0: usize = (2usize)
                .wrapping_mul(j_0)
                .wrapping_sub(1 as libc::c_int as usize);
            let mut k_1: usize = 0;
            while k_1 < l1 {
                let mut i_0: usize = 1 as libc::c_int as usize;
                let mut ic: usize = ido.wrapping_sub(i_0).wrapping_sub(2usize);
                while i_0 <= ido.wrapping_sub(2usize) {
                    // 109
                    *ch.offset(
                        i_0.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(j_0))))
                            as isize,
                    ) = *cc.offset(
                        i_0.wrapping_add(
                            ido.wrapping_mul(
                                j2_0.wrapping_add(1 as libc::c_int as usize)
                                    .wrapping_add(cdim.wrapping_mul(k_1)),
                            ),
                        ) as isize,
                    ) + *cc
                        .offset(ic.wrapping_add(
                            ido.wrapping_mul(j2_0.wrapping_add(cdim.wrapping_mul(k_1))),
                        ) as isize);
                    *ch.offset(
                        i_0.wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(jc_0))))
                            as isize,
                    ) = *cc.offset(
                        i_0.wrapping_add(
                            ido.wrapping_mul(
                                j2_0.wrapping_add(1 as libc::c_int as usize)
                                    .wrapping_add(cdim.wrapping_mul(k_1)),
                            ),
                        ) as isize,
                    ) - *cc
                        .offset(ic.wrapping_add(
                            ido.wrapping_mul(j2_0.wrapping_add(cdim.wrapping_mul(k_1))),
                        ) as isize);
                    *ch.offset(
                        i_0.wrapping_add(1 as libc::c_int as usize)
                            .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(j_0))))
                            as isize,
                    ) =
                        *cc.offset(
                            i_0.wrapping_add(1 as libc::c_int as usize).wrapping_add(
                                ido.wrapping_mul(
                                    j2_0.wrapping_add(1 as libc::c_int as usize)
                                        .wrapping_add(cdim.wrapping_mul(k_1)),
                                ),
                            ) as isize,
                        ) - *cc.offset(ic.wrapping_add(1 as libc::c_int as usize).wrapping_add(
                            ido.wrapping_mul(j2_0.wrapping_add(cdim.wrapping_mul(k_1))),
                        ) as isize);
                    *ch.offset(
                        i_0.wrapping_add(1 as libc::c_int as usize)
                            .wrapping_add(ido.wrapping_mul(k_1.wrapping_add(l1.wrapping_mul(jc_0))))
                            as isize,
                    ) =
                        *cc.offset(
                            i_0.wrapping_add(1 as libc::c_int as usize).wrapping_add(
                                ido.wrapping_mul(
                                    j2_0.wrapping_add(1 as libc::c_int as usize)
                                        .wrapping_add(cdim.wrapping_mul(k_1)),
                                ),
                            ) as isize,
                        ) + *cc.offset(ic.wrapping_add(1 as libc::c_int as usize).wrapping_add(
                            ido.wrapping_mul(j2_0.wrapping_add(cdim.wrapping_mul(k_1))),
                        ) as isize);
                    i_0 = (i_0 as usize).wrapping_add(2usize) as usize as usize;
                    ic = (ic as usize).wrapping_sub(2usize) as usize as usize
                }
                k_1 = k_1.wrapping_add(1)
            }
            j_0 = j_0.wrapping_add(1);
            jc_0 = jc_0.wrapping_sub(1)
        }
    }
    let mut l: usize = 1 as libc::c_int as usize;
    let mut lc: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while l < ipph {
        let mut ik: usize = 0;
        while ik < idl1 {
            *cc.offset(ik.wrapping_add(idl1.wrapping_mul(l)) as isize) = *ch
                .offset(ik.wrapping_add(idl1.wrapping_mul(0)) as isize)
                + *csarr.offset((2usize).wrapping_mul(l) as isize)
                    * *ch.offset(
                        ik.wrapping_add(idl1.wrapping_mul(1 as libc::c_int as usize)) as isize,
                    )
                + *csarr.offset((4 as libc::c_int as usize).wrapping_mul(l) as isize)
                    * *ch.offset(ik.wrapping_add(idl1.wrapping_mul(2usize)) as isize);
            *cc.offset(ik.wrapping_add(idl1.wrapping_mul(lc)) as isize) = *csarr.offset(
                (2usize)
                    .wrapping_mul(l)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            ) * *ch.offset(
                ik.wrapping_add(idl1.wrapping_mul(ip.wrapping_sub(1 as libc::c_int as usize)))
                    as isize,
            ) + *csarr.offset(
                (4 as libc::c_int as usize)
                    .wrapping_mul(l)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            ) * *ch
                .offset(ik.wrapping_add(idl1.wrapping_mul(ip.wrapping_sub(2usize))) as isize);
            ik = ik.wrapping_add(1)
        }
        let mut iang: usize = (2usize).wrapping_mul(l);
        let mut j_1: usize = 3 as libc::c_int as usize;
        let mut jc_1: usize = ip.wrapping_sub(3 as libc::c_int as usize);
        while j_1 < ipph.wrapping_sub(3 as libc::c_int as usize) {
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang > ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar1: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai1: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang > ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar2: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai2: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang > ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar3: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai3: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang > ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar4: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai4: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            let mut ik_0: usize = 0;
            while ik_0 < idl1 {
                *cc.offset(ik_0.wrapping_add(idl1.wrapping_mul(l)) as isize) += ar1
                    * *ch.offset(ik_0.wrapping_add(idl1.wrapping_mul(j_1)) as isize)
                    + ar2
                        * *ch.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(j_1.wrapping_add(1 as libc::c_int as usize)),
                        ) as isize)
                    + ar3
                        * *ch.offset(
                            ik_0.wrapping_add(idl1.wrapping_mul(j_1.wrapping_add(2usize))) as isize,
                        )
                    + ar4
                        * *ch.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(j_1.wrapping_add(3 as libc::c_int as usize)),
                        ) as isize);
                *cc.offset(ik_0.wrapping_add(idl1.wrapping_mul(lc)) as isize) += ai1
                    * *ch.offset(ik_0.wrapping_add(idl1.wrapping_mul(jc_1)) as isize)
                    + ai2
                        * *ch.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(jc_1.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize)
                    + ai3
                        * *ch.offset(
                            ik_0.wrapping_add(idl1.wrapping_mul(jc_1.wrapping_sub(2usize)))
                                as isize,
                        )
                    + ai4
                        * *ch.offset(ik_0.wrapping_add(
                            idl1.wrapping_mul(jc_1.wrapping_sub(3 as libc::c_int as usize)),
                        ) as isize);
                ik_0 = ik_0.wrapping_add(1)
            }
            j_1 = (j_1 as usize).wrapping_add(4 as libc::c_int as usize) as usize as usize;
            jc_1 = (jc_1 as usize).wrapping_sub(4 as libc::c_int as usize) as usize as usize
        }
        while j_1 < ipph.wrapping_sub(1 as libc::c_int as usize) {
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang > ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar1_0: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai1_0: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang > ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let ar2_0: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let ai2_0: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            let mut ik_1: usize = 0;
            while ik_1 < idl1 {
                *cc.offset(ik_1.wrapping_add(idl1.wrapping_mul(l)) as isize) += ar1_0
                    * *ch.offset(ik_1.wrapping_add(idl1.wrapping_mul(j_1)) as isize)
                    + ar2_0
                        * *ch.offset(ik_1.wrapping_add(
                            idl1.wrapping_mul(j_1.wrapping_add(1 as libc::c_int as usize)),
                        ) as isize);
                *cc.offset(ik_1.wrapping_add(idl1.wrapping_mul(lc)) as isize) += ai1_0
                    * *ch.offset(ik_1.wrapping_add(idl1.wrapping_mul(jc_1)) as isize)
                    + ai2_0
                        * *ch.offset(ik_1.wrapping_add(
                            idl1.wrapping_mul(jc_1.wrapping_sub(1 as libc::c_int as usize)),
                        ) as isize);
                ik_1 = ik_1.wrapping_add(1)
            }
            j_1 = (j_1 as usize).wrapping_add(2usize) as usize as usize;
            jc_1 = (jc_1 as usize).wrapping_sub(2usize) as usize as usize
        }
        while j_1 < ipph {
            iang = (iang as usize).wrapping_add(l) as usize as usize;
            if iang > ip {
                iang = (iang as usize).wrapping_sub(ip) as usize as usize
            }
            let war: f64 = *csarr.offset((2usize).wrapping_mul(iang) as isize);
            let wai: f64 = *csarr.offset(
                (2usize)
                    .wrapping_mul(iang)
                    .wrapping_add(1 as libc::c_int as usize) as isize,
            );
            let mut ik_2: usize = 0;
            while ik_2 < idl1 {
                *cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(l)) as isize) +=
                    war * *ch.offset(ik_2.wrapping_add(idl1.wrapping_mul(j_1)) as isize);
                *cc.offset(ik_2.wrapping_add(idl1.wrapping_mul(lc)) as isize) +=
                    wai * *ch.offset(ik_2.wrapping_add(idl1.wrapping_mul(jc_1)) as isize);
                ik_2 = ik_2.wrapping_add(1)
            }
            j_1 = j_1.wrapping_add(1);
            jc_1 = jc_1.wrapping_sub(1)
        }
        l = l.wrapping_add(1);
        lc = lc.wrapping_sub(1)
    }
    let mut j_2: usize = 1 as libc::c_int as usize;
    while j_2 < ipph {
        let mut ik_3: usize = 0;
        while ik_3 < idl1 {
            *ch.offset(ik_3.wrapping_add(idl1.wrapping_mul(0)) as isize) +=
                *ch.offset(ik_3.wrapping_add(idl1.wrapping_mul(j_2)) as isize);
            ik_3 = ik_3.wrapping_add(1)
        }
        j_2 = j_2.wrapping_add(1)
    }
    let mut j_3: usize = 1 as libc::c_int as usize;
    let mut jc_2: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while j_3 < ipph {
        // 124
        let mut k_2: usize = 0;
        while k_2 < l1 {
            *ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                    as isize,
            ) = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                    as isize,
            ) - *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                    as isize,
            );
            *ch.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                    as isize,
            ) = *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(j_3))))
                    as isize,
            ) + *cc.offset(
                (0usize).wrapping_add(ido.wrapping_mul(k_2.wrapping_add(l1.wrapping_mul(jc_2))))
                    as isize,
            );
            k_2 = k_2.wrapping_add(1)
        }
        j_3 = j_3.wrapping_add(1);
        jc_2 = jc_2.wrapping_sub(1)
    }
    if ido == 1 as libc::c_int as usize {
        return;
    }
    let mut j_4: usize = 1 as libc::c_int as usize;
    let mut jc_3: usize = ip.wrapping_sub(1 as libc::c_int as usize);
    while j_4 < ipph {
        // 127
        let mut k_3: usize = 0;
        while k_3 < l1 {
            let mut i_1: usize = 1 as libc::c_int as usize;
            while i_1 <= ido.wrapping_sub(2usize) {
                *ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ) = *cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ) - *cc.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                );
                *ch.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                ) = *cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ) + *cc.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                );
                *ch.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ) = *cc.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ) + *cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                );
                *ch.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                ) = *cc.offset(
                    i_1.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(j_4))))
                        as isize,
                ) - *cc.offset(
                    i_1.wrapping_add(ido.wrapping_mul(k_3.wrapping_add(l1.wrapping_mul(jc_3))))
                        as isize,
                );
                i_1 = (i_1 as usize).wrapping_add(2usize) as usize as usize
            }
            k_3 = k_3.wrapping_add(1)
        }
        j_4 = j_4.wrapping_add(1);
        jc_3 = jc_3.wrapping_sub(1)
    }
    // All in CH
    let mut j_5: usize = 1 as libc::c_int as usize;
    while j_5 < ip {
        let is: usize = j_5
            .wrapping_sub(1 as libc::c_int as usize)
            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize));
        let mut k_4: usize = 0;
        while k_4 < l1 {
            let mut idij: usize = is;
            let mut i_2: usize = 1 as libc::c_int as usize;
            while i_2 <= ido.wrapping_sub(2usize) {
                let t1: f64 = *ch.offset(
                    i_2.wrapping_add(ido.wrapping_mul(k_4.wrapping_add(l1.wrapping_mul(j_5))))
                        as isize,
                );
                let t2: f64 = *ch.offset(
                    i_2.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_4.wrapping_add(l1.wrapping_mul(j_5))))
                        as isize,
                );
                *ch.offset(
                    i_2.wrapping_add(ido.wrapping_mul(k_4.wrapping_add(l1.wrapping_mul(j_5))))
                        as isize,
                ) = *wa.offset(idij as isize) * t1
                    - *wa.offset(idij.wrapping_add(1 as libc::c_int as usize) as isize) * t2;
                *ch.offset(
                    i_2.wrapping_add(1 as libc::c_int as usize)
                        .wrapping_add(ido.wrapping_mul(k_4.wrapping_add(l1.wrapping_mul(j_5))))
                        as isize,
                ) = *wa.offset(idij as isize) * t2
                    + *wa.offset(idij.wrapping_add(1 as libc::c_int as usize) as isize) * t1;
                idij = (idij as usize).wrapping_add(2usize) as usize as usize;
                i_2 = (i_2 as usize).wrapping_add(2usize) as usize as usize
            }
            k_4 = k_4.wrapping_add(1)
        }
        j_5 = j_5.wrapping_add(1)
    }
}
unsafe fn copy_and_norm(c: *mut f64, p1: *mut f64, n: usize, fct: f64) {
    if p1 != c {
        if fct != 1.0f64 {
            let mut i: usize = 0;
            while i < n {
                *c.offset(i as isize) = fct * *p1.offset(i as isize);
                i = i.wrapping_add(1)
            }
        } else {
            libc::memcpy(
                c as *mut libc::c_void,
                p1 as *const libc::c_void,
                n.wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t,
            );
        }
    } else if fct != 1.0f64 {
        let mut i_0: usize = 0;
        while i_0 < n {
            *c.offset(i_0 as isize) *= fct;
            i_0 = i_0.wrapping_add(1)
        }
    };
}
unsafe fn rfftp_forward(plan: RFFTPPlan, c: *mut f64, fct: f64) -> libc::c_int {
    if (*plan).length == 1 as libc::c_int as usize {
        return 0 as libc::c_int;
    }
    let n: usize = (*plan).length;
    let mut l1: usize = n;
    let nf: usize = (*plan).nfct;
    let ch: *mut f64 =
        libc::malloc(n.wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t)
            as *mut f64;
    if ch.is_null() {
        return -(1 as libc::c_int);
    }
    let mut p1: *mut f64 = c;
    let mut p2: *mut f64 = ch;
    let mut k1: usize = 0;
    while k1 < nf {
        let k: usize = nf.wrapping_sub(k1).wrapping_sub(1 as libc::c_int as usize);
        let ip: usize = (*plan).fct[k as usize].fct;
        let ido: usize = n.wrapping_div(l1);
        l1 = (l1 as usize).wrapping_div(ip) as usize as usize;
        if ip == 4 as libc::c_int as usize {
            radf4(ido, l1, p1, p2, (*plan).fct[k as usize].tw);
        } else if ip == 2usize {
            radf2(ido, l1, p1, p2, (*plan).fct[k as usize].tw);
        } else if ip == 3 as libc::c_int as usize {
            radf3(ido, l1, p1, p2, (*plan).fct[k as usize].tw);
        } else if ip == 5 as libc::c_int as usize {
            radf5(ido, l1, p1, p2, (*plan).fct[k as usize].tw);
        } else {
            radfg(
                ido,
                ip,
                l1,
                p1,
                p2,
                (*plan).fct[k as usize].tw,
                (*plan).fct[k as usize].tws,
            );
            let tmp_: *mut f64 = p1;
            p1 = p2;
            p2 = tmp_
        }
        let tmp_0: *mut f64 = p1;
        p1 = p2;
        p2 = tmp_0;
        k1 = k1.wrapping_add(1)
    }
    copy_and_norm(c, p1, n, fct);
    libc::free(ch as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe fn rfftp_backward(plan: RFFTPPlan, c: *mut f64, fct: f64) -> libc::c_int {
    if (*plan).length == 1 as libc::c_int as usize {
        return 0 as libc::c_int;
    }
    let n: usize = (*plan).length;
    let mut l1: usize = 1 as libc::c_int as usize;
    let nf: usize = (*plan).nfct;
    let ch: *mut f64 =
        libc::malloc(n.wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t)
            as *mut f64;
    if ch.is_null() {
        return -(1 as libc::c_int);
    }
    let mut p1: *mut f64 = c;
    let mut p2: *mut f64 = ch;
    let mut k: usize = 0;
    while k < nf {
        let ip: usize = (*plan).fct[k as usize].fct;
        let ido: usize = n.wrapping_div(ip.wrapping_mul(l1));
        if ip == 4 as libc::c_int as usize {
            radb4(ido, l1, p1, p2, (*plan).fct[k as usize].tw);
        } else if ip == 2usize {
            radb2(ido, l1, p1, p2, (*plan).fct[k as usize].tw);
        } else if ip == 3 as libc::c_int as usize {
            radb3(ido, l1, p1, p2, (*plan).fct[k as usize].tw);
        } else if ip == 5 as libc::c_int as usize {
            radb5(ido, l1, p1, p2, (*plan).fct[k as usize].tw);
        } else {
            radbg(
                ido,
                ip,
                l1,
                p1,
                p2,
                (*plan).fct[k as usize].tw,
                (*plan).fct[k as usize].tws,
            );
        }
        let tmp_: *mut f64 = p1;
        p1 = p2;
        p2 = tmp_;
        l1 = (l1 as usize).wrapping_mul(ip) as usize as usize;
        k = k.wrapping_add(1)
    }
    copy_and_norm(c, p1, n, fct);
    libc::free(ch as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe fn rfftp_factorize(mut plan: RFFTPPlan) -> libc::c_int {
    let mut length: usize = (*plan).length;
    let mut nfct: usize = 0;
    while length.wrapping_rem(4 as libc::c_int as usize) == 0 {
        if nfct >= 25 as libc::c_int as usize {
            return -(1 as libc::c_int);
        }
        let fresh5 = nfct;
        nfct = nfct.wrapping_add(1);
        (*plan).fct[fresh5 as usize].fct = 4 as libc::c_int as usize;
        length >>= 2 as libc::c_int
    }
    if length.wrapping_rem(2usize) == 0 {
        length >>= 1 as libc::c_int;
        // factor 2 should be at the front of the factor list
        if nfct >= 25 as libc::c_int as usize {
            return -(1 as libc::c_int);
        }
        let fresh6 = nfct;
        nfct = nfct.wrapping_add(1);
        (*plan).fct[fresh6 as usize].fct = 2usize;
        let tmp_: usize = (*plan).fct[0].fct;
        (*plan).fct[0].fct = (*plan).fct[nfct.wrapping_sub(1 as libc::c_int as usize) as usize].fct;
        (*plan).fct[nfct.wrapping_sub(1 as libc::c_int as usize) as usize].fct = tmp_
    }
    let mut maxl: usize =
        (f64::sqrt(length as f64) as usize).wrapping_add(1 as libc::c_int as usize);
    let mut divisor: usize = 3 as libc::c_int as usize;
    while length > 1 as libc::c_int as usize && divisor < maxl {
        if length.wrapping_rem(divisor) == 0 {
            while length.wrapping_rem(divisor) == 0 {
                if nfct >= 25 as libc::c_int as usize {
                    return -(1 as libc::c_int);
                }
                let fresh7 = nfct;
                nfct = nfct.wrapping_add(1);
                (*plan).fct[fresh7 as usize].fct = divisor;
                length = (length as usize).wrapping_div(divisor) as usize as usize
            }
            maxl = (f64::sqrt(length as f64) as usize).wrapping_add(1 as libc::c_int as usize)
        }
        divisor = (divisor as usize).wrapping_add(2usize) as usize as usize
    }
    if length > 1 as libc::c_int as usize {
        let fresh8 = nfct;
        nfct = nfct.wrapping_add(1);
        (*plan).fct[fresh8 as usize].fct = length
    }
    (*plan).nfct = nfct;
    return 0 as libc::c_int;
}
unsafe fn rfftp_twsize(plan: RFFTPPlan) -> usize {
    let mut twsize: usize = 0;
    let mut l1: usize = 1 as libc::c_int as usize;
    let mut k: usize = 0;
    while k < (*plan).nfct {
        let ip: usize = (*plan).fct[k as usize].fct;
        let ido: usize = (*plan).length.wrapping_div(l1.wrapping_mul(ip));
        twsize = (twsize as usize).wrapping_add(
            ip.wrapping_sub(1 as libc::c_int as usize)
                .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize)),
        ) as usize as usize;
        if ip > 5 as libc::c_int as usize {
            twsize = (twsize as usize).wrapping_add((2usize).wrapping_mul(ip)) as usize as usize
        }
        l1 = (l1 as usize).wrapping_mul(ip) as usize as usize;
        k = k.wrapping_add(1)
    }
    return twsize;
}
#[inline(never)]
unsafe fn rfftp_comp_twiddle(mut plan: RFFTPPlan) -> libc::c_int {
    let length: usize = (*plan).length;
    let twid: *mut f64 =
        libc::malloc(2 * length as libc::size_t * std::mem::size_of::<f64>()) as *mut f64;
    if twid.is_null() {
        return -(1 as libc::c_int);
    }
    sincos_2pibyn_half(length, twid);
    let mut l1: usize = 1 as libc::c_int as usize;
    let mut ptr: *mut f64 = (*plan).mem;
    let mut k: usize = 0;
    while k < (*plan).nfct {
        let ip: usize = (*plan).fct[k as usize].fct;
        let ido: usize = length.wrapping_div(l1.wrapping_mul(ip));
        if k < (*plan).nfct.wrapping_sub(1 as libc::c_int as usize) {
            // last factor doesn't need twiddles
            (*plan).fct[k as usize].tw = ptr;
            ptr = ptr.offset(
                ip.wrapping_sub(1 as libc::c_int as usize)
                    .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize))
                    as isize,
            );
            let mut j: usize = 1 as libc::c_int as usize;
            while j < ip {
                let mut i: usize = 1 as libc::c_int as usize;
                while i
                    <= ido
                        .wrapping_sub(1 as libc::c_int as usize)
                        .wrapping_div(2usize)
                {
                    *(*plan).fct[k as usize].tw.offset(
                        j.wrapping_sub(1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize))
                            .wrapping_add((2usize).wrapping_mul(i))
                            .wrapping_sub(2usize) as isize,
                    ) = *twid
                        .offset((2usize).wrapping_mul(j).wrapping_mul(l1).wrapping_mul(i) as isize);
                    *(*plan).fct[k as usize].tw.offset(
                        j.wrapping_sub(1 as libc::c_int as usize)
                            .wrapping_mul(ido.wrapping_sub(1 as libc::c_int as usize))
                            .wrapping_add((2usize).wrapping_mul(i))
                            .wrapping_sub(1 as libc::c_int as usize)
                            as isize,
                    ) = *twid.offset(
                        (2usize)
                            .wrapping_mul(j)
                            .wrapping_mul(l1)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as usize)
                            as isize,
                    );
                    i = i.wrapping_add(1)
                }
                j = j.wrapping_add(1)
            }
        }
        if ip > 5 as libc::c_int as usize {
            // special factors required by *g functions
            (*plan).fct[k as usize].tws = ptr;
            ptr = ptr.offset((2usize).wrapping_mul(ip) as isize);
            *(*plan).fct[k as usize].tws.offset(0) = 1.0f64;
            *(*plan).fct[k as usize].tws.offset(1) = 0.0f64;
            let mut i_0: usize = 1 as libc::c_int as usize;
            while i_0 <= ip >> 1 as libc::c_int {
                *(*plan).fct[k as usize]
                    .tws
                    .offset((2usize).wrapping_mul(i_0) as isize) = *twid.offset(
                    (2usize)
                        .wrapping_mul(i_0)
                        .wrapping_mul(length.wrapping_div(ip)) as isize,
                );
                *(*plan).fct[k as usize].tws.offset(
                    (2usize)
                        .wrapping_mul(i_0)
                        .wrapping_add(1 as libc::c_int as usize) as isize,
                ) = *twid.offset(
                    (2usize)
                        .wrapping_mul(i_0)
                        .wrapping_mul(length.wrapping_div(ip))
                        .wrapping_add(1 as libc::c_int as usize) as isize,
                );
                *(*plan).fct[k as usize]
                    .tws
                    .offset((2usize).wrapping_mul(ip.wrapping_sub(i_0)) as isize) = *twid.offset(
                    (2usize)
                        .wrapping_mul(i_0)
                        .wrapping_mul(length.wrapping_div(ip)) as isize,
                );
                *(*plan).fct[k as usize].tws.offset(
                    (2usize)
                        .wrapping_mul(ip.wrapping_sub(i_0))
                        .wrapping_add(1 as libc::c_int as usize) as isize,
                ) = -*twid.offset(
                    (2usize)
                        .wrapping_mul(i_0)
                        .wrapping_mul(length.wrapping_div(ip))
                        .wrapping_add(1 as libc::c_int as usize) as isize,
                );
                i_0 = i_0.wrapping_add(1)
            }
        }
        l1 = (l1 as usize).wrapping_mul(ip) as usize as usize;
        k = k.wrapping_add(1)
    }
    libc::free(twid as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe fn make_rfftp_plan(length: usize) -> RFFTPPlan {
    if length == 0 {
        return 0 as RFFTPPlan;
    }
    let mut plan: RFFTPPlan =
        libc::malloc(std::mem::size_of::<RFFTPPlanI>() as libc::size_t) as *mut RFFTPPlanI;
    if plan.is_null() {
        return 0 as RFFTPPlan;
    }
    (*plan).length = length;
    (*plan).nfct = 0;
    (*plan).mem = 0 as *mut f64;
    let mut i: usize = 0;
    while i < 25 as libc::c_int as usize {
        (*plan).fct[i as usize] = RFFTPFctData {
            fct: 0,
            tw: 0 as *mut f64,
            tws: 0 as *mut f64,
        };
        i = i.wrapping_add(1)
    }
    if length == 1 as libc::c_int as usize {
        return plan;
    }
    if rfftp_factorize(plan) != 0 as libc::c_int {
        libc::free(plan as *mut libc::c_void);
        return 0 as RFFTPPlan;
    }
    let tws: usize = rfftp_twsize(plan);
    (*plan).mem =
        libc::malloc(tws.wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t)
            as *mut f64;
    if (*plan).mem.is_null() {
        libc::free(plan as *mut libc::c_void);
        return 0 as RFFTPPlan;
    }
    if rfftp_comp_twiddle(plan) != 0 as libc::c_int {
        libc::free((*plan).mem as *mut libc::c_void);
        (*plan).mem = 0 as *mut f64;
        libc::free(plan as *mut libc::c_void);
        return 0 as RFFTPPlan;
    }
    return plan;
}
#[inline(never)]
unsafe fn destroy_rfftp_plan(mut plan: RFFTPPlan) {
    libc::free((*plan).mem as *mut libc::c_void);
    (*plan).mem = 0 as *mut f64;
    libc::free(plan as *mut libc::c_void);
}
#[inline(never)]
unsafe fn make_fftblue_plan(length: usize) -> FFTBluePlan {
    let mut plan: FFTBluePlan =
        libc::malloc(std::mem::size_of::<FFTBluePlanI>() as libc::size_t) as *mut FFTBluePlanI;
    if plan.is_null() {
        return 0 as FFTBluePlan;
    }
    (*plan).n = length;
    (*plan).n2 = good_size(
        (*plan)
            .n
            .wrapping_mul(2usize)
            .wrapping_sub(1 as libc::c_int as usize),
    );
    (*plan).mem = libc::malloc(
        (2usize)
            .wrapping_mul((*plan).n)
            .wrapping_add((2usize).wrapping_mul((*plan).n2))
            .wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t,
    ) as *mut f64;
    if (*plan).mem.is_null() {
        libc::free(plan as *mut libc::c_void);
        return 0 as FFTBluePlan;
    }
    (*plan).bk = (*plan).mem;
    (*plan).bkf = (*plan).bk.offset((2usize).wrapping_mul((*plan).n) as isize);
    /* initialize b_k */
    let tmp: *mut f64 = libc::malloc(
        (4 as libc::c_int as usize)
            .wrapping_mul((*plan).n)
            .wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t,
    ) as *mut f64;
    if tmp.is_null() {
        libc::free((*plan).mem as *mut libc::c_void);
        (*plan).mem = 0 as *mut f64;
        libc::free(plan as *mut libc::c_void);
        return 0 as FFTBluePlan;
    }
    sincos_2pibyn((2usize).wrapping_mul((*plan).n), tmp);
    *(*plan).bk.offset(0) = 1 as libc::c_int as f64;
    *(*plan).bk.offset(1) = 0 as libc::c_int as f64;
    let mut coeff: usize = 0;
    let mut m: usize = 1 as libc::c_int as usize;
    while m < (*plan).n {
        coeff = (coeff as usize).wrapping_add(
            (2usize)
                .wrapping_mul(m)
                .wrapping_sub(1 as libc::c_int as usize),
        ) as usize as usize;
        if coeff >= (2usize).wrapping_mul((*plan).n) {
            coeff =
                (coeff as usize).wrapping_sub((2usize).wrapping_mul((*plan).n)) as usize as usize
        }
        *(*plan).bk.offset((2usize).wrapping_mul(m) as isize) =
            *tmp.offset((2usize).wrapping_mul(coeff) as isize);
        *(*plan).bk.offset(
            (2usize)
                .wrapping_mul(m)
                .wrapping_add(1 as libc::c_int as usize) as isize,
        ) = *tmp.offset(
            (2usize)
                .wrapping_mul(coeff)
                .wrapping_add(1 as libc::c_int as usize) as isize,
        );
        m = m.wrapping_add(1)
    }
    /* initialize the zero-padded, Fourier transformed b_k. Add normalisation. */
    let xn2: f64 = 1.0f64 / (*plan).n2 as f64;
    *(*plan).bkf.offset(0) = *(*plan).bk.offset(0) * xn2;
    *(*plan).bkf.offset(1) = *(*plan).bk.offset(1) * xn2;
    let mut m_0: usize = 2usize;
    while m_0 < (2usize).wrapping_mul((*plan).n) {
        let ref mut fresh9 = *(*plan)
            .bkf
            .offset((2usize).wrapping_mul((*plan).n2).wrapping_sub(m_0) as isize);
        *fresh9 = *(*plan).bk.offset(m_0 as isize) * xn2;
        *(*plan).bkf.offset(m_0 as isize) = *fresh9;
        let ref mut fresh10 = *(*plan).bkf.offset(
            (2usize)
                .wrapping_mul((*plan).n2)
                .wrapping_sub(m_0)
                .wrapping_add(1 as libc::c_int as usize) as isize,
        );
        *fresh10 = *(*plan)
            .bk
            .offset(m_0.wrapping_add(1 as libc::c_int as usize) as isize)
            * xn2;
        *(*plan)
            .bkf
            .offset(m_0.wrapping_add(1 as libc::c_int as usize) as isize) = *fresh10;
        m_0 = (m_0 as usize).wrapping_add(2usize) as usize as usize
    }
    let mut m_1: usize = (2usize).wrapping_mul((*plan).n);
    while m_1
        <= (2usize)
            .wrapping_mul((*plan).n2)
            .wrapping_sub((2usize).wrapping_mul((*plan).n))
            .wrapping_add(1 as libc::c_int as usize)
    {
        *(*plan).bkf.offset(m_1 as isize) = 0.0f64;
        m_1 = m_1.wrapping_add(1)
    }
    (*plan).plan = make_cfftp_plan((*plan).n2);
    if (*plan).plan.is_null() {
        libc::free(tmp as *mut libc::c_void);
        libc::free((*plan).mem as *mut libc::c_void);
        libc::free(plan as *mut libc::c_void);
        return 0 as FFTBluePlan;
    }
    if cfftp_forward((*plan).plan, (*plan).bkf, 1.0f64) != 0 as libc::c_int {
        libc::free(tmp as *mut libc::c_void);
        libc::free((*plan).mem as *mut libc::c_void);
        libc::free(plan as *mut libc::c_void);
        return 0 as FFTBluePlan;
    }
    libc::free(tmp as *mut libc::c_void);
    return plan;
}
#[inline(never)]
unsafe fn destroy_fftblue_plan(plan: FFTBluePlan) {
    libc::free((*plan).mem as *mut libc::c_void);
    destroy_cfftp_plan((*plan).plan);
    libc::free(plan as *mut libc::c_void);
}
#[inline(never)]
unsafe fn fftblue_fft(plan: FFTBluePlan, c: *mut f64, isign: libc::c_int, fct: f64) -> libc::c_int {
    let n: usize = (*plan).n;
    let n2: usize = (*plan).n2;
    let bk: *mut f64 = (*plan).bk;
    let bkf: *mut f64 = (*plan).bkf;
    let akf: *mut f64 = libc::malloc(
        (2usize)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t,
    ) as *mut f64;
    if akf.is_null() {
        return -(1 as libc::c_int);
    }
    /* initialize a_k and FFT it */
    if isign > 0 as libc::c_int {
        let mut m: usize = 0;
        while m < (2usize).wrapping_mul(n) {
            *akf.offset(m as isize) = *c.offset(m as isize) * *bk.offset(m as isize)
                - *c.offset(m.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *bk.offset(m.wrapping_add(1 as libc::c_int as usize) as isize);
            *akf.offset(m.wrapping_add(1 as libc::c_int as usize) as isize) = *c.offset(m as isize)
                * *bk.offset(m.wrapping_add(1 as libc::c_int as usize) as isize)
                + *c.offset(m.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *bk.offset(m as isize);
            m = (m as usize).wrapping_add(2usize) as usize as usize
        }
    } else {
        let mut m_0: usize = 0;
        while m_0 < (2usize).wrapping_mul(n) {
            *akf.offset(m_0 as isize) = *c.offset(m_0 as isize) * *bk.offset(m_0 as isize)
                + *c.offset(m_0.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *bk.offset(m_0.wrapping_add(1 as libc::c_int as usize) as isize);
            *akf.offset(m_0.wrapping_add(1 as libc::c_int as usize) as isize) = -*c
                .offset(m_0 as isize)
                * *bk.offset(m_0.wrapping_add(1 as libc::c_int as usize) as isize)
                + *c.offset(m_0.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *bk.offset(m_0 as isize);
            m_0 = (m_0 as usize).wrapping_add(2usize) as usize as usize
        }
    }
    let mut m_1: usize = (2usize).wrapping_mul(n);
    while m_1 < (2usize).wrapping_mul(n2) {
        *akf.offset(m_1 as isize) = 0 as libc::c_int as f64;
        m_1 = m_1.wrapping_add(1)
    }
    if cfftp_forward((*plan).plan, akf, fct) != 0 as libc::c_int {
        libc::free(akf as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    /* do the convolution */
    if isign > 0 as libc::c_int {
        let mut m_2: usize = 0;
        while m_2 < (2usize).wrapping_mul(n2) {
            let im: f64 = -*akf.offset(m_2 as isize)
                * *bkf.offset(m_2.wrapping_add(1 as libc::c_int as usize) as isize)
                + *akf.offset(m_2.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *bkf.offset(m_2 as isize);
            *akf.offset(m_2 as isize) = *akf.offset(m_2 as isize) * *bkf.offset(m_2 as isize)
                + *akf.offset(m_2.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *bkf.offset(m_2.wrapping_add(1 as libc::c_int as usize) as isize);
            *akf.offset(m_2.wrapping_add(1 as libc::c_int as usize) as isize) = im;
            m_2 = (m_2 as usize).wrapping_add(2usize) as usize as usize
        }
    } else {
        let mut m_3: usize = 0;
        while m_3 < (2usize).wrapping_mul(n2) {
            let im_0: f64 = *akf.offset(m_3 as isize)
                * *bkf.offset(m_3.wrapping_add(1 as libc::c_int as usize) as isize)
                + *akf.offset(m_3.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *bkf.offset(m_3 as isize);
            *akf.offset(m_3 as isize) = *akf.offset(m_3 as isize) * *bkf.offset(m_3 as isize)
                - *akf.offset(m_3.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *bkf.offset(m_3.wrapping_add(1 as libc::c_int as usize) as isize);
            *akf.offset(m_3.wrapping_add(1 as libc::c_int as usize) as isize) = im_0;
            m_3 = (m_3 as usize).wrapping_add(2usize) as usize as usize
        }
    }
    /* inverse FFT */
    if cfftp_backward((*plan).plan, akf, 1.0f64) != 0 as libc::c_int {
        libc::free(akf as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    /* multiply by b_k */
    if isign > 0 as libc::c_int {
        let mut m_4: usize = 0; /* fudge factor that appears to give good overall performance */
        while m_4 < (2usize).wrapping_mul(n) {
            *c.offset(m_4 as isize) = *bk.offset(m_4 as isize) * *akf.offset(m_4 as isize)
                - *bk.offset(m_4.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *akf.offset(m_4.wrapping_add(1 as libc::c_int as usize) as isize);
            *c.offset(m_4.wrapping_add(1 as libc::c_int as usize) as isize) = *bk
                .offset(m_4.wrapping_add(1 as libc::c_int as usize) as isize)
                * *akf.offset(m_4 as isize)
                + *bk.offset(m_4 as isize)
                    * *akf.offset(m_4.wrapping_add(1 as libc::c_int as usize) as isize);
            m_4 = (m_4 as usize).wrapping_add(2usize) as usize as usize
        }
    } else {
        let mut m_5: usize = 0;
        while m_5 < (2usize).wrapping_mul(n) {
            *c.offset(m_5 as isize) = *bk.offset(m_5 as isize) * *akf.offset(m_5 as isize)
                + *bk.offset(m_5.wrapping_add(1 as libc::c_int as usize) as isize)
                    * *akf.offset(m_5.wrapping_add(1 as libc::c_int as usize) as isize);
            *c.offset(m_5.wrapping_add(1 as libc::c_int as usize) as isize) = -*bk
                .offset(m_5.wrapping_add(1 as libc::c_int as usize) as isize)
                * *akf.offset(m_5 as isize)
                + *bk.offset(m_5 as isize)
                    * *akf.offset(m_5.wrapping_add(1 as libc::c_int as usize) as isize);
            m_5 = (m_5 as usize).wrapping_add(2usize) as usize as usize
        }
    }
    libc::free(akf as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe fn cfftblue_backward(plan: FFTBluePlan, c: *mut f64, fct: f64) -> libc::c_int {
    return fftblue_fft(plan, c, 1 as libc::c_int, fct);
}
unsafe fn cfftblue_forward(plan: FFTBluePlan, c: *mut f64, fct: f64) -> libc::c_int {
    return fftblue_fft(plan, c, -(1 as libc::c_int), fct);
}
unsafe fn rfftblue_backward(plan: FFTBluePlan, c: *mut f64, fct: f64) -> libc::c_int {
    let n: usize = (*plan).n;
    let tmp: *mut f64 = libc::malloc(
        (2usize)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t,
    ) as *mut f64;
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    *tmp.offset(0) = *c.offset(0);
    *tmp.offset(1) = 0.0f64;
    libc::memcpy(
        tmp.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        c.offset(1) as *const libc::c_void,
        n.wrapping_sub(1 as libc::c_int as usize)
            .wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t,
    );
    if n & 1 as libc::c_int as usize == 0 {
        *tmp.offset(n.wrapping_add(1 as libc::c_int as usize) as isize) = 0.0f64
    }
    let mut m: usize = 2usize;
    while m < n {
        *tmp.offset((2usize).wrapping_mul(n).wrapping_sub(m) as isize) = *tmp.offset(m as isize);
        *tmp.offset(
            (2usize)
                .wrapping_mul(n)
                .wrapping_sub(m)
                .wrapping_add(1 as libc::c_int as usize) as isize,
        ) = -*tmp.offset(m.wrapping_add(1 as libc::c_int as usize) as isize);
        m = (m as usize).wrapping_add(2usize) as usize as usize
    }
    if fftblue_fft(plan, tmp, 1 as libc::c_int, fct) != 0 as libc::c_int {
        libc::free(tmp as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    let mut m_0: usize = 0;
    while m_0 < n {
        *c.offset(m_0 as isize) = *tmp.offset((2usize).wrapping_mul(m_0) as isize);
        m_0 = m_0.wrapping_add(1)
    }
    libc::free(tmp as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe fn rfftblue_forward(plan: FFTBluePlan, c: *mut f64, fct: f64) -> libc::c_int {
    let n: usize = (*plan).n;
    let tmp: *mut f64 =
        libc::malloc(2 * n as libc::size_t * std::mem::size_of::<f64>()) as *mut f64;
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    let mut m: usize = 0;
    while m < n {
        *tmp.offset((2usize).wrapping_mul(m) as isize) = *c.offset(m as isize);
        *tmp.offset(
            (2usize)
                .wrapping_mul(m)
                .wrapping_add(1 as libc::c_int as usize) as isize,
        ) = 0.0f64;
        m = m.wrapping_add(1)
    }
    if fftblue_fft(plan, tmp, -(1 as libc::c_int), fct) != 0 as libc::c_int {
        libc::free(tmp as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    *c.offset(0) = *tmp.offset(0);
    libc::memcpy(
        c.offset(1) as *mut libc::c_void,
        tmp.offset(2 as libc::c_int as isize) as *const libc::c_void,
        n.wrapping_sub(1 as libc::c_int as usize)
            .wrapping_mul(::std::mem::size_of::<f64>() as usize) as libc::size_t,
    );
    libc::free(tmp as *mut libc::c_void);
    return 0 as libc::c_int;
}

pub unsafe fn make_cfft_plan(length: usize) -> CFFTPlan {
    if length == 0 {
        return 0 as CFFTPlan;
    }
    let mut plan: CFFTPlan =
        libc::malloc(::std::mem::size_of::<CFFTPlanI>() as libc::size_t) as *mut CFFTPlanI;
    if plan.is_null() {
        return 0 as CFFTPlan;
    }
    (*plan).blueplan = 0 as FFTBluePlan;
    (*plan).packplan = 0 as CFFTPPlan;
    if length < 50 as libc::c_int as usize
        || largest_prime_factor(length) as f64 <= f64::sqrt(length as f64)
    {
        (*plan).packplan = make_cfftp_plan(length);
        if (*plan).packplan.is_null() {
            libc::free(plan as *mut libc::c_void);
            return 0 as CFFTPlan;
        }
        return plan;
    }
    let comp1: f64 = cost_guess(length);
    let mut comp2: f64 = 2 as libc::c_int as f64
        * cost_guess(good_size(
            (2usize)
                .wrapping_mul(length)
                .wrapping_sub(1 as libc::c_int as usize),
        ));
    comp2 *= 1.5f64;
    if comp2 < comp1 {
        // use Bluestein
        (*plan).blueplan = make_fftblue_plan(length);
        if (*plan).blueplan.is_null() {
            libc::free(plan as *mut libc::c_void);
            return 0 as CFFTPlan;
        }
    } else {
        (*plan).packplan = make_cfftp_plan(length);
        if (*plan).packplan.is_null() {
            libc::free(plan as *mut libc::c_void);
            return 0 as CFFTPlan;
        }
    }
    return plan;
}

pub unsafe fn destroy_cfft_plan(plan: CFFTPlan) {
    if !(*plan).blueplan.is_null() {
        destroy_fftblue_plan((*plan).blueplan);
    }
    if !(*plan).packplan.is_null() {
        destroy_cfftp_plan((*plan).packplan);
    }
    libc::free(plan as *mut libc::c_void);
}

pub unsafe fn cfft_backward(plan: CFFTPlan, c: *mut f64, fct: f64) -> libc::c_int {
    if !(*plan).packplan.is_null() {
        return cfftp_backward((*plan).packplan, c, fct);
    }
    // if (plan->blueplan)
    return cfftblue_backward((*plan).blueplan, c, fct);
}

pub unsafe fn cfft_forward(plan: CFFTPlan, c: *mut f64, fct: f64) -> libc::c_int {
    if !(*plan).packplan.is_null() {
        return cfftp_forward((*plan).packplan, c, fct);
    }
    // if (plan->blueplan)
    return cfftblue_forward((*plan).blueplan, c, fct); /* fudge factor that appears to give good overall performance */
}

pub unsafe fn make_rfft_plan(length: usize) -> RFFTPlan {
    if length == 0 {
        return 0 as RFFTPlan;
    }
    let mut plan: RFFTPlan = libc::malloc(
        (1 as libc::c_int as libc::size_t)
            .wrapping_mul(::std::mem::size_of::<RFFTPlanI>() as libc::size_t),
    ) as *mut RFFTPlanI;
    if plan.is_null() {
        return 0 as RFFTPlan;
    }
    (*plan).blueplan = 0 as FFTBluePlan;
    (*plan).packplan = 0 as RFFTPPlan;
    if length < 50 as libc::c_int as usize
        || largest_prime_factor(length) as f64 <= f64::sqrt(length as f64)
    {
        (*plan).packplan = make_rfftp_plan(length);
        if (*plan).packplan.is_null() {
            libc::free(plan as *mut libc::c_void);
            return 0 as RFFTPlan;
        }
        return plan;
    }
    let comp1: f64 = 0.5f64 * cost_guess(length);
    let mut comp2: f64 = 2 as libc::c_int as f64
        * cost_guess(good_size(
            (2usize)
                .wrapping_mul(length)
                .wrapping_sub(1 as libc::c_int as usize),
        ));
    comp2 *= 1.5f64;
    if comp2 < comp1 {
        // use Bluestein
        (*plan).blueplan = make_fftblue_plan(length);
        if (*plan).blueplan.is_null() {
            libc::free(plan as *mut libc::c_void);
            return 0 as RFFTPlan;
        }
    } else {
        (*plan).packplan = make_rfftp_plan(length);
        if (*plan).packplan.is_null() {
            libc::free(plan as *mut libc::c_void);
            return 0 as RFFTPlan;
        }
    }
    return plan;
}

pub unsafe fn destroy_rfft_plan(plan: RFFTPlan) {
    if !(*plan).blueplan.is_null() {
        destroy_fftblue_plan((*plan).blueplan);
    }
    if !(*plan).packplan.is_null() {
        destroy_rfftp_plan((*plan).packplan);
    }
    libc::free(plan as *mut libc::c_void);
}

pub unsafe fn rfft_length(plan: RFFTPlan) -> usize {
    if !(*plan).packplan.is_null() {
        return (*(*plan).packplan).length;
    }
    return (*(*plan).blueplan).n;
}

pub unsafe fn cfft_length(plan: CFFTPlan) -> usize {
    if !(*plan).packplan.is_null() {
        return (*(*plan).packplan).length;
    }
    return (*(*plan).blueplan).n;
}

pub unsafe fn rfft_backward(plan: RFFTPlan, c: *mut f64, fct: f64) -> libc::c_int {
    if !(*plan).packplan.is_null() {
        return rfftp_backward((*plan).packplan, c, fct);
    } else {
        // if (plan->blueplan)
        return rfftblue_backward((*plan).blueplan, c, fct);
    };
}
/*
 * This file is part of pocketfft.
 * Licensed under a 3-clause BSD style license - see LICENSE.md
 */
/* ! \file pocketfft.h
 *  Public interface of the pocketfft library
 *
 *  Copyright (C) 2008-2018 Max-Planck-Society
 *  \author Martin Reinecke
 */

pub unsafe fn rfft_forward(plan: RFFTPlan, c: *mut f64, fct: f64) -> libc::c_int {
    if !(*plan).packplan.is_null() {
        return rfftp_forward((*plan).packplan, c, fct);
    } else {
        // if (plan->blueplan)
        return rfftblue_forward((*plan).blueplan, c, fct);
    };
}
