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
    pub comb_l: [Comb; 8],
    pub comb_r: [Comb; 8],
    pub allpass_l: [AllPass; 4],
    pub allpass_r: [AllPass; 4],
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
pub struct AllPass {
    pub feedback: f32,
    pub buffer: *mut f32,
    pub bufsize: i32,
    pub bufidx: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Comb {
    pub feedback: f32,
    pub filterstore: f32,
    pub damp1: f32,
    pub damp2: f32,
    pub buffer: *mut f32,
    pub bufsize: i32,
    pub bufidx: i32,
}

pub unsafe fn fluid_allpass_setbuffer(
    mut allpass: *mut AllPass,
    buf: *mut f32,
    size: i32,
) {
    (*allpass).bufidx = 0 as i32;
    (*allpass).buffer = buf;
    (*allpass).bufsize = size;
}

pub unsafe fn fluid_allpass_init(allpass: *mut AllPass) {
    let mut i: i32;
    let len: i32 = (*allpass).bufsize;
    let buf: *mut f32 = (*allpass).buffer;
    i = 0 as i32;
    while i < len {
        *buf.offset(i as isize) = 1e-8f32;
        i += 1
    }
}

pub unsafe fn fluid_allpass_setfeedback(mut allpass: *mut AllPass, val: f32) {
    (*allpass).feedback = val;
}

pub unsafe fn fluid_comb_setbuffer(mut comb: *mut Comb, buf: *mut f32, size: i32) {
    (*comb).filterstore = 0 as i32 as f32;
    (*comb).bufidx = 0 as i32;
    (*comb).buffer = buf;
    (*comb).bufsize = size;
}

pub unsafe fn fluid_comb_init(comb: *mut Comb) {
    let mut i: i32;
    let buf: *mut f32 = (*comb).buffer;
    let len: i32 = (*comb).bufsize;
    i = 0 as i32;
    while i < len {
        *buf.offset(i as isize) = 1e-8f32;
        i += 1
    }
}

pub unsafe fn fluid_comb_setdamp(mut comb: *mut Comb, val: f32) {
    (*comb).damp1 = val;
    (*comb).damp2 = 1 as i32 as f32 - val;
}

pub unsafe fn fluid_comb_setfeedback(comb: *mut Comb, val: f32) {
    (*comb).feedback = val;
}

pub unsafe fn new_fluid_revmodel() -> *mut ReverbModel {
    let mut rev: *mut ReverbModel;
    rev = libc::malloc(::std::mem::size_of::<ReverbModel>() as libc::size_t) as *mut ReverbModel;
    if rev.is_null() {
        return 0 as *mut ReverbModel;
    }
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(0 as i32 as isize),
        (*rev).bufcomb_l1.as_mut_ptr(),
        1116 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(0 as i32 as isize),
        (*rev).bufcomb_r1.as_mut_ptr(),
        1116 as i32 + 23 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(1 as i32 as isize),
        (*rev).bufcomb_l2.as_mut_ptr(),
        1188 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(1 as i32 as isize),
        (*rev).bufcomb_r2.as_mut_ptr(),
        1188 as i32 + 23 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(2 as i32 as isize),
        (*rev).bufcomb_l3.as_mut_ptr(),
        1277 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(2 as i32 as isize),
        (*rev).bufcomb_r3.as_mut_ptr(),
        1277 as i32 + 23 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(3 as i32 as isize),
        (*rev).bufcomb_l4.as_mut_ptr(),
        1356 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(3 as i32 as isize),
        (*rev).bufcomb_r4.as_mut_ptr(),
        1356 as i32 + 23 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(4 as i32 as isize),
        (*rev).bufcomb_l5.as_mut_ptr(),
        1422 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(4 as i32 as isize),
        (*rev).bufcomb_r5.as_mut_ptr(),
        1422 as i32 + 23 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(5 as i32 as isize),
        (*rev).bufcomb_l6.as_mut_ptr(),
        1491 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(5 as i32 as isize),
        (*rev).bufcomb_r6.as_mut_ptr(),
        1491 as i32 + 23 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(6 as i32 as isize),
        (*rev).bufcomb_l7.as_mut_ptr(),
        1557 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(6 as i32 as isize),
        (*rev).bufcomb_r7.as_mut_ptr(),
        1557 as i32 + 23 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_l.as_mut_ptr().offset(7 as i32 as isize),
        (*rev).bufcomb_l8.as_mut_ptr(),
        1617 as i32,
    );
    fluid_comb_setbuffer(
        &mut *(*rev).comb_r.as_mut_ptr().offset(7 as i32 as isize),
        (*rev).bufcomb_r8.as_mut_ptr(),
        1617 as i32 + 23 as i32,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(0 as i32 as isize),
        (*rev).bufallpass_l1.as_mut_ptr(),
        556 as i32,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(0 as i32 as isize),
        (*rev).bufallpass_r1.as_mut_ptr(),
        556 as i32 + 23 as i32,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(1 as i32 as isize),
        (*rev).bufallpass_l2.as_mut_ptr(),
        441 as i32,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(1 as i32 as isize),
        (*rev).bufallpass_r2.as_mut_ptr(),
        441 as i32 + 23 as i32,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(2 as i32 as isize),
        (*rev).bufallpass_l3.as_mut_ptr(),
        341 as i32,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(2 as i32 as isize),
        (*rev).bufallpass_r3.as_mut_ptr(),
        341 as i32 + 23 as i32,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(3 as i32 as isize),
        (*rev).bufallpass_l4.as_mut_ptr(),
        225 as i32,
    );
    fluid_allpass_setbuffer(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(3 as i32 as isize),
        (*rev).bufallpass_r4.as_mut_ptr(),
        225 as i32 + 23 as i32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(0 as i32 as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(0 as i32 as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(1 as i32 as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(1 as i32 as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(2 as i32 as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(2 as i32 as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_l
            .as_mut_ptr()
            .offset(3 as i32 as isize),
        0.5f32,
    );
    fluid_allpass_setfeedback(
        &mut *(*rev)
            .allpass_r
            .as_mut_ptr()
            .offset(3 as i32 as isize),
        0.5f32,
    );
    (*rev).roomsize = 0.5f32 * 0.28f32 + 0.7f32;
    (*rev).damp = 0.2f32 * 1.0f32;
    (*rev).wet = 1 as i32 as f32 * 3.0f32;
    (*rev).width = 1 as i32 as f32;
    (*rev).gain = 0.015f32;
    fluid_revmodel_update(rev);
    fluid_revmodel_init(rev);
    return rev;
}

pub unsafe fn delete_fluid_revmodel(rev: *mut ReverbModel) {
    libc::free(rev as *mut libc::c_void);
}

pub unsafe fn fluid_revmodel_init(rev: *mut ReverbModel) {
    let mut i: i32;
    i = 0 as i32;
    while i < 8 as i32 {
        fluid_comb_init(&mut *(*rev).comb_l.as_mut_ptr().offset(i as isize));
        fluid_comb_init(&mut *(*rev).comb_r.as_mut_ptr().offset(i as isize));
        i += 1
    }
    i = 0 as i32;
    while i < 4 as i32 {
        fluid_allpass_init(&mut *(*rev).allpass_l.as_mut_ptr().offset(i as isize));
        fluid_allpass_init(&mut *(*rev).allpass_r.as_mut_ptr().offset(i as isize));
        i += 1
    }
}

pub unsafe fn fluid_revmodel_reset(rev: *mut ReverbModel) {
    fluid_revmodel_init(rev);
}

pub unsafe fn fluid_revmodel_processreplace(
    mut rev: *mut ReverbModel,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    let mut i: i32;
    let mut k: i32;
    let mut out_l: f32;
    let mut out_r: f32;
    let mut input: f32;
    k = 0 as i32;
    while k < 64 as i32 {
        out_r = 0 as i32 as f32;
        out_l = out_r;
        input = (((2 as i32 as f32 * *in_0.offset(k as isize)) as f64 + 1e-8f64)
            * (*rev).gain as f64) as f32;
        i = 0 as i32;
        while i < 8 as i32 {
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
                (*rev).comb_l[i as usize].bufidx = 0 as i32
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
                (*rev).comb_r[i as usize].bufidx = 0 as i32
            }
            out_r += _tmp_0;
            i += 1
        }
        i = 0 as i32;
        while i < 4 as i32 {
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
                (*rev).allpass_l[i as usize].bufidx = 0 as i32
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
                (*rev).allpass_r[i as usize].bufidx = 0 as i32
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

pub unsafe fn fluid_revmodel_processmix(
    mut rev: *mut ReverbModel,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    let mut i: i32;
    let mut k: i32;
    let mut out_l: f32;
    let mut out_r: f32;
    let mut input: f32;
    k = 0 as i32;
    while k < 64 as i32 {
        out_r = 0 as i32 as f32;
        out_l = out_r;
        input = (((2 as i32 as f32 * *in_0.offset(k as isize)) as f64 + 1e-8f64)
            * (*rev).gain as f64) as f32;
        i = 0 as i32;
        while i < 8 as i32 {
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
                (*rev).comb_l[i as usize].bufidx = 0 as i32
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
                (*rev).comb_r[i as usize].bufidx = 0 as i32
            }
            out_r += _tmp_0;
            i += 1
        }
        i = 0 as i32;
        while i < 4 as i32 {
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
                (*rev).allpass_l[i as usize].bufidx = 0 as i32
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
                (*rev).allpass_r[i as usize].bufidx = 0 as i32
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

pub unsafe fn fluid_revmodel_update(mut rev: *mut ReverbModel) {
    let mut i: i32;
    (*rev).wet1 = (*rev).wet * ((*rev).width / 2 as i32 as f32 + 0.5f32);
    (*rev).wet2 = (*rev).wet
        * ((1 as i32 as f32 - (*rev).width) / 2 as i32 as f32);
    i = 0 as i32;
    while i < 8 as i32 {
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
    i = 0 as i32;
    while i < 8 as i32 {
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

pub unsafe fn fluid_revmodel_setroomsize(mut rev: *mut ReverbModel, value: f32) {
    (*rev).roomsize = value * 0.28f32 + 0.7f32;
    fluid_revmodel_update(rev);
}

pub unsafe fn fluid_revmodel_getroomsize(rev: *mut ReverbModel) -> f32 {
    return ((*rev).roomsize - 0.7f32) / 0.28f32;
}

pub unsafe fn fluid_revmodel_setdamp(mut rev: *mut ReverbModel, value: f32) {
    (*rev).damp = value * 1.0f32;
    fluid_revmodel_update(rev);
}

pub unsafe fn fluid_revmodel_getdamp(rev: *mut ReverbModel) -> f32 {
    return (*rev).damp / 1.0f32;
}

pub unsafe fn fluid_revmodel_setlevel(mut rev: *mut ReverbModel, mut value: f32) {
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

pub unsafe fn fluid_revmodel_getlevel(rev: *const ReverbModel) -> f32 {
    return (*rev).wet / 3.0f32;
}

pub unsafe fn fluid_revmodel_setwidth(rev: *mut ReverbModel, value: f32) {
    (*rev).width = value;
    fluid_revmodel_update(rev);
}

pub unsafe fn fluid_revmodel_getwidth(rev: *const ReverbModel) -> f32 {
    return (*rev).width;
}
