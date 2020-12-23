#[no_mangle]
pub static mut fluid_ct2hz_tab: [f32; 1200] = [0.; 1200];
#[no_mangle]
pub static mut fluid_cb2amp_tab: [f32; 961] = [0.; 961];
#[no_mangle]
pub static mut fluid_atten2amp_tab: [f32; 1441] = [0.; 1441];
#[no_mangle]
pub static mut fluid_posbp_tab: [f32; 128] = [0.; 128];
#[no_mangle]
pub static mut fluid_concave_tab: [f32; 128] = [0.; 128];
#[no_mangle]
pub static mut fluid_convex_tab: [f32; 128] = [0.; 128];
#[no_mangle]
pub static mut fluid_pan_tab: [f32; 1002] = [0.; 1002];
#[no_mangle]
pub unsafe extern "C" fn fluid_conversion_config() {
    let mut i: libc::c_int;
    let mut x: f64;
    i = 0 as libc::c_int;
    while i < 1200 as libc::c_int {
        fluid_ct2hz_tab[i as usize] =
            f64::powf(2.0f64, i as f64 / 1200.0f64) as f32;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 961 as libc::c_int {
        fluid_cb2amp_tab[i as usize] =
            f64::powf(10.0f64, i as f64 / -200.0f64) as f32;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 1441 as libc::c_int {
        fluid_atten2amp_tab[i as usize] =
            f64::powf(10.0f64, i as f64 / -200.0f64) as f32;
        i += 1
    }
    fluid_concave_tab[0 as libc::c_int as usize] = 0.0f64 as f32;
    fluid_concave_tab[127 as libc::c_int as usize] = 1.0f64 as f32;
    fluid_convex_tab[0 as libc::c_int as usize] = 0 as libc::c_int as f32;
    fluid_convex_tab[127 as libc::c_int as usize] = 1.0f64 as f32;
    i = 1 as libc::c_int;
    while i < 127 as libc::c_int {
        x = -20.0f64 / 96.0f64 * f64::ln((i * i) as f64 / (127.0f64 * 127.0f64))
            / f64::ln(10.0f64);
        fluid_convex_tab[i as usize] = (1.0f64 - x) as f32;
        fluid_concave_tab[(127 as libc::c_int - i) as usize] = x as f32;
        i += 1
    }
    x = 3.141592654f64 / 2.0f64 / (1002 as libc::c_int as f64 - 1.0f64);
    i = 0 as libc::c_int;
    while i < 1002 as libc::c_int {
        fluid_pan_tab[i as usize] = f64::sin(i as f64 * x) as f32;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_ct2hz_real(cents: f32) -> f32 {
    if cents < 0 as libc::c_int as libc::c_float {
        return 1.0f64 as f32;
    } else if cents < 900 as libc::c_int as libc::c_float {
        return 6.875f64 as f32
            * fluid_ct2hz_tab
                [(cents + 300 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 2100 as libc::c_int as libc::c_float {
        return 13.75f64 as f32
            * fluid_ct2hz_tab
                [(cents - 900 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 3300 as libc::c_int as libc::c_float {
        return 27.5f64 as f32
            * fluid_ct2hz_tab
                [(cents - 2100 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 4500 as libc::c_int as libc::c_float {
        return 55.0f64 as f32
            * fluid_ct2hz_tab
                [(cents - 3300 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 5700 as libc::c_int as libc::c_float {
        return 110.0f64 as f32
            * fluid_ct2hz_tab
                [(cents - 4500 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 6900 as libc::c_int as libc::c_float {
        return 220.0f64 as f32
            * fluid_ct2hz_tab
                [(cents - 5700 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 8100 as libc::c_int as libc::c_float {
        return 440.0f64 as f32
            * fluid_ct2hz_tab
                [(cents - 6900 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 9300 as libc::c_int as libc::c_float {
        return 880.0f64 as f32
            * fluid_ct2hz_tab
                [(cents - 8100 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 10500 as libc::c_int as libc::c_float {
        return 1760.0f64 as f32
            * fluid_ct2hz_tab
                [(cents - 9300 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 11700 as libc::c_int as libc::c_float {
        return 3520.0f64 as f32
            * fluid_ct2hz_tab
                [(cents - 10500 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 12900 as libc::c_int as libc::c_float {
        return 7040.0f64 as f32
            * fluid_ct2hz_tab
                [(cents - 11700 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else if cents < 14100 as libc::c_int as libc::c_float {
        return 14080.0f64 as f32
            * fluid_ct2hz_tab
                [(cents - 12900 as libc::c_int as libc::c_float) as libc::c_int as usize];
    } else {
        return 1.0f64 as f32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_ct2hz(mut cents: f32) -> f32 {
    if cents >= 13500 as libc::c_int as libc::c_float {
        cents = 13500 as libc::c_int as f32
    } else if cents < 1500 as libc::c_int as libc::c_float {
        cents = 1500 as libc::c_int as f32
    }
    return fluid_ct2hz_real(cents);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_cb2amp(cb: f32) -> f32 {
    if cb < 0 as libc::c_int as libc::c_float {
        return 1.0f64 as f32;
    }
    if cb >= 961 as libc::c_int as libc::c_float {
        return 0.0f64 as f32;
    }
    return fluid_cb2amp_tab[cb as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn fluid_atten2amp(atten: f32) -> f32 {
    if atten < 0 as libc::c_int as libc::c_float {
        return 1.0f64 as f32;
    } else if atten >= 1441 as libc::c_int as libc::c_float {
        return 0.0f64 as f32;
    } else {
        return fluid_atten2amp_tab[atten as libc::c_int as usize];
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tc2sec_delay(mut tc: f32) -> f32 {
    if tc <= -32768.0f32 {
        return 0.0f32;
    }
    if (tc as f64) < -12000.0f64 {
        tc = -12000.0f32
    }
    if tc > 5000.0f32 {
        tc = 5000.0f32
    }
    return f64::powf(2.0f64, tc as f64 / 1200.0f64) as f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tc2sec_attack(mut tc: f32) -> f32 {
    if tc as f64 <= -32768.0f64 {
        return 0.0f64 as f32;
    }
    if (tc as f64) < -12000.0f64 {
        tc = -12000.0f64 as f32
    }
    if tc as f64 > 8000.0f64 {
        tc = 8000.0f64 as f32
    }
    return f64::powf(2.0f64, tc as f64 / 1200.0f64) as f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tc2sec(tc: f32) -> f32 {
    return f64::powf(2.0f64, tc as f64 / 1200.0f64) as f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tc2sec_release(mut tc: f32) -> f32 {
    if tc as f64 <= -32768.0f64 {
        return 0.0f64 as f32;
    }
    if (tc as f64) < -12000.0f64 {
        tc = -12000.0f64 as f32
    }
    if tc as f64 > 8000.0f64 {
        tc = 8000.0f64 as f32
    }
    return f64::powf(2.0f64, tc as f64 / 1200.0f64) as f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_act2hz(c: f32) -> f32 {
    return (8.176f64 * f64::powf(2.0f64, c as f64 / 1200.0f64)) as f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_hz2ct(f: f32) -> f32 {
    return (6900 as libc::c_int as f64
        + 1200 as libc::c_int as f64 * f64::ln(f as f64 / 440.0f64)
            / f64::ln(2.0f64)) as f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_pan(mut c: f32, left: libc::c_int) -> f32 {
    if left != 0 {
        c = -c
    }
    if c < -(500 as libc::c_int) as libc::c_float {
        return 0.0f64 as f32;
    } else if c > 500 as libc::c_int as libc::c_float {
        return 1.0f64 as f32;
    } else {
        return fluid_pan_tab[(c + 500 as libc::c_int as libc::c_float) as libc::c_int as usize];
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_concave(val: f32) -> f32 {
    if val < 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int as f32;
    } else {
        if val > 127 as libc::c_int as libc::c_float {
            return 1 as libc::c_int as f32;
        }
    }
    return fluid_concave_tab[val as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn fluid_convex(val: f32) -> f32 {
    if val < 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int as f32;
    } else {
        if val > 127 as libc::c_int as libc::c_float {
            return 1 as libc::c_int as f32;
        }
    }
    return fluid_convex_tab[val as libc::c_int as usize];
}
