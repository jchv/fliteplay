#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log10(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}

pub type fluid_real_t = libc::c_float;

#[no_mangle]
pub static mut fluid_ct2hz_tab: [fluid_real_t; 1200] = [0.; 1200];
#[no_mangle]
pub static mut fluid_cb2amp_tab: [fluid_real_t; 961] = [0.; 961];
#[no_mangle]
pub static mut fluid_atten2amp_tab: [fluid_real_t; 1441] = [0.; 1441];
#[no_mangle]
pub static mut fluid_posbp_tab: [fluid_real_t; 128] = [0.; 128];
#[no_mangle]
pub static mut fluid_concave_tab: [fluid_real_t; 128] = [0.; 128];
#[no_mangle]
pub static mut fluid_convex_tab: [fluid_real_t; 128] = [0.; 128];
#[no_mangle]
pub static mut fluid_pan_tab: [fluid_real_t; 1002] = [0.; 1002];

#[no_mangle]
pub unsafe extern "C" fn fluid_conversion_config() {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < 1200 as libc::c_int {
        fluid_ct2hz_tab[i as usize] = pow(2.0f64, i as libc::c_double / 1200.0f64) as fluid_real_t;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 961 as libc::c_int {
        fluid_cb2amp_tab[i as usize] =
            pow(10.0f64, i as libc::c_double / -200.0f64) as fluid_real_t;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 1441 as libc::c_int {
        fluid_atten2amp_tab[i as usize] =
            pow(10.0f64, i as libc::c_double / -200.0f64) as fluid_real_t;
        i += 1
    }
    fluid_concave_tab[0 as libc::c_int as usize] = 0.0f64 as fluid_real_t;
    fluid_concave_tab[127 as libc::c_int as usize] = 1.0f64 as fluid_real_t;

    fluid_convex_tab[0 as libc::c_int as usize] = 0 as libc::c_int as fluid_real_t;
    fluid_convex_tab[127 as libc::c_int as usize] = 1.0f64 as fluid_real_t;
    x = log10(128.0f64 / 127.0f64);
    i = 1 as libc::c_int;
    while i < 127 as libc::c_int {
        x = -20.0f64 / 96.0f64 * log((i * i) as libc::c_double / (127.0f64 * 127.0f64))
            / log(10.0f64);
        fluid_convex_tab[i as usize] = (1.0f64 - x) as fluid_real_t;
        fluid_concave_tab[(127 as libc::c_int - i) as usize] = x as fluid_real_t;
        i += 1
    }

    x = 3.141592654f64 / 2.0f64 / (1002 as libc::c_int as libc::c_double - 1.0f64);
    i = 0 as libc::c_int;
    while i < 1002 as libc::c_int {
        fluid_pan_tab[i as usize] = sin(i as libc::c_double * x) as fluid_real_t;
        i += 1
    }
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ct2hz_real(mut cents: fluid_real_t) -> fluid_real_t {
    if cents < 0 as libc::c_int as libc::c_float {
        return 1.0f64 as fluid_real_t;
    } else if cents < 900 as libc::c_int as libc::c_float {
        return 6.875f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents + 300 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 2100 as libc::c_int as libc::c_float {
        return 13.75f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 900 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 3300 as libc::c_int as libc::c_float {
        return 27.5f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 2100 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 4500 as libc::c_int as libc::c_float {
        return 55.0f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 3300 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 5700 as libc::c_int as libc::c_float {
        return 110.0f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 4500 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 6900 as libc::c_int as libc::c_float {
        return 220.0f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 5700 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 8100 as libc::c_int as libc::c_float {
        return 440.0f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 6900 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 9300 as libc::c_int as libc::c_float {
        return 880.0f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 8100 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 10500 as libc::c_int as libc::c_float {
        return 1760.0f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 9300 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 11700 as libc::c_int as libc::c_float {
        return 3520.0f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 10500 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 12900 as libc::c_int as libc::c_float {
        return 7040.0f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 11700 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 14100 as libc::c_int as libc::c_float {
        return 14080.0f64 as fluid_real_t
            * fluid_ct2hz_tab
                [(cents - 12900 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else {
        return 1.0f64 as fluid_real_t;
    };
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ct2hz(mut cents: fluid_real_t) -> fluid_real_t {
    if cents >= 13500 as libc::c_int as libc::c_float {
        cents = 13500 as libc::c_int as fluid_real_t
    } else if cents < 1500 as libc::c_int as libc::c_float {
        cents = 1500 as libc::c_int as fluid_real_t
    }
    return fluid_ct2hz_real(cents);
}

#[no_mangle]
pub unsafe extern "C" fn fluid_cb2amp(mut cb: fluid_real_t) -> fluid_real_t {
    if cb < 0 as libc::c_int as libc::c_float {
        return 1.0f64 as fluid_real_t;
    }
    if cb >= 961 as libc::c_int as libc::c_float {
        return 0.0f64 as fluid_real_t;
    }
    return fluid_cb2amp_tab[cb as libc::c_int as usize];
}

#[no_mangle]
pub unsafe extern "C" fn fluid_atten2amp(mut atten: fluid_real_t) -> fluid_real_t {
    if atten < 0 as libc::c_int as libc::c_float {
        return 1.0f64 as fluid_real_t;
    } else if atten >= 1441 as libc::c_int as libc::c_float {
        return 0.0f64 as fluid_real_t;
    } else {
        return fluid_atten2amp_tab[atten as libc::c_int as usize];
    };
}

#[no_mangle]
pub unsafe extern "C" fn fluid_tc2sec_delay(mut tc: fluid_real_t) -> fluid_real_t {
    if tc <= -32768.0f32 {
        return 0.0f32;
    }
    if (tc as libc::c_double) < -12000.0f64 {
        tc = -12000.0f32
    }
    if tc > 5000.0f32 {
        tc = 5000.0f32
    }
    return pow(2.0f64, tc as libc::c_double / 1200.0f64) as fluid_real_t;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_tc2sec_attack(mut tc: fluid_real_t) -> fluid_real_t {
    if tc as libc::c_double <= -32768.0f64 {
        return 0.0f64 as fluid_real_t;
    }
    if (tc as libc::c_double) < -12000.0f64 {
        tc = -12000.0f64 as fluid_real_t
    }
    if tc as libc::c_double > 8000.0f64 {
        tc = 8000.0f64 as fluid_real_t
    }
    return pow(2.0f64, tc as libc::c_double / 1200.0f64) as fluid_real_t;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_tc2sec(mut tc: fluid_real_t) -> fluid_real_t {
    return pow(2.0f64, tc as libc::c_double / 1200.0f64) as fluid_real_t;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_tc2sec_release(mut tc: fluid_real_t) -> fluid_real_t {
    if tc as libc::c_double <= -32768.0f64 {
        return 0.0f64 as fluid_real_t;
    }
    if (tc as libc::c_double) < -12000.0f64 {
        tc = -12000.0f64 as fluid_real_t
    }
    if tc as libc::c_double > 8000.0f64 {
        tc = 8000.0f64 as fluid_real_t
    }
    return pow(2.0f64, tc as libc::c_double / 1200.0f64) as fluid_real_t;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_act2hz(mut c: fluid_real_t) -> fluid_real_t {
    return (8.176f64 * pow(2.0f64, c as libc::c_double / 1200.0f64)) as fluid_real_t;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_hz2ct(mut f: fluid_real_t) -> fluid_real_t {
    return (6900 as libc::c_int as libc::c_double
        + 1200 as libc::c_int as libc::c_double * log(f as libc::c_double / 440.0f64) / log(2.0f64))
        as fluid_real_t;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_pan(mut c: fluid_real_t, mut left: libc::c_int) -> fluid_real_t {
    if left != 0 {
        c = -c
    }
    if c < -(500 as libc::c_int) as libc::c_float {
        return 0.0f64 as fluid_real_t;
    } else if c > 500 as libc::c_int as libc::c_float {
        return 1.0f64 as fluid_real_t;
    } else {
        return fluid_pan_tab[(c + 500 as libc::c_int as libc::c_float) as libc::c_int as usize];
    };
}

#[no_mangle]
pub unsafe extern "C" fn fluid_concave(mut val: fluid_real_t) -> fluid_real_t {
    if val < 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int as fluid_real_t;
    } else {
        if val > 127 as libc::c_int as libc::c_float {
            return 1 as libc::c_int as fluid_real_t;
        }
    }
    return fluid_concave_tab[val as libc::c_int as usize];
}

#[no_mangle]
pub unsafe extern "C" fn fluid_convex(mut val: fluid_real_t) -> fluid_real_t {
    if val < 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int as fluid_real_t;
    } else {
        if val > 127 as libc::c_int as libc::c_float {
            return 1 as libc::c_int as fluid_real_t;
        }
    }
    return fluid_convex_tab[val as libc::c_int as usize];
}
