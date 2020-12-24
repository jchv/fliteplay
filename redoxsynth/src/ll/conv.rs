static mut CT2HZ_TAB: [f32; 1200] = [0.; 1200];
static mut CB2AMP_TAB: [f32; 961] = [0.; 961];
static mut ATTEN2AMP_TAB: [f32; 1441] = [0.; 1441];
static mut CONCAVE_TAB: [f32; 128] = [0.; 128];
static mut CONVEX_TAB: [f32; 128] = [0.; 128];
static mut PAN_TAB: [f32; 1002] = [0.; 1002];

pub fn fluid_conversion_config() {
    unsafe {
        for i in 0..1200 {
            CT2HZ_TAB[i] = f64::powf(2.0f64, i as f64 / 1200.0f64) as f32;
        }
        for i in 0..961 {
            CB2AMP_TAB[i] = f64::powf(10.0f64, i as f64 / -200.0f64) as f32;
        }
        for i in 0..1441 {
            ATTEN2AMP_TAB[i] = f64::powf(10.0f64, i as f64 / -200.0f64) as f32;
        }
        CONCAVE_TAB[0] = 0.0f32;
        CONCAVE_TAB[127] = 1.0f32;
        CONVEX_TAB[0] = 0 as i32 as f32;
        CONVEX_TAB[127] = 1.0f32;
        let mut x: f64;
        for i in 1..127 {
            x = -20.0f64 / 96.0f64 * f64::ln((i * i) as f64 / (127.0f64 * 127.0f64)) / f64::ln(10.0f64);
            CONVEX_TAB[i] = (1.0f64 - x) as f32;
            CONCAVE_TAB[127 - i] = x as f32;
        }
        x = 3.141592654f64 / 2.0f64 / (1002 as i32 as f64 - 1.0f64);
        for i in 0..1002 {
            PAN_TAB[i] = f64::sin(i as f64 * x) as f32;
        }
    }
}

pub fn fluid_ct2hz_real(cents: f32) -> f32 {
    return unsafe {
        if cents < 0f32 {
            1.0f32
        } else if cents < 900f32 {
            6.875f32 * CT2HZ_TAB [cents as usize + 300]
        } else if cents < 2100f32 {
            13.75f32 * CT2HZ_TAB [cents as usize - 900]
        } else if cents < 3300f32 {
            27.5f32 * CT2HZ_TAB [cents as usize - 2100]
        } else if cents < 4500f32 {
            55.0f32 * CT2HZ_TAB [cents as usize - 3300]
        } else if cents < 5700f32 {
            110.0f32 * CT2HZ_TAB [cents as usize - 4500]
        } else if cents < 6900f32 {
            220.0f32 * CT2HZ_TAB [cents as usize - 5700]
        } else if cents < 8100f32 {
            440.0f32 * CT2HZ_TAB [cents as usize - 6900]
        } else if cents < 9300f32 {
            880.0f32 * CT2HZ_TAB [cents as usize - 8100]
        } else if cents < 10500f32 {
            1760.0f32 * CT2HZ_TAB [cents as usize - 9300]
        } else if cents < 11700f32 {
            3520.0f32 * CT2HZ_TAB [cents as usize - 10500]
        } else if cents < 12900f32 {
            7040.0f32 * CT2HZ_TAB [cents as usize - 11700]
        } else if cents < 14100f32 {
            14080.0f32 * CT2HZ_TAB [cents as usize - 12900]
        } else {
            1.0f32
        }
    };
}

pub fn fluid_ct2hz(mut cents: f32) -> f32 {
    if cents >= 13500f32 {
        cents = 13500f32
    } else if cents < 1500f32 {
        cents = 1500f32
    }
    return fluid_ct2hz_real(cents);
}

pub fn fluid_cb2amp(cb: f32) -> f32 {
    return if cb < 0f32 {
        1.0f32
    } else if cb >= 961f32 {
        0.0f32
    } else  {
        unsafe { CB2AMP_TAB[cb as i32 as usize] }
    };
}

pub fn fluid_atten2amp(atten: f32) -> f32 {
    return if atten < 0f32 {
        1.0f32
    } else if atten >= 1441f32 {
        0.0f32
    } else {
        unsafe { ATTEN2AMP_TAB[atten as i32 as usize] }
    };
}

pub fn fluid_tc2sec_delay(mut tc: f32) -> f32 {
    if tc <= -32768.0f32 {
        return 0.0f32;
    }
    if tc < -12000.0f32 {
        tc = -12000.0f32
    }
    if tc > 5000.0f32 {
        tc = 5000.0f32
    }
    return f64::powf(2.0f64, tc as f64 / 1200.0f64) as f32;
}

pub fn fluid_tc2sec_attack(mut tc: f32) -> f32 {
    if tc <= -32768.0f32 {
        return 0.0f32;
    }
    if tc < -12000.0f32 {
        tc = -12000.0f32
    }
    if tc > 8000.0f32 {
        tc = 8000.0f32
    }
    return f64::powf(2.0f64, tc as f64 / 1200.0f64) as f32;
}

pub fn fluid_tc2sec(tc: f32) -> f32 {
    return f64::powf(2.0f64, tc as f64 / 1200.0f64) as f32;
}

pub fn fluid_tc2sec_release(mut tc: f32) -> f32 {
    if tc <= -32768.0f32 {
        return 0.0f32;
    }
    if tc < -12000.0f32 {
        tc = -12000.0f32
    }
    if tc > 8000.0f32 {
        tc = 8000.0f32
    }
    return f64::powf(2.0f64, tc as f64 / 1200.0f64) as f32;
}

pub fn fluid_act2hz(c: f32) -> f32 {
    return (8.176f64 * f64::powf(2.0f64, c as f64 / 1200.0f64)) as f32;
}

pub fn fluid_pan(mut c: f32, left: i32) -> f32 {
    if left != 0 {
        c = -c
    }

    return if c < -500f32 {
        0f32
    } else if c > 500f32 {
        1f32
    } else {
        unsafe { PAN_TAB[(c + 500f32) as usize] }
    };
}

pub fn fluid_concave(val: f32) -> f32 {
    return if val < 0f32 {
        0f32
    } else if val > 127f32 {
        1f32
    } else {
        unsafe { CONCAVE_TAB[val as i32 as usize] }
    };
}

pub fn fluid_convex(val: f32) -> f32 {
    return if val < 0f32 {
        0f32
    } else if val > 127f32 {
        1f32
    } else {
        unsafe { CONVEX_TAB[val as i32 as usize] }
    };
}
