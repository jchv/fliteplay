#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]
use crate::channel::fluid_channel_get_cc;
use crate::channel::fluid_channel_t;
use crate::conv::fluid_concave;
use crate::conv::fluid_convex;
use crate::voice::fluid_voice_t;
pub type fluid_real_t = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_mod_t {
    pub dest: libc::c_uchar,
    pub src1: libc::c_uchar,
    pub flags1: libc::c_uchar,
    pub src2: libc::c_uchar,
    pub flags2: libc::c_uchar,
    pub amount: libc::c_double,
    pub next: *mut fluid_mod_t,
}
pub type fluid_mod_flags = libc::c_uint;
pub const FLUID_MOD_CC: fluid_mod_flags = 16;
pub const FLUID_MOD_GC: fluid_mod_flags = 0;
pub const FLUID_MOD_SWITCH: fluid_mod_flags = 12;
pub const FLUID_MOD_LINEAR: fluid_mod_flags = 0;
pub const FLUID_MOD_UNIPOLAR: fluid_mod_flags = 0;
pub const FLUID_MOD_NEGATIVE: fluid_mod_flags = 1;
pub const FLUID_MOD_POSITIVE: fluid_mod_flags = 0;
pub type fluid_mod_src = libc::c_uint;
pub const FLUID_MOD_VELOCITY: fluid_mod_src = 2;
pub type fluid_gen_type = libc::c_uint;
pub const GEN_FILTERFC: fluid_gen_type = 8;
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_clone(mut mod_0: *mut fluid_mod_t, mut src: *mut fluid_mod_t) {
    (*mod_0).dest = (*src).dest;
    (*mod_0).src1 = (*src).src1;
    (*mod_0).flags1 = (*src).flags1;
    (*mod_0).src2 = (*src).src2;
    (*mod_0).flags2 = (*src).flags2;
    (*mod_0).amount = (*src).amount;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_set_source1(
    mut mod_0: *mut fluid_mod_t,
    mut src: libc::c_int,
    mut flags: libc::c_int,
) {
    (*mod_0).src1 = src as libc::c_uchar;
    (*mod_0).flags1 = flags as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_set_source2(
    mut mod_0: *mut fluid_mod_t,
    mut src: libc::c_int,
    mut flags: libc::c_int,
) {
    (*mod_0).src2 = src as libc::c_uchar;
    (*mod_0).flags2 = flags as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_set_dest(mut mod_0: *mut fluid_mod_t, mut dest: libc::c_int) {
    (*mod_0).dest = dest as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_set_amount(
    mut mod_0: *mut fluid_mod_t,
    mut amount: libc::c_double,
) {
    (*mod_0).amount = amount;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_source1(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).src1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_flags1(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).flags1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_source2(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).src2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_flags2(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).flags2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_dest(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).dest as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_amount(mut mod_0: *mut fluid_mod_t) -> libc::c_double {
    return (*mod_0).amount as fluid_real_t as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_value(
    mut mod_0: *mut fluid_mod_t,
    mut chan: *mut fluid_channel_t,
    mut voice: *mut fluid_voice_t,
) -> fluid_real_t {
    let mut v1: fluid_real_t;
    let mut v2: fluid_real_t = 1.0f64 as fluid_real_t;
    let mut range1: fluid_real_t = 127.0f64 as fluid_real_t;
    let mut range2: fluid_real_t = 127.0f64 as fluid_real_t;
    if chan.is_null() {
        return 0.0f32;
    }
    if (*mod_0).src2 as libc::c_int == FLUID_MOD_VELOCITY as libc::c_int
        && (*mod_0).src1 as libc::c_int == FLUID_MOD_VELOCITY as libc::c_int
        && (*mod_0).flags1 as libc::c_int
            == FLUID_MOD_GC as libc::c_int
                | FLUID_MOD_UNIPOLAR as libc::c_int
                | FLUID_MOD_NEGATIVE as libc::c_int
                | FLUID_MOD_LINEAR as libc::c_int
        && (*mod_0).flags2 as libc::c_int
            == FLUID_MOD_GC as libc::c_int
                | FLUID_MOD_UNIPOLAR as libc::c_int
                | FLUID_MOD_POSITIVE as libc::c_int
                | FLUID_MOD_SWITCH as libc::c_int
        && (*mod_0).dest as libc::c_int == GEN_FILTERFC as libc::c_int
    {
        return 0 as libc::c_int as fluid_real_t;
    }
    if (*mod_0).src1 as libc::c_int > 0 as libc::c_int {
        if (*mod_0).flags1 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0 {
            v1 = fluid_channel_get_cc(chan, (*mod_0).src1 as libc::c_int) as fluid_real_t
        } else {
            match (*mod_0).src1 as libc::c_int {
                0 => v1 = range1,
                2 => v1 = (*voice).vel as fluid_real_t,
                3 => v1 = (*voice).key as fluid_real_t,
                10 => v1 = (*chan).key_pressure[(*voice).key as usize] as fluid_real_t,
                13 => v1 = (*chan).channel_pressure as fluid_real_t,
                14 => {
                    v1 = (*chan).pitch_bend as fluid_real_t;
                    range1 = 0x4000 as libc::c_int as fluid_real_t
                }
                16 => v1 = (*chan).pitch_wheel_sensitivity as fluid_real_t,
                _ => v1 = 0.0f64 as fluid_real_t,
            }
        }
        match (*mod_0).flags1 as libc::c_int & 0xf as libc::c_int {
            0 => v1 /= range1,
            1 => v1 = 1.0f32 - v1 / range1,
            2 => v1 = -1.0f32 + 2.0f32 * v1 / range1,
            3 => v1 = 1.0f32 - 2.0f32 * v1 / range1,
            4 => v1 = fluid_concave(v1),
            5 => v1 = fluid_concave(127 as libc::c_int as libc::c_float - v1),
            6 => {
                v1 = if v1 > 64 as libc::c_int as libc::c_float {
                    fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (v1 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    -fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v1),
                    )
                }
            }
            7 => {
                v1 = if v1 > 64 as libc::c_int as libc::c_float {
                    -fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (v1 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v1),
                    )
                }
            }
            8 => v1 = fluid_convex(v1),
            9 => v1 = fluid_convex(127 as libc::c_int as libc::c_float - v1),
            10 => {
                v1 = if v1 > 64 as libc::c_int as libc::c_float {
                    fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (v1 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    -fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v1),
                    )
                }
            }
            11 => {
                v1 = if v1 > 64 as libc::c_int as libc::c_float {
                    -fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (v1 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v1),
                    )
                }
            }
            12 => {
                v1 = if v1 >= 64 as libc::c_int as libc::c_float {
                    1.0f32
                } else {
                    0.0f32
                }
            }
            13 => {
                v1 = if v1 >= 64 as libc::c_int as libc::c_float {
                    0.0f32
                } else {
                    1.0f32
                }
            }
            14 => {
                v1 = if v1 >= 64 as libc::c_int as libc::c_float {
                    1.0f32
                } else {
                    -1.0f32
                }
            }
            15 => {
                v1 = if v1 >= 64 as libc::c_int as libc::c_float {
                    -1.0f32
                } else {
                    1.0f32
                }
            }
            _ => {}
        }
    } else {
        return 0.0f64 as fluid_real_t;
    }
    if v1 == 0.0f32 {
        return 0.0f32;
    }
    if (*mod_0).src2 as libc::c_int > 0 as libc::c_int {
        if (*mod_0).flags2 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0 {
            v2 = fluid_channel_get_cc(chan, (*mod_0).src2 as libc::c_int) as fluid_real_t
        } else {
            match (*mod_0).src2 as libc::c_int {
                0 => v2 = range2,
                2 => v2 = (*voice).vel as fluid_real_t,
                3 => v2 = (*voice).key as fluid_real_t,
                10 => v2 = (*chan).key_pressure[(*voice).key as usize] as fluid_real_t,
                13 => v2 = (*chan).channel_pressure as fluid_real_t,
                14 => v2 = (*chan).pitch_bend as fluid_real_t,
                16 => v2 = (*chan).pitch_wheel_sensitivity as fluid_real_t,
                _ => v1 = 0.0f32,
            }
        }
        match (*mod_0).flags2 as libc::c_int & 0xf as libc::c_int {
            0 => v2 /= range2,
            1 => v2 = 1.0f32 - v2 / range2,
            2 => v2 = -1.0f32 + 2.0f32 * v2 / range2,
            3 => v2 = -1.0f32 + 2.0f32 * v2 / range2,
            4 => v2 = fluid_concave(v2),
            5 => v2 = fluid_concave(127 as libc::c_int as libc::c_float - v2),
            6 => {
                v2 = if v2 > 64 as libc::c_int as libc::c_float {
                    fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (v2 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    -fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v2),
                    )
                }
            }
            7 => {
                v2 = if v2 > 64 as libc::c_int as libc::c_float {
                    -fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (v2 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v2),
                    )
                }
            }
            8 => v2 = fluid_convex(v2),
            9 => v2 = 1.0f32 - fluid_convex(v2),
            10 => {
                v2 = if v2 > 64 as libc::c_int as libc::c_float {
                    -fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (v2 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v2),
                    )
                }
            }
            11 => {
                v2 = if v2 > 64 as libc::c_int as libc::c_float {
                    -fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (v2 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v2),
                    )
                }
            }
            12 => {
                v2 = if v2 >= 64 as libc::c_int as libc::c_float {
                    1.0f32
                } else {
                    0.0f32
                }
            }
            13 => {
                v2 = if v2 >= 64 as libc::c_int as libc::c_float {
                    0.0f32
                } else {
                    1.0f32
                }
            }
            14 => {
                v2 = if v2 >= 64 as libc::c_int as libc::c_float {
                    1.0f32
                } else {
                    -1.0f32
                }
            }
            15 => {
                v2 = if v2 >= 64 as libc::c_int as libc::c_float {
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
    return (*mod_0).amount as fluid_real_t * v1 * v2;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_new() -> *mut fluid_mod_t {
    let mut mod_0: *mut fluid_mod_t =
        libc::malloc(::std::mem::size_of::<fluid_mod_t>() as libc::size_t) as *mut fluid_mod_t;
    if mod_0.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_mod_t;
    }
    return mod_0;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_delete(mut mod_0: *mut fluid_mod_t) {
    libc::free(mod_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_test_identity(
    mut mod1: *mut fluid_mod_t,
    mut mod2: *mut fluid_mod_t,
) -> libc::c_int {
    if (*mod1).dest as libc::c_int != (*mod2).dest as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*mod1).src1 as libc::c_int != (*mod2).src1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*mod1).src2 as libc::c_int != (*mod2).src2 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*mod1).flags1 as libc::c_int != (*mod2).flags1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*mod1).flags2 as libc::c_int != (*mod2).flags2 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
