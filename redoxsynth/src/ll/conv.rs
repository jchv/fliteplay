pub static mut FLUID_CT2HZ_TAB: [f32; 1200] = [0.; 1200];
pub static mut FLUID_CB2AMP_TAB: [f32; 961] = [0.; 961];
pub static mut FLUID_ATTEN2AMP_TAB: [f32; 1441] = [0.; 1441];
pub static mut FLUID_CONCAVE_TAB: [f32; 128] = [0.; 128];
pub static mut FLUID_CONVEX_TAB: [f32; 128] = [0.; 128];
pub static mut FLUID_PAN_TAB: [f32; 1002] = [0.; 1002];

pub fn fluid_conversion_config() {
    unsafe {
        for i in 0..1200 {
            FLUID_CT2HZ_TAB[i] = f64::powf(2.0f64, i as f64 / 1200.0f64) as f32;
        }
        for i in 0..961 {
            FLUID_CB2AMP_TAB[i] = f64::powf(10.0f64, i as f64 / -200.0f64) as f32;
        }
        for i in 0..1441 {
            FLUID_ATTEN2AMP_TAB[i] = f64::powf(10.0f64, i as f64 / -200.0f64) as f32;
        }
        FLUID_CONCAVE_TAB[0] = 0.0f64 as f32;
        FLUID_CONCAVE_TAB[127] = 1.0f64 as f32;
        FLUID_CONVEX_TAB[0] = 0 as i32 as f32;
        FLUID_CONVEX_TAB[127] = 1.0f64 as f32;
        let mut x: f64;
        for i in 1..127 {
            x = -20.0f64 / 96.0f64 * f64::ln((i * i) as f64 / (127.0f64 * 127.0f64)) / f64::ln(10.0f64);
            FLUID_CONVEX_TAB[i] = (1.0f64 - x) as f32;
            FLUID_CONCAVE_TAB[127 - i] = x as f32;
        }
        x = 3.141592654f64 / 2.0f64 / (1002 as i32 as f64 - 1.0f64);
        for i in 0..1002 {
            FLUID_PAN_TAB[i] = f64::sin(i as f64 * x) as f32;
        }
    }
}

pub fn fluid_ct2hz_real(cents: f32) -> f32 {
    unsafe {
        if cents < 0 as i32 as libc::c_float {
            return 1.0f64 as f32;
        } else if cents < 900 as i32 as libc::c_float {
            return 6.875f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents + 300 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 2100 as i32 as libc::c_float {
            return 13.75f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 900 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 3300 as i32 as libc::c_float {
            return 27.5f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 2100 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 4500 as i32 as libc::c_float {
            return 55.0f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 3300 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 5700 as i32 as libc::c_float {
            return 110.0f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 4500 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 6900 as i32 as libc::c_float {
            return 220.0f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 5700 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 8100 as i32 as libc::c_float {
            return 440.0f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 6900 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 9300 as i32 as libc::c_float {
            return 880.0f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 8100 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 10500 as i32 as libc::c_float {
            return 1760.0f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 9300 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 11700 as i32 as libc::c_float {
            return 3520.0f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 10500 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 12900 as i32 as libc::c_float {
            return 7040.0f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 11700 as i32 as libc::c_float) as i32 as usize];
        } else if cents < 14100 as i32 as libc::c_float {
            return 14080.0f64 as f32
                * FLUID_CT2HZ_TAB
                    [(cents - 12900 as i32 as libc::c_float) as i32 as usize];
        } else {
            return 1.0f64 as f32;
        };
    }
}

pub fn fluid_ct2hz(mut cents: f32) -> f32 {
    if cents >= 13500 as i32 as libc::c_float {
        cents = 13500 as i32 as f32
    } else if cents < 1500 as i32 as libc::c_float {
        cents = 1500 as i32 as f32
    }
    return fluid_ct2hz_real(cents);
}

pub fn fluid_cb2amp(cb: f32) -> f32 {
    unsafe {
        if cb < 0 as i32 as libc::c_float {
            return 1.0f64 as f32;
        }
        if cb >= 961 as i32 as libc::c_float {
            return 0.0f64 as f32;
        }
        return FLUID_CB2AMP_TAB[cb as i32 as usize];
    }
}

pub fn fluid_atten2amp(atten: f32) -> f32 {
    unsafe {
        if atten < 0 as i32 as libc::c_float {
            return 1.0f64 as f32;
        } else if atten >= 1441 as i32 as libc::c_float {
            return 0.0f64 as f32;
        } else {
            return FLUID_ATTEN2AMP_TAB[atten as i32 as usize];
        };
    }
}

pub fn fluid_tc2sec_delay(mut tc: f32) -> f32 {
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

pub fn fluid_tc2sec_attack(mut tc: f32) -> f32 {
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

pub fn fluid_tc2sec(tc: f32) -> f32 {
    return f64::powf(2.0f64, tc as f64 / 1200.0f64) as f32;
}

pub fn fluid_tc2sec_release(mut tc: f32) -> f32 {
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

pub fn fluid_act2hz(c: f32) -> f32 {
    return (8.176f64 * f64::powf(2.0f64, c as f64 / 1200.0f64)) as f32;
}

pub fn fluid_pan(mut c: f32, left: i32) -> f32 {
    unsafe {
        if left != 0 {
            c = -c
        }
        if c < -(500 as i32) as libc::c_float {
            return 0.0f64 as f32;
        } else if c > 500 as i32 as libc::c_float {
            return 1.0f64 as f32;
        } else {
            return FLUID_PAN_TAB[(c + 500 as i32 as libc::c_float) as i32 as usize];
        };
    }
}

pub fn fluid_concave(val: f32) -> f32 {
    unsafe {
        if val < 0 as i32 as libc::c_float {
            return 0 as i32 as f32;
        } else {
            if val > 127 as i32 as libc::c_float {
                return 1 as i32 as f32;
            }
        }
        return FLUID_CONCAVE_TAB[val as i32 as usize];
    }
}

pub fn fluid_convex(val: f32) -> f32 {
    unsafe {
        if val < 0 as i32 as libc::c_float {
            return 0 as i32 as f32;
        } else {
            if val > 127 as i32 as libc::c_float {
                return 1 as i32 as f32;
            }
        }
        return FLUID_CONVEX_TAB[val as i32 as usize];
    }
}
