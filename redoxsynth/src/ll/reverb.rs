#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReverbModel {
    pub roomsize: f32,
    pub damp: f32,
    pub wet: f32,
    pub wet1: f32,
    pub wet2: f32,
    pub width: f32,
    pub gain: f32,
    pub comb_l: [fluid_comb; 8],
    pub comb_r: [fluid_comb; 8],
    pub allpass_l: [fluid_allpass; 4],
    pub allpass_r: [fluid_allpass; 4],
    pub bufcomb_l1: [f32; 1116],
    pub bufcomb_r1: [f32; 1139],
    pub bufcomb_l2: [f32; 1188],
    pub bufcomb_r2: [f32; 1211],
    pub bufcomb_l3: [f32; 1277],
    pub bufcomb_r3: [f32; 1300],
    pub bufcomb_l4: [f32; 1356],
    pub bufcomb_r4: [f32; 1379],
    pub bufcomb_l5: [f32; 1422],
    pub bufcomb_r5: [f32; 1445],
    pub bufcomb_l6: [f32; 1491],
    pub bufcomb_r6: [f32; 1514],
    pub bufcomb_l7: [f32; 1557],
    pub bufcomb_r7: [f32; 1580],
    pub bufcomb_l8: [f32; 1617],
    pub bufcomb_r8: [f32; 1640],
    pub bufallpass_l1: [f32; 556],
    pub bufallpass_r1: [f32; 579],
    pub bufallpass_l2: [f32; 441],
    pub bufallpass_r2: [f32; 464],
    pub bufallpass_l3: [f32; 341],
    pub bufallpass_r3: [f32; 364],
    pub bufallpass_l4: [f32; 225],
    pub bufallpass_r4: [f32; 248],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_allpass {
    pub feedback: f32,
    pub buffer: *mut f32,
    pub bufsize: libc::c_int,
    pub bufidx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_comb {
    pub feedback: f32,
    pub filterstore: f32,
    pub damp1: f32,
    pub damp2: f32,
    pub buffer: *mut f32,
    pub bufsize: libc::c_int,
    pub bufidx: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn fluid_allpass_setbuffer(
    mut allpass: *mut fluid_allpass,
    buf: *mut f32,
    size: libc::c_int,
) {
    (*allpass).bufidx = 0 as libc::c_int;
    (*allpass).buffer = buf;
    (*allpass).bufsize = size;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_allpass_init(allpass: *mut fluid_allpass) {
    let mut i: libc::c_int;
    let len: libc::c_int = (*allpass).bufsize;
    let buf: *mut f32 = (*allpass).buffer;
    i = 0 as libc::c_int;
    while i < len {
        *buf.offset(i as isize) = 1e-8f64 as f32;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_allpass_setfeedback(
    mut allpass: *mut fluid_allpass,
    val: f32,
) {
    (*allpass).feedback = val;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_allpass_getfeedback(
    allpass: *mut fluid_allpass,
) -> f32 {
    return (*allpass).feedback;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_setbuffer(
    mut comb: *mut fluid_comb,
    buf: *mut f32,
    size: libc::c_int,
) {
    (*comb).filterstore = 0 as libc::c_int as f32;
    (*comb).bufidx = 0 as libc::c_int;
    (*comb).buffer = buf;
    (*comb).bufsize = size;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_init(comb: *mut fluid_comb) {
    let mut i: libc::c_int;
    let buf: *mut f32 = (*comb).buffer;
    let len: libc::c_int = (*comb).bufsize;
    i = 0 as libc::c_int;
    while i < len {
        *buf.offset(i as isize) = 1e-8f64 as f32;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_setdamp(mut comb: *mut fluid_comb, val: f32) {
    (*comb).damp1 = val;
    (*comb).damp2 = 1 as libc::c_int as libc::c_float - val;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_getdamp(comb: *const fluid_comb) -> f32 {
    return (*comb).damp1;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_setfeedback(comb: *mut fluid_comb, val: f32) {
    (*comb).feedback = val;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_comb_getfeedback(comb: *const fluid_comb) -> f32 {
    return (*comb).feedback;
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_revmodel() -> *mut ReverbModel {
    let mut rev: *mut ReverbModel;
    rev = libc::malloc(::std::mem::size_of::<ReverbModel>() as libc::size_t)
        as *mut ReverbModel;
    if rev.is_null() {
        return 0 as *mut ReverbModel;
    }
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(0 as libc::c_int as isize),
        (*rev).bufcomb_l1.as_mut_ptr(),
        1116 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(0 as libc::c_int as isize),
        (*rev).bufcomb_r1.as_mut_ptr(),
        1116 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(1 as libc::c_int as isize),
        (*rev).bufcomb_l2.as_mut_ptr(),
        1188 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(1 as libc::c_int as isize),
        (*rev).bufcomb_r2.as_mut_ptr(),
        1188 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(2 as libc::c_int as isize),
        (*rev).bufcomb_l3.as_mut_ptr(),
        1277 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(2 as libc::c_int as isize),
        (*rev).bufcomb_r3.as_mut_ptr(),
        1277 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(3 as libc::c_int as isize),
        (*rev).bufcomb_l4.as_mut_ptr(),
        1356 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(3 as libc::c_int as isize),
        (*rev).bufcomb_r4.as_mut_ptr(),
        1356 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(4 as libc::c_int as isize),
        (*rev).bufcomb_l5.as_mut_ptr(),
        1422 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(4 as libc::c_int as isize),
        (*rev).bufcomb_r5.as_mut_ptr(),
        1422 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(5 as libc::c_int as isize),
        (*rev).bufcomb_l6.as_mut_ptr(),
        1491 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(5 as libc::c_int as isize),
        (*rev).bufcomb_r6.as_mut_ptr(),
        1491 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(6 as libc::c_int as isize),
        (*rev).bufcomb_l7.as_mut_ptr(),
        1557 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(6 as libc::c_int as isize),
        (*rev).bufcomb_r7.as_mut_ptr(),
        1557 as libc::c_int + 23 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(7 as libc::c_int as isize),
        (*rev).bufcomb_l8.as_mut_ptr(),
        1617 as libc::c_int,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(7 as libc::c_int as isize),
        (*rev).bufcomb_r8.as_mut_ptr(),
        1617 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        (*rev).bufallpass_l1.as_mut_ptr(),
        556 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        (*rev).bufallpass_r1.as_mut_ptr(),
        556 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        (*rev).bufallpass_l2.as_mut_ptr(),
        441 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        (*rev).bufallpass_r2.as_mut_ptr(),
        441 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        (*rev).bufallpass_l3.as_mut_ptr(),
        341 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        (*rev).bufallpass_r3.as_mut_ptr(),
        341 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        (*rev).bufallpass_l4.as_mut_ptr(),
        225 as libc::c_int,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        (*rev).bufallpass_r4.as_mut_ptr(),
        225 as libc::c_int + 23 as libc::c_int,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        0.5f32,
    );
    (*rev).roomsize = 0.5f32 * 0.28f32 + 0.7f32;
    (*rev).damp = 0.2f32 * 1.0f32;
    (*rev).wet = 1 as libc::c_int as libc::c_float * 3.0f32;
    (*rev).width = 1 as libc::c_int as f32;
    (*rev).gain = 0.015f32;
    fluid_revmodel_update(rev);
    fluid_revmodel_init(rev);
    return rev;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_revmodel(rev: *mut ReverbModel) {
    libc::free(rev as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_init(rev: *mut ReverbModel) {
    let mut i: libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        fluid_comb_init(&mut *(*rev).comb_l.as_mut_ptr().offset(i as isize));
        fluid_comb_init(&mut *(*rev).comb_r.as_mut_ptr().offset(i as isize));
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        fluid_allpass_init(&mut *(*rev).allpass_l.as_mut_ptr().offset(i as isize));
        fluid_allpass_init(&mut *(*rev).allpass_r.as_mut_ptr().offset(i as isize));
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_reset(rev: *mut ReverbModel) {
    fluid_revmodel_init(rev);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_processreplace(
    mut rev: *mut ReverbModel,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    let mut i: libc::c_int;
    let mut k: libc::c_int;
    let mut out_l: f32;
    let mut out_r: f32;
    let mut input: f32;
    k = 0 as libc::c_int;
    while k < 64 as libc::c_int {
        out_r = 0 as libc::c_int as f32;
        out_l = out_r;
        input = (((2 as libc::c_int as libc::c_float * *in_0.offset(k as isize)) as f64
            + 1e-8f64)
            * (*rev).gain as f64) as f32;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let mut _tmp: f32 = *(*rev).comb_l[i as usize]
                .buffer
                .offset((*rev).comb_l[i as usize].bufidx as isize);
            (*rev).comb_l[i as usize].filterstore = _tmp * (*rev).comb_l[i as usize].damp2
                + (*rev).comb_l[i as usize].filterstore * (*rev).comb_l[i as usize].damp1;
            *(*rev).comb_l[i as usize]
                .buffer
                .offset((*rev).comb_l[i as usize].bufidx as isize) =
                input + (*rev).comb_l[i as usize].filterstore * (*rev).comb_l[i as usize].feedback;
            (*rev).comb_l[i as usize].bufidx += 1;
            if (*rev).comb_l[i as usize].bufidx >= (*rev).comb_l[i as usize].bufsize {
                (*rev).comb_l[i as usize].bufidx = 0 as libc::c_int
            }
            out_l += _tmp;
            let mut _tmp_0: f32 = *(*rev).comb_r[i as usize]
                .buffer
                .offset((*rev).comb_r[i as usize].bufidx as isize);
            (*rev).comb_r[i as usize].filterstore = _tmp_0 * (*rev).comb_r[i as usize].damp2
                + (*rev).comb_r[i as usize].filterstore * (*rev).comb_r[i as usize].damp1;
            *(*rev).comb_r[i as usize]
                .buffer
                .offset((*rev).comb_r[i as usize].bufidx as isize) =
                input + (*rev).comb_r[i as usize].filterstore * (*rev).comb_r[i as usize].feedback;
            (*rev).comb_r[i as usize].bufidx += 1;
            if (*rev).comb_r[i as usize].bufidx >= (*rev).comb_r[i as usize].bufsize {
                (*rev).comb_r[i as usize].bufidx = 0 as libc::c_int
            }
            out_r += _tmp_0;
            i += 1
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            let output: f32;
            let bufout: f32;
            bufout = *(*rev).allpass_l[i as usize]
                .buffer
                .offset((*rev).allpass_l[i as usize].bufidx as isize);
            output = bufout - out_l;
            *(*rev).allpass_l[i as usize]
                .buffer
                .offset((*rev).allpass_l[i as usize].bufidx as isize) =
                out_l + bufout * (*rev).allpass_l[i as usize].feedback;
            (*rev).allpass_l[i as usize].bufidx += 1;
            if (*rev).allpass_l[i as usize].bufidx >= (*rev).allpass_l[i as usize].bufsize {
                (*rev).allpass_l[i as usize].bufidx = 0 as libc::c_int
            }
            out_l = output;
            let output_0: f32;
            let bufout_0: f32;
            bufout_0 = *(*rev).allpass_r[i as usize]
                .buffer
                .offset((*rev).allpass_r[i as usize].bufidx as isize);
            output_0 = bufout_0 - out_r;
            *(*rev).allpass_r[i as usize]
                .buffer
                .offset((*rev).allpass_r[i as usize].bufidx as isize) =
                out_r + bufout_0 * (*rev).allpass_r[i as usize].feedback;
            (*rev).allpass_r[i as usize].bufidx += 1;
            if (*rev).allpass_r[i as usize].bufidx >= (*rev).allpass_r[i as usize].bufsize {
                (*rev).allpass_r[i as usize].bufidx = 0 as libc::c_int
            }
            out_r = output_0;
            i += 1
        }
        out_l = (out_l as f64 - 1e-8f64) as f32;
        out_r = (out_r as f64 - 1e-8f64) as f32;
        *left_out.offset(k as isize) = out_l * (*rev).wet1 + out_r * (*rev).wet2;
        *right_out.offset(k as isize) = out_r * (*rev).wet1 + out_l * (*rev).wet2;
        k += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_processmix(
    mut rev: *mut ReverbModel,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    let mut i: libc::c_int;
    let mut k: libc::c_int;
    let mut out_l: f32;
    let mut out_r: f32;
    let mut input: f32;
    k = 0 as libc::c_int;
    while k < 64 as libc::c_int {
        out_r = 0 as libc::c_int as f32;
        out_l = out_r;
        input = (((2 as libc::c_int as libc::c_float * *in_0.offset(k as isize)) as f64
            + 1e-8f64)
            * (*rev).gain as f64) as f32;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let mut _tmp: f32 = *(*rev).comb_l[i as usize]
                .buffer
                .offset((*rev).comb_l[i as usize].bufidx as isize);
            (*rev).comb_l[i as usize].filterstore = _tmp * (*rev).comb_l[i as usize].damp2
                + (*rev).comb_l[i as usize].filterstore * (*rev).comb_l[i as usize].damp1;
            *(*rev).comb_l[i as usize]
                .buffer
                .offset((*rev).comb_l[i as usize].bufidx as isize) =
                input + (*rev).comb_l[i as usize].filterstore * (*rev).comb_l[i as usize].feedback;
            (*rev).comb_l[i as usize].bufidx += 1;
            if (*rev).comb_l[i as usize].bufidx >= (*rev).comb_l[i as usize].bufsize {
                (*rev).comb_l[i as usize].bufidx = 0 as libc::c_int
            }
            out_l += _tmp;
            let mut _tmp_0: f32 = *(*rev).comb_r[i as usize]
                .buffer
                .offset((*rev).comb_r[i as usize].bufidx as isize);
            (*rev).comb_r[i as usize].filterstore = _tmp_0 * (*rev).comb_r[i as usize].damp2
                + (*rev).comb_r[i as usize].filterstore * (*rev).comb_r[i as usize].damp1;
            *(*rev).comb_r[i as usize]
                .buffer
                .offset((*rev).comb_r[i as usize].bufidx as isize) =
                input + (*rev).comb_r[i as usize].filterstore * (*rev).comb_r[i as usize].feedback;
            (*rev).comb_r[i as usize].bufidx += 1;
            if (*rev).comb_r[i as usize].bufidx >= (*rev).comb_r[i as usize].bufsize {
                (*rev).comb_r[i as usize].bufidx = 0 as libc::c_int
            }
            out_r += _tmp_0;
            i += 1
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            let output: f32;
            let bufout: f32;
            bufout = *(*rev).allpass_l[i as usize]
                .buffer
                .offset((*rev).allpass_l[i as usize].bufidx as isize);
            output = bufout - out_l;
            *(*rev).allpass_l[i as usize]
                .buffer
                .offset((*rev).allpass_l[i as usize].bufidx as isize) =
                out_l + bufout * (*rev).allpass_l[i as usize].feedback;
            (*rev).allpass_l[i as usize].bufidx += 1;
            if (*rev).allpass_l[i as usize].bufidx >= (*rev).allpass_l[i as usize].bufsize {
                (*rev).allpass_l[i as usize].bufidx = 0 as libc::c_int
            }
            out_l = output;
            let output_0: f32;
            let bufout_0: f32;
            bufout_0 = *(*rev).allpass_r[i as usize]
                .buffer
                .offset((*rev).allpass_r[i as usize].bufidx as isize);
            output_0 = bufout_0 - out_r;
            *(*rev).allpass_r[i as usize]
                .buffer
                .offset((*rev).allpass_r[i as usize].bufidx as isize) =
                out_r + bufout_0 * (*rev).allpass_r[i as usize].feedback;
            (*rev).allpass_r[i as usize].bufidx += 1;
            if (*rev).allpass_r[i as usize].bufidx >= (*rev).allpass_r[i as usize].bufsize {
                (*rev).allpass_r[i as usize].bufidx = 0 as libc::c_int
            }
            out_r = output_0;
            i += 1
        }
        out_l = (out_l as f64 - 1e-8f64) as f32;
        out_r = (out_r as f64 - 1e-8f64) as f32;
        let ref mut fresh0 = *left_out.offset(k as isize);
        *fresh0 += out_l * (*rev).wet1 + out_r * (*rev).wet2;
        let ref mut fresh1 = *right_out.offset(k as isize);
        *fresh1 += out_r * (*rev).wet1 + out_l * (*rev).wet2;
        k += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_update(mut rev: *mut ReverbModel) {
    let mut i: libc::c_int;
    (*rev).wet1 = (*rev).wet * ((*rev).width / 2 as libc::c_int as libc::c_float + 0.5f32);
    (*rev).wet2 = (*rev).wet
        * ((1 as libc::c_int as libc::c_float - (*rev).width) / 2 as libc::c_int as libc::c_float);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        fluid_comb_setfeedback(
            &mut *(*rev).comb_l.as_mut_ptr().offset(i as isize),
            (*rev).roomsize,
        );
        fluid_comb_setfeedback(
            &mut *(*rev).comb_r.as_mut_ptr().offset(i as isize),
            (*rev).roomsize,
        );
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        fluid_comb_setdamp(
            &mut *(*rev).comb_l.as_mut_ptr().offset(i as isize),
            (*rev).damp,
        );
        fluid_comb_setdamp(
            &mut *(*rev).comb_r.as_mut_ptr().offset(i as isize),
            (*rev).damp,
        );
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_setroomsize(
    mut rev: *mut ReverbModel,
    value: f32,
) {
    (*rev).roomsize = value * 0.28f32 + 0.7f32;
    fluid_revmodel_update(rev);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_getroomsize(
    rev: *mut ReverbModel,
) -> f32 {
    return ((*rev).roomsize - 0.7f32) / 0.28f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_setdamp(
    mut rev: *mut ReverbModel,
    value: f32,
) {
    (*rev).damp = value * 1.0f32;
    fluid_revmodel_update(rev);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_getdamp(rev: *mut ReverbModel) -> f32 {
    return (*rev).damp / 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_setlevel(
    mut rev: *mut ReverbModel,
    mut value: f32,
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
pub unsafe extern "C" fn fluid_revmodel_getlevel(rev: *const ReverbModel) -> f32 {
    return (*rev).wet / 3.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_setwidth(
    rev: *mut ReverbModel,
    value: f32,
) {
    (*rev).width = value;
    fluid_revmodel_update(rev);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_revmodel_getwidth(rev: *const ReverbModel) -> f32 {
    return (*rev).width;
}
