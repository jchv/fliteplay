#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]
pub type fluid_real_t = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_revmodel_t {
    pub roomsize: fluid_real_t,
    pub damp: fluid_real_t,
    pub wet: fluid_real_t,
    pub wet1: fluid_real_t,
    pub wet2: fluid_real_t,
    pub width: fluid_real_t,
    pub gain: fluid_real_t,
    pub combL: [fluid_comb; 8],
    pub combR: [fluid_comb; 8],
    pub allpassL: [fluid_allpass; 4],
    pub allpassR: [fluid_allpass; 4],
    pub bufcombL1: [fluid_real_t; 1116],
    pub bufcombR1: [fluid_real_t; 1139],
    pub bufcombL2: [fluid_real_t; 1188],
    pub bufcombR2: [fluid_real_t; 1211],
    pub bufcombL3: [fluid_real_t; 1277],
    pub bufcombR3: [fluid_real_t; 1300],
    pub bufcombL4: [fluid_real_t; 1356],
    pub bufcombR4: [fluid_real_t; 1379],
    pub bufcombL5: [fluid_real_t; 1422],
    pub bufcombR5: [fluid_real_t; 1445],
    pub bufcombL6: [fluid_real_t; 1491],
    pub bufcombR6: [fluid_real_t; 1514],
    pub bufcombL7: [fluid_real_t; 1557],
    pub bufcombR7: [fluid_real_t; 1580],
    pub bufcombL8: [fluid_real_t; 1617],
    pub bufcombR8: [fluid_real_t; 1640],
    pub bufallpassL1: [fluid_real_t; 556],
    pub bufallpassR1: [fluid_real_t; 579],
    pub bufallpassL2: [fluid_real_t; 441],
    pub bufallpassR2: [fluid_real_t; 464],
    pub bufallpassL3: [fluid_real_t; 341],
    pub bufallpassR3: [fluid_real_t; 364],
    pub bufallpassL4: [fluid_real_t; 225],
    pub bufallpassR4: [fluid_real_t; 248],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_allpass {
    pub feedback: fluid_real_t,
    pub buffer: *mut fluid_real_t,
    pub bufsize: libc::c_int,
    pub bufidx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_comb {
    pub feedback: fluid_real_t,
    pub filterstore: fluid_real_t,
    pub damp1: fluid_real_t,
    pub damp2: fluid_real_t,
    pub buffer: *mut fluid_real_t,
    pub bufsize: libc::c_int,
    pub bufidx: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn fluid_allpass_setbuffer(
    mut allpass: *mut fluid_allpass,
    mut buf: *mut fluid_real_t,
    mut size: libc::c_int,
) {
    (*allpass).bufidx = 0 as libc::c_int;
    (*allpass).buffer = buf;
    (*allpass).bufsize = size;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_allpass_init(mut allpass: *mut fluid_allpass) {
    let mut i: libc::c_int;
    let mut len: libc::c_int = (*allpass).bufsize;
    let mut buf: *mut fluid_real_t = (*allpass).buffer;
    i = 0 as libc::c_int;
    while i < len {
        *buf.offset(i as isize) = 1e-8f64 as fluid_real_t;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_allpass_setfeedback(
    mut allpass: *mut fluid_allpass,
    mut val: fluid_real_t,
) {
    (*allpass).feedback = val;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_allpass_getfeedback(
    mut allpass: *mut fluid_allpass,
) -> fluid_real_t {
    return (*allpass).feedback;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_setbuffer(
    mut comb: *mut fluid_comb,
    mut buf: *mut fluid_real_t,
    mut size: libc::c_int,
) {
    (*comb).filterstore = 0 as libc::c_int as fluid_real_t;
    (*comb).bufidx = 0 as libc::c_int;
    (*comb).buffer = buf;
    (*comb).bufsize = size;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_init(mut comb: *mut fluid_comb) {
    let mut i: libc::c_int;
    let mut buf: *mut fluid_real_t = (*comb).buffer;
    let mut len: libc::c_int = (*comb).bufsize;
    i = 0 as libc::c_int;
    while i < len {
        *buf.offset(i as isize) = 1e-8f64 as fluid_real_t;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_setdamp(mut comb: *mut fluid_comb, mut val: fluid_real_t) {
    (*comb).damp1 = val;
    (*comb).damp2 = 1 as libc::c_int as libc::c_float - val;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_getdamp(mut comb: *mut fluid_comb) -> fluid_real_t {
    return (*comb).damp1;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_setfeedback(mut comb: *mut fluid_comb, mut val: fluid_real_t) {
    (*comb).feedback = val;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_getfeedback(mut comb: *mut fluid_comb) -> fluid_real_t {
    return (*comb).feedback;
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_revmodel() -> *mut fluid_revmodel_t {
    let mut rev: *mut fluid_revmodel_t;
    rev = libc::malloc(::std::mem::size_of::<fluid_revmodel_t>() as libc::size_t)
        as *mut fluid_revmodel_t;
    if rev.is_null() {
        return 0 as *mut fluid_revmodel_t;
    }
    fluid_comb_setbuffer(
        &mut *(*rev).combL.as_mut_ptr().offset(0 as libc::c_int as isize),
        (*rev).bufcombL1.as_mut_ptr(),
        1116 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combR.as_mut_ptr().offset(0 as libc::c_int as isize),
        (*rev).bufcombR1.as_mut_ptr(),
        1116 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combL.as_mut_ptr().offset(1 as libc::c_int as isize),
        (*rev).bufcombL2.as_mut_ptr(),
        1188 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combR.as_mut_ptr().offset(1 as libc::c_int as isize),
        (*rev).bufcombR2.as_mut_ptr(),
        1188 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combL.as_mut_ptr().offset(2 as libc::c_int as isize),
        (*rev).bufcombL3.as_mut_ptr(),
        1277 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combR.as_mut_ptr().offset(2 as libc::c_int as isize),
        (*rev).bufcombR3.as_mut_ptr(),
        1277 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combL.as_mut_ptr().offset(3 as libc::c_int as isize),
        (*rev).bufcombL4.as_mut_ptr(),
        1356 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combR.as_mut_ptr().offset(3 as libc::c_int as isize),
        (*rev).bufcombR4.as_mut_ptr(),
        1356 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combL.as_mut_ptr().offset(4 as libc::c_int as isize),
        (*rev).bufcombL5.as_mut_ptr(),
        1422 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combR.as_mut_ptr().offset(4 as libc::c_int as isize),
        (*rev).bufcombR5.as_mut_ptr(),
        1422 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combL.as_mut_ptr().offset(5 as libc::c_int as isize),
        (*rev).bufcombL6.as_mut_ptr(),
        1491 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combR.as_mut_ptr().offset(5 as libc::c_int as isize),
        (*rev).bufcombR6.as_mut_ptr(),
        1491 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combL.as_mut_ptr().offset(6 as libc::c_int as isize),
        (*rev).bufcombL7.as_mut_ptr(),
        1557 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combR.as_mut_ptr().offset(6 as libc::c_int as isize),
        (*rev).bufcombR7.as_mut_ptr(),
        1557 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combL.as_mut_ptr().offset(7 as libc::c_int as isize),
        (*rev).bufcombL8.as_mut_ptr(),
        1617 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).combR.as_mut_ptr().offset(7 as libc::c_int as isize),
        (*rev).bufcombR8.as_mut_ptr(),
        1617 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpassL
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        (*rev).bufallpassL1.as_mut_ptr(),
        556 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpassR
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        (*rev).bufallpassR1.as_mut_ptr(),
        556 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpassL
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        (*rev).bufallpassL2.as_mut_ptr(),
        441 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpassR
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        (*rev).bufallpassR2.as_mut_ptr(),
        441 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpassL
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        (*rev).bufallpassL3.as_mut_ptr(),
        341 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpassR
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        (*rev).bufallpassR3.as_mut_ptr(),
        341 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpassL
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        (*rev).bufallpassL4.as_mut_ptr(),
        225 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpassR
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        (*rev).bufallpassR4.as_mut_ptr(),
        225 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpassL
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpassR
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpassL
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpassR
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpassL
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpassR
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpassL
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpassR
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        0.5f32,
    );
    (*rev).roomsize = 0.5f32 * 0.28f32 + 0.7f32;
    (*rev).damp = 0.2f32 * 1.0f32;
    (*rev).wet = 1 as libc::c_int as libc::c_float * 3.0f32;
    (*rev).width = 1 as libc::c_int as fluid_real_t;
    (*rev).gain = 0.015f32;
    fluid_revmodel_update(rev);
    fluid_revmodel_init(rev);
    return rev;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_revmodel(mut rev: *mut fluid_revmodel_t) {
    libc::free(rev as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_init(mut rev: *mut fluid_revmodel_t) {
    let mut i: libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        fluid_comb_init(&mut *(*rev).combL.as_mut_ptr().offset(i as isize));
        fluid_comb_init(&mut *(*rev).combR.as_mut_ptr().offset(i as isize));
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        fluid_allpass_init(&mut *(*rev).allpassL.as_mut_ptr().offset(i as isize));
        fluid_allpass_init(&mut *(*rev).allpassR.as_mut_ptr().offset(i as isize));
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_reset(mut rev: *mut fluid_revmodel_t) {
    fluid_revmodel_init(rev);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_processreplace(
    mut rev: *mut fluid_revmodel_t,
    mut in_0: *mut fluid_real_t,
    mut left_out: *mut fluid_real_t,
    mut right_out: *mut fluid_real_t,
) {
    let mut i: libc::c_int;
    let mut k: libc::c_int;
    let mut outL: fluid_real_t;
    let mut outR: fluid_real_t;
    let mut input: fluid_real_t;
    k = 0 as libc::c_int;
    while k < 64 as libc::c_int {
        outR = 0 as libc::c_int as fluid_real_t;
        outL = outR;
        input = (((2 as libc::c_int as libc::c_float * *in_0.offset(k as isize)) as libc::c_double
            + 1e-8f64)
            * (*rev).gain as libc::c_double) as fluid_real_t;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let mut _tmp: fluid_real_t = *(*rev).combL[i as usize]
                .buffer
                .offset((*rev).combL[i as usize].bufidx as isize);
            (*rev).combL[i as usize].filterstore = _tmp * (*rev).combL[i as usize].damp2
                + (*rev).combL[i as usize].filterstore * (*rev).combL[i as usize].damp1;
            *(*rev).combL[i as usize]
                .buffer
                .offset((*rev).combL[i as usize].bufidx as isize) =
                input + (*rev).combL[i as usize].filterstore * (*rev).combL[i as usize].feedback;
            (*rev).combL[i as usize].bufidx += 1;
            if (*rev).combL[i as usize].bufidx >= (*rev).combL[i as usize].bufsize {
                (*rev).combL[i as usize].bufidx = 0 as libc::c_int
            }
            outL += _tmp;
            let mut _tmp_0: fluid_real_t = *(*rev).combR[i as usize]
                .buffer
                .offset((*rev).combR[i as usize].bufidx as isize);
            (*rev).combR[i as usize].filterstore = _tmp_0 * (*rev).combR[i as usize].damp2
                + (*rev).combR[i as usize].filterstore * (*rev).combR[i as usize].damp1;
            *(*rev).combR[i as usize]
                .buffer
                .offset((*rev).combR[i as usize].bufidx as isize) =
                input + (*rev).combR[i as usize].filterstore * (*rev).combR[i as usize].feedback;
            (*rev).combR[i as usize].bufidx += 1;
            if (*rev).combR[i as usize].bufidx >= (*rev).combR[i as usize].bufsize {
                (*rev).combR[i as usize].bufidx = 0 as libc::c_int
            }
            outR += _tmp_0;
            i += 1
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            let mut output: fluid_real_t;
            let mut bufout: fluid_real_t;
            bufout = *(*rev).allpassL[i as usize]
                .buffer
                .offset((*rev).allpassL[i as usize].bufidx as isize);
            output = bufout - outL;
            *(*rev).allpassL[i as usize]
                .buffer
                .offset((*rev).allpassL[i as usize].bufidx as isize) =
                outL + bufout * (*rev).allpassL[i as usize].feedback;
            (*rev).allpassL[i as usize].bufidx += 1;
            if (*rev).allpassL[i as usize].bufidx >= (*rev).allpassL[i as usize].bufsize {
                (*rev).allpassL[i as usize].bufidx = 0 as libc::c_int
            }
            outL = output;
            let mut output_0: fluid_real_t;
            let mut bufout_0: fluid_real_t;
            bufout_0 = *(*rev).allpassR[i as usize]
                .buffer
                .offset((*rev).allpassR[i as usize].bufidx as isize);
            output_0 = bufout_0 - outR;
            *(*rev).allpassR[i as usize]
                .buffer
                .offset((*rev).allpassR[i as usize].bufidx as isize) =
                outR + bufout_0 * (*rev).allpassR[i as usize].feedback;
            (*rev).allpassR[i as usize].bufidx += 1;
            if (*rev).allpassR[i as usize].bufidx >= (*rev).allpassR[i as usize].bufsize {
                (*rev).allpassR[i as usize].bufidx = 0 as libc::c_int
            }
            outR = output_0;
            i += 1
        }
        outL = (outL as libc::c_double - 1e-8f64) as fluid_real_t;
        outR = (outR as libc::c_double - 1e-8f64) as fluid_real_t;
        *left_out.offset(k as isize) = outL * (*rev).wet1 + outR * (*rev).wet2;
        *right_out.offset(k as isize) = outR * (*rev).wet1 + outL * (*rev).wet2;
        k += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_processmix(
    mut rev: *mut fluid_revmodel_t,
    mut in_0: *mut fluid_real_t,
    mut left_out: *mut fluid_real_t,
    mut right_out: *mut fluid_real_t,
) {
    let mut i: libc::c_int;
    let mut k: libc::c_int;
    let mut outL: fluid_real_t;
    let mut outR: fluid_real_t;
    let mut input: fluid_real_t;
    k = 0 as libc::c_int;
    while k < 64 as libc::c_int {
        outR = 0 as libc::c_int as fluid_real_t;
        outL = outR;
        input = (((2 as libc::c_int as libc::c_float * *in_0.offset(k as isize)) as libc::c_double
            + 1e-8f64)
            * (*rev).gain as libc::c_double) as fluid_real_t;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let mut _tmp: fluid_real_t = *(*rev).combL[i as usize]
                .buffer
                .offset((*rev).combL[i as usize].bufidx as isize);
            (*rev).combL[i as usize].filterstore = _tmp * (*rev).combL[i as usize].damp2
                + (*rev).combL[i as usize].filterstore * (*rev).combL[i as usize].damp1;
            *(*rev).combL[i as usize]
                .buffer
                .offset((*rev).combL[i as usize].bufidx as isize) =
                input + (*rev).combL[i as usize].filterstore * (*rev).combL[i as usize].feedback;
            (*rev).combL[i as usize].bufidx += 1;
            if (*rev).combL[i as usize].bufidx >= (*rev).combL[i as usize].bufsize {
                (*rev).combL[i as usize].bufidx = 0 as libc::c_int
            }
            outL += _tmp;
            let mut _tmp_0: fluid_real_t = *(*rev).combR[i as usize]
                .buffer
                .offset((*rev).combR[i as usize].bufidx as isize);
            (*rev).combR[i as usize].filterstore = _tmp_0 * (*rev).combR[i as usize].damp2
                + (*rev).combR[i as usize].filterstore * (*rev).combR[i as usize].damp1;
            *(*rev).combR[i as usize]
                .buffer
                .offset((*rev).combR[i as usize].bufidx as isize) =
                input + (*rev).combR[i as usize].filterstore * (*rev).combR[i as usize].feedback;
            (*rev).combR[i as usize].bufidx += 1;
            if (*rev).combR[i as usize].bufidx >= (*rev).combR[i as usize].bufsize {
                (*rev).combR[i as usize].bufidx = 0 as libc::c_int
            }
            outR += _tmp_0;
            i += 1
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            let mut output: fluid_real_t;
            let mut bufout: fluid_real_t;
            bufout = *(*rev).allpassL[i as usize]
                .buffer
                .offset((*rev).allpassL[i as usize].bufidx as isize);
            output = bufout - outL;
            *(*rev).allpassL[i as usize]
                .buffer
                .offset((*rev).allpassL[i as usize].bufidx as isize) =
                outL + bufout * (*rev).allpassL[i as usize].feedback;
            (*rev).allpassL[i as usize].bufidx += 1;
            if (*rev).allpassL[i as usize].bufidx >= (*rev).allpassL[i as usize].bufsize {
                (*rev).allpassL[i as usize].bufidx = 0 as libc::c_int
            }
            outL = output;
            let mut output_0: fluid_real_t;
            let mut bufout_0: fluid_real_t;
            bufout_0 = *(*rev).allpassR[i as usize]
                .buffer
                .offset((*rev).allpassR[i as usize].bufidx as isize);
            output_0 = bufout_0 - outR;
            *(*rev).allpassR[i as usize]
                .buffer
                .offset((*rev).allpassR[i as usize].bufidx as isize) =
                outR + bufout_0 * (*rev).allpassR[i as usize].feedback;
            (*rev).allpassR[i as usize].bufidx += 1;
            if (*rev).allpassR[i as usize].bufidx >= (*rev).allpassR[i as usize].bufsize {
                (*rev).allpassR[i as usize].bufidx = 0 as libc::c_int
            }
            outR = output_0;
            i += 1
        }
        outL = (outL as libc::c_double - 1e-8f64) as fluid_real_t;
        outR = (outR as libc::c_double - 1e-8f64) as fluid_real_t;
        let ref mut fresh0 = *left_out.offset(k as isize);
        *fresh0 += outL * (*rev).wet1 + outR * (*rev).wet2;
        let ref mut fresh1 = *right_out.offset(k as isize);
        *fresh1 += outR * (*rev).wet1 + outL * (*rev).wet2;
        k += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_update(mut rev: *mut fluid_revmodel_t) {
    let mut i: libc::c_int;
    (*rev).wet1 = (*rev).wet * ((*rev).width / 2 as libc::c_int as libc::c_float + 0.5f32);
    (*rev).wet2 = (*rev).wet
        * ((1 as libc::c_int as libc::c_float - (*rev).width) / 2 as libc::c_int as libc::c_float);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        fluid_comb_setfeedback(
            &mut *(*rev).combL.as_mut_ptr().offset(i as isize),
            (*rev).roomsize,
        );
        fluid_comb_setfeedback(
            &mut *(*rev).combR.as_mut_ptr().offset(i as isize),
            (*rev).roomsize,
        );
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        fluid_comb_setdamp(
            &mut *(*rev).combL.as_mut_ptr().offset(i as isize),
            (*rev).damp,
        );
        fluid_comb_setdamp(
            &mut *(*rev).combR.as_mut_ptr().offset(i as isize),
            (*rev).damp,
        );
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_setroomsize(
    mut rev: *mut fluid_revmodel_t,
    mut value: fluid_real_t,
) {
    (*rev).roomsize = value * 0.28f32 + 0.7f32;
    fluid_revmodel_update(rev);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_getroomsize(
    mut rev: *mut fluid_revmodel_t,
) -> fluid_real_t {
    return ((*rev).roomsize - 0.7f32) / 0.28f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_setdamp(
    mut rev: *mut fluid_revmodel_t,
    mut value: fluid_real_t,
) {
    (*rev).damp = value * 1.0f32;
    fluid_revmodel_update(rev);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_getdamp(mut rev: *mut fluid_revmodel_t) -> fluid_real_t {
    return (*rev).damp / 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_setlevel(
    mut rev: *mut fluid_revmodel_t,
    mut value: fluid_real_t,
) {
    value = if value < 0.0f32 {
        0.0f32
    } else if value > 1.0f32 {
        1.0f32
    } else {
        value
    };
    (*rev).wet = value * 3.0f32;
    fluid_revmodel_update(rev);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_getlevel(mut rev: *mut fluid_revmodel_t) -> fluid_real_t {
    return (*rev).wet / 3.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_setwidth(
    mut rev: *mut fluid_revmodel_t,
    mut value: fluid_real_t,
) {
    (*rev).width = value;
    fluid_revmodel_update(rev);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_getwidth(mut rev: *mut fluid_revmodel_t) -> fluid_real_t {
    return (*rev).width;
}
