use super::channel::fluid_channel_get_cc;
use super::channel::Channel;
use super::conv::fluid_concave;
use super::conv::fluid_convex;
use super::voice::fluid_voice_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mod {
    pub dest: libc::c_uchar,
    pub src1: libc::c_uchar,
    pub flags1: libc::c_uchar,
    pub src2: libc::c_uchar,
    pub flags2: libc::c_uchar,
    pub amount: f64,
    pub next: *mut Mod,
}
pub type ModFlags = u32;
pub const FLUID_MOD_CC: ModFlags = 16;
pub const FLUID_MOD_GC: ModFlags = 0;
pub const FLUID_MOD_SWITCH: ModFlags = 12;
pub const FLUID_MOD_LINEAR: ModFlags = 0;
pub const FLUID_MOD_UNIPOLAR: ModFlags = 0;
pub const FLUID_MOD_NEGATIVE: ModFlags = 1;
pub const FLUID_MOD_POSITIVE: ModFlags = 0;
pub type ModSrc = u32;
pub const FLUID_MOD_VELOCITY: ModSrc = 2;
pub type GenType = u32;
pub const GEN_FILTERFC: GenType = 8;

pub fn fluid_mod_clone(mut mod_0: &mut Mod, src: &Mod) {
    mod_0.dest = src.dest;
    mod_0.src1 = src.src1;
    mod_0.flags1 = src.flags1;
    mod_0.src2 = src.src2;
    mod_0.flags2 = src.flags2;
    mod_0.amount = src.amount;
}

pub unsafe fn fluid_mod_set_source1(
    mut mod_0: *mut Mod,
    src: i32,
    flags: i32,
) {
    (*mod_0).src1 = src as libc::c_uchar;
    (*mod_0).flags1 = flags as libc::c_uchar;
}

pub unsafe fn fluid_mod_set_source2(
    mut mod_0: *mut Mod,
    src: i32,
    flags: i32,
) {
    (*mod_0).src2 = src as libc::c_uchar;
    (*mod_0).flags2 = flags as libc::c_uchar;
}

pub unsafe fn fluid_mod_set_dest(mut mod_0: *mut Mod, dest: i32) {
    (*mod_0).dest = dest as libc::c_uchar;
}

pub unsafe fn fluid_mod_set_amount(mut mod_0: *mut Mod, amount: f64) {
    (*mod_0).amount = amount;
}

pub unsafe fn fluid_mod_get_dest(mod_0: *mut Mod) -> i32 {
    return (*mod_0).dest as i32;
}

pub unsafe fn fluid_mod_get_value(
    mod_0: *mut Mod,
    chan: *mut Channel,
    voice: *mut fluid_voice_t,
) -> f32 {
    let mut v1: f32;
    let mut v2: f32 = 1.0f64 as f32;
    let mut range1: f32 = 127.0f64 as f32;
    let range2: f32 = 127.0f64 as f32;
    if chan.is_null() {
        return 0.0f32;
    }
    if (*mod_0).src2 as i32 == FLUID_MOD_VELOCITY as i32
        && (*mod_0).src1 as i32 == FLUID_MOD_VELOCITY as i32
        && (*mod_0).flags1 as i32
            == FLUID_MOD_GC as i32
                | FLUID_MOD_UNIPOLAR as i32
                | FLUID_MOD_NEGATIVE as i32
                | FLUID_MOD_LINEAR as i32
        && (*mod_0).flags2 as i32
            == FLUID_MOD_GC as i32
                | FLUID_MOD_UNIPOLAR as i32
                | FLUID_MOD_POSITIVE as i32
                | FLUID_MOD_SWITCH as i32
        && (*mod_0).dest as i32 == GEN_FILTERFC as i32
    {
        return 0 as i32 as f32;
    }
    if (*mod_0).src1 as i32 > 0 as i32 {
        if (*mod_0).flags1 as i32 & FLUID_MOD_CC as i32 != 0 {
            v1 = fluid_channel_get_cc(chan.as_ref().unwrap(), (*mod_0).src1 as i32) as f32
        } else {
            match (*mod_0).src1 as i32 {
                0 => v1 = range1,
                2 => v1 = (*voice).vel as f32,
                3 => v1 = (*voice).key as f32,
                10 => v1 = (*chan).key_pressure[(*voice).key as usize] as f32,
                13 => v1 = (*chan).channel_pressure as f32,
                14 => {
                    v1 = (*chan).pitch_bend as f32;
                    range1 = 0x4000 as i32 as f32
                }
                16 => v1 = (*chan).pitch_wheel_sensitivity as f32,
                _ => v1 = 0.0f64 as f32,
            }
        }
        match (*mod_0).flags1 as i32 & 0xf as i32 {
            0 => v1 /= range1,
            1 => v1 = 1.0f32 - v1 / range1,
            2 => v1 = -1.0f32 + 2.0f32 * v1 / range1,
            3 => v1 = 1.0f32 - 2.0f32 * v1 / range1,
            4 => v1 = fluid_concave(v1),
            5 => v1 = fluid_concave(127 as i32 as libc::c_float - v1),
            6 => {
                v1 = if v1 > 64 as i32 as libc::c_float {
                    fluid_concave(
                        2 as i32 as libc::c_float
                            * (v1 - 64 as i32 as libc::c_float),
                    )
                } else {
                    -fluid_concave(
                        2 as i32 as libc::c_float
                            * (64 as i32 as libc::c_float - v1),
                    )
                }
            }
            7 => {
                v1 = if v1 > 64 as i32 as libc::c_float {
                    -fluid_concave(
                        2 as i32 as libc::c_float
                            * (v1 - 64 as i32 as libc::c_float),
                    )
                } else {
                    fluid_concave(
                        2 as i32 as libc::c_float
                            * (64 as i32 as libc::c_float - v1),
                    )
                }
            }
            8 => v1 = fluid_convex(v1),
            9 => v1 = fluid_convex(127 as i32 as libc::c_float - v1),
            10 => {
                v1 = if v1 > 64 as i32 as libc::c_float {
                    fluid_convex(
                        2 as i32 as libc::c_float
                            * (v1 - 64 as i32 as libc::c_float),
                    )
                } else {
                    -fluid_convex(
                        2 as i32 as libc::c_float
                            * (64 as i32 as libc::c_float - v1),
                    )
                }
            }
            11 => {
                v1 = if v1 > 64 as i32 as libc::c_float {
                    -fluid_convex(
                        2 as i32 as libc::c_float
                            * (v1 - 64 as i32 as libc::c_float),
                    )
                } else {
                    fluid_convex(
                        2 as i32 as libc::c_float
                            * (64 as i32 as libc::c_float - v1),
                    )
                }
            }
            12 => {
                v1 = if v1 >= 64 as i32 as libc::c_float {
                    1.0f32
                } else {
                    0.0f32
                }
            }
            13 => {
                v1 = if v1 >= 64 as i32 as libc::c_float {
                    0.0f32
                } else {
                    1.0f32
                }
            }
            14 => {
                v1 = if v1 >= 64 as i32 as libc::c_float {
                    1.0f32
                } else {
                    -1.0f32
                }
            }
            15 => {
                v1 = if v1 >= 64 as i32 as libc::c_float {
                    -1.0f32
                } else {
                    1.0f32
                }
            }
            _ => {}
        }
    } else {
        return 0.0f64 as f32;
    }
    if v1 == 0.0f32 {
        return 0.0f32;
    }
    if (*mod_0).src2 as i32 > 0 as i32 {
        if (*mod_0).flags2 as i32 & FLUID_MOD_CC as i32 != 0 {
            v2 = fluid_channel_get_cc(chan.as_ref().unwrap(), (*mod_0).src2 as i32) as f32
        } else {
            match (*mod_0).src2 as i32 {
                0 => v2 = range2,
                2 => v2 = (*voice).vel as f32,
                3 => v2 = (*voice).key as f32,
                10 => v2 = (*chan).key_pressure[(*voice).key as usize] as f32,
                13 => v2 = (*chan).channel_pressure as f32,
                14 => v2 = (*chan).pitch_bend as f32,
                16 => v2 = (*chan).pitch_wheel_sensitivity as f32,
                _ => v1 = 0.0f32,
            }
        }
        match (*mod_0).flags2 as i32 & 0xf as i32 {
            0 => v2 /= range2,
            1 => v2 = 1.0f32 - v2 / range2,
            2 => v2 = -1.0f32 + 2.0f32 * v2 / range2,
            3 => v2 = -1.0f32 + 2.0f32 * v2 / range2,
            4 => v2 = fluid_concave(v2),
            5 => v2 = fluid_concave(127 as i32 as libc::c_float - v2),
            6 => {
                v2 = if v2 > 64 as i32 as libc::c_float {
                    fluid_concave(
                        2 as i32 as libc::c_float
                            * (v2 - 64 as i32 as libc::c_float),
                    )
                } else {
                    -fluid_concave(
                        2 as i32 as libc::c_float
                            * (64 as i32 as libc::c_float - v2),
                    )
                }
            }
            7 => {
                v2 = if v2 > 64 as i32 as libc::c_float {
                    -fluid_concave(
                        2 as i32 as libc::c_float
                            * (v2 - 64 as i32 as libc::c_float),
                    )
                } else {
                    fluid_concave(
                        2 as i32 as libc::c_float
                            * (64 as i32 as libc::c_float - v2),
                    )
                }
            }
            8 => v2 = fluid_convex(v2),
            9 => v2 = 1.0f32 - fluid_convex(v2),
            10 => {
                v2 = if v2 > 64 as i32 as libc::c_float {
                    -fluid_convex(
                        2 as i32 as libc::c_float
                            * (v2 - 64 as i32 as libc::c_float),
                    )
                } else {
                    fluid_convex(
                        2 as i32 as libc::c_float
                            * (64 as i32 as libc::c_float - v2),
                    )
                }
            }
            11 => {
                v2 = if v2 > 64 as i32 as libc::c_float {
                    -fluid_convex(
                        2 as i32 as libc::c_float
                            * (v2 - 64 as i32 as libc::c_float),
                    )
                } else {
                    fluid_convex(
                        2 as i32 as libc::c_float
                            * (64 as i32 as libc::c_float - v2),
                    )
                }
            }
            12 => {
                v2 = if v2 >= 64 as i32 as libc::c_float {
                    1.0f32
                } else {
                    0.0f32
                }
            }
            13 => {
                v2 = if v2 >= 64 as i32 as libc::c_float {
                    0.0f32
                } else {
                    1.0f32
                }
            }
            14 => {
                v2 = if v2 >= 64 as i32 as libc::c_float {
                    1.0f32
                } else {
                    -1.0f32
                }
            }
            15 => {
                v2 = if v2 >= 64 as i32 as libc::c_float {
                    -1.0f32
                } else {
                    1.0f32
                }
            }
            _ => {}
        }
    } else {
        v2 = 1.0f32
    }
    return (*mod_0).amount as f32 * v1 * v2;
}

pub unsafe fn fluid_mod_new() -> *mut Mod {
    let mod_0: *mut Mod =
        libc::malloc(::std::mem::size_of::<Mod>() as libc::size_t) as *mut Mod;
    if mod_0.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut Mod;
    }
    return mod_0;
}

pub unsafe fn fluid_mod_delete(mod_0: *mut Mod) {
    libc::free(mod_0 as *mut libc::c_void);
}

pub fn fluid_mod_test_identity(
    mod1: &Mod,
    mod2: &Mod,
) -> i32 {
    if mod1.dest as i32 != mod2.dest as i32 {
        return 0 as i32;
    }
    if mod1.src1 as i32 != mod2.src1 as i32 {
        return 0 as i32;
    }
    if mod1.src2 as i32 != mod2.src2 as i32 {
        return 0 as i32;
    }
    if mod1.flags1 as i32 != mod2.flags1 as i32 {
        return 0 as i32;
    }
    if mod1.flags2 as i32 != mod2.flags2 as i32 {
        return 0 as i32;
    }
    return 1 as i32;
}
