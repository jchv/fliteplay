const DC_OFFSET: f32 = 1e-8;
const STEREO_SPREAD: usize = 23;

const COMBTUNING_L1: usize = 1116;
const COMBTUNING_R1: usize = 1116 + STEREO_SPREAD;
const COMBTUNING_L2: usize = 1188;
const COMBTUNING_R2: usize = 1188 + STEREO_SPREAD;
const COMBTUNING_L3: usize = 1277;
const COMBTUNING_R3: usize = 1277 + STEREO_SPREAD;
const COMBTUNING_L4: usize = 1356;
const COMBTUNING_R4: usize = 1356 + STEREO_SPREAD;
const COMBTUNING_L5: usize = 1422;
const COMBTUNING_R5: usize = 1422 + STEREO_SPREAD;
const COMBTUNING_L6: usize = 1491;
const COMBTUNING_R6: usize = 1491 + STEREO_SPREAD;
const COMBTUNING_L7: usize = 1557;
const COMBTUNING_R7: usize = 1557 + STEREO_SPREAD;
const COMBTUNING_L8: usize = 1617;
const COMBTUNING_R8: usize = 1617 + STEREO_SPREAD;
const ALLPASSTUNING_L1: usize = 556;
const ALLPASSTUNING_R1: usize = 556 + STEREO_SPREAD;
const ALLPASSTUNING_L2: usize = 441;
const ALLPASSTUNING_R2: usize = 441 + STEREO_SPREAD;
const ALLPASSTUNING_L3: usize = 341;
const ALLPASSTUNING_R3: usize = 341 + STEREO_SPREAD;
const ALLPASSTUNING_L4: usize = 225;
const ALLPASSTUNING_R4: usize = 225 + STEREO_SPREAD;

#[derive(Copy, Clone)]
pub struct ReverbModel {
    pub roomsize: f32,
    pub damp: f32,
    pub wet: f32,
    pub wet1: f32,
    pub wet2: f32,
    pub width: f32,
    pub gain: f32,
    pub comb_l1: Comb<COMBTUNING_L1>,
    pub comb_r1: Comb<COMBTUNING_R1>,
    pub comb_l2: Comb<COMBTUNING_L2>,
    pub comb_r2: Comb<COMBTUNING_R2>,
    pub comb_l3: Comb<COMBTUNING_L3>,
    pub comb_r3: Comb<COMBTUNING_R3>,
    pub comb_l4: Comb<COMBTUNING_L4>,
    pub comb_r4: Comb<COMBTUNING_R4>,
    pub comb_l5: Comb<COMBTUNING_L5>,
    pub comb_r5: Comb<COMBTUNING_R5>,
    pub comb_l6: Comb<COMBTUNING_L6>,
    pub comb_r6: Comb<COMBTUNING_R6>,
    pub comb_l7: Comb<COMBTUNING_L7>,
    pub comb_r7: Comb<COMBTUNING_R7>,
    pub comb_l8: Comb<COMBTUNING_L8>,
    pub comb_r8: Comb<COMBTUNING_R8>,
    pub allpass_l1: AllPass<ALLPASSTUNING_L1>,
    pub allpass_r1: AllPass<ALLPASSTUNING_R1>,
    pub allpass_l2: AllPass<ALLPASSTUNING_L2>,
    pub allpass_r2: AllPass<ALLPASSTUNING_R2>,
    pub allpass_l3: AllPass<ALLPASSTUNING_L3>,
    pub allpass_r3: AllPass<ALLPASSTUNING_R3>,
    pub allpass_l4: AllPass<ALLPASSTUNING_L4>,
    pub allpass_r4: AllPass<ALLPASSTUNING_R4>,
}

#[derive(Copy, Clone)]
pub struct AllPass<const N: usize> {
    pub feedback: f32,
    pub buffer: [f32; N],
    pub bufidx: usize,
}

impl<const N: usize> AllPass<N> {
    pub fn new(feedback: f32) -> AllPass<N> {
        return AllPass{
            feedback,
            buffer: [DC_OFFSET; N],
            bufidx: 0,
        };
    }

    pub fn set_feedback(self: &mut AllPass<N>, val: f32) {
        self.feedback = val;
    }

    pub fn process(self: &mut AllPass<N>, input: f32) -> f32 {
        let bufout: f32 = self.buffer[self.bufidx];
        let output: f32 = bufout - input;
        self.buffer[self.bufidx] = input + bufout * self.feedback;
        self.bufidx += 1;
        if self.bufidx >= N {
            self.bufidx = 0
        }
        return output;
    }
}

#[derive(Copy, Clone)]
pub struct Comb<const N: usize> {
    pub feedback: f32,
    pub filterstore: f32,
    pub damp1: f32,
    pub damp2: f32,
    pub buffer: [f32; N],
    pub bufidx: usize,
}

impl<const N: usize> Comb<N> {
    pub fn new() -> Comb<N> {
        return Comb{
            feedback: 0f32,
            filterstore: 0f32,
            damp1: 0f32,
            damp2: 0f32,
            buffer: [DC_OFFSET; N],
            bufidx: 0,
        };
    }

    pub fn set_damp(self: &mut Comb<N>, val: f32) {
        self.damp1 = val;
        self.damp2 = 1f32 - val;
    }
    
    pub fn set_feedback(self: &mut Comb<N>, val: f32) {
        self.feedback = val;
    }

    pub fn process(self: &mut Comb<N>, input: f32) -> f32 {
        let mut _tmp = self.buffer[self.bufidx];
        self.filterstore = _tmp * self.damp2 + self.filterstore * self.damp1;
        self.buffer[self.bufidx] = input + self.filterstore * self.feedback;
        self.bufidx += 1;
        if self.bufidx >= N {
            self.bufidx = 0
        }
        return _tmp;
    }
}

pub fn new_fluid_revmodel() -> ReverbModel {
    let mut rev = ReverbModel{
        roomsize: 0.5f32 * 0.28f32 + 0.7f32,
        damp: 0.2f32 * 1.0f32,
        wet: 1f32 * 3.0f32,
        wet1: 0f32,
        wet2: 0f32,
        width: 1f32,
        gain: 0.015f32,
        comb_l1: Comb::new(),
        comb_r1: Comb::new(),
        comb_l2: Comb::new(),
        comb_r2: Comb::new(),
        comb_l3: Comb::new(),
        comb_r3: Comb::new(),
        comb_l4: Comb::new(),
        comb_r4: Comb::new(),
        comb_l5: Comb::new(),
        comb_r5: Comb::new(),
        comb_l6: Comb::new(),
        comb_r6: Comb::new(),
        comb_l7: Comb::new(),
        comb_r7: Comb::new(),
        comb_l8: Comb::new(),
        comb_r8: Comb::new(),
        allpass_l1: AllPass::new(0.5f32),
        allpass_r1: AllPass::new(0.5f32),
        allpass_l2: AllPass::new(0.5f32),
        allpass_r2: AllPass::new(0.5f32),
        allpass_l3: AllPass::new(0.5f32),
        allpass_r3: AllPass::new(0.5f32),
        allpass_l4: AllPass::new(0.5f32),
        allpass_r4: AllPass::new(0.5f32),
    };
    fluid_revmodel_update(&mut rev);
    fluid_revmodel_init(&mut rev);
    return rev;
}

pub fn fluid_revmodel_init(rev: &mut ReverbModel) {
    rev.comb_l1 = Comb::new();
    rev.comb_r1 = Comb::new();
    rev.comb_l2 = Comb::new();
    rev.comb_r2 = Comb::new();
    rev.comb_l3 = Comb::new();
    rev.comb_r3 = Comb::new();
    rev.comb_l4 = Comb::new();
    rev.comb_r4 = Comb::new();
    rev.comb_l5 = Comb::new();
    rev.comb_r5 = Comb::new();
    rev.comb_l6 = Comb::new();
    rev.comb_r6 = Comb::new();
    rev.comb_l7 = Comb::new();
    rev.comb_r7 = Comb::new();
    rev.comb_l8 = Comb::new();
    rev.comb_r8 = Comb::new();
    rev.allpass_l1 = AllPass::new(0.5f32);
    rev.allpass_r1 = AllPass::new(0.5f32);
    rev.allpass_l2 = AllPass::new(0.5f32);
    rev.allpass_r2 = AllPass::new(0.5f32);
    rev.allpass_l3 = AllPass::new(0.5f32);
    rev.allpass_r3 = AllPass::new(0.5f32);
    rev.allpass_l4 = AllPass::new(0.5f32);
    rev.allpass_r4 = AllPass::new(0.5f32);
}

pub fn fluid_revmodel_reset(rev: &mut ReverbModel) {
    fluid_revmodel_init(rev);
}

pub fn fluid_revmodel_processreplace(
    rev: &mut ReverbModel,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    for k in 0..64 {
        let mut out_r = 0f32;
        let mut out_l = 0f32;

        let input = unsafe {
            (((2 as i32 as f32 * *in_0.offset(k as isize)) as f64 + 1e-8f64) * rev.gain as f64) as f32
        };

        out_l += rev.comb_l1.process(input);
        out_r += rev.comb_r1.process(input);
        out_l += rev.comb_l2.process(input);
        out_r += rev.comb_r2.process(input);
        out_l += rev.comb_l3.process(input);
        out_r += rev.comb_r3.process(input);
        out_l += rev.comb_l4.process(input);
        out_r += rev.comb_r4.process(input);
        out_l += rev.comb_l5.process(input);
        out_r += rev.comb_r5.process(input);
        out_l += rev.comb_l6.process(input);
        out_r += rev.comb_r6.process(input);
        out_l += rev.comb_l7.process(input);
        out_r += rev.comb_r7.process(input);
        out_l += rev.comb_l8.process(input);
        out_r += rev.comb_r8.process(input);
        
        out_l = rev.allpass_l1.process(out_l);
        out_r = rev.allpass_r1.process(out_r);
        out_l = rev.allpass_l2.process(out_l);
        out_r = rev.allpass_r2.process(out_r);
        out_l = rev.allpass_l3.process(out_l);
        out_r = rev.allpass_r3.process(out_r);
        out_l = rev.allpass_l4.process(out_l);
        out_r = rev.allpass_r4.process(out_r);

        out_l -= DC_OFFSET;
        out_r -= DC_OFFSET;

        unsafe {
            *left_out.offset(k as isize) = out_l * rev.wet1 + out_r * rev.wet2;
            *right_out.offset(k as isize) = out_r * rev.wet1 + out_l * rev.wet2;
        }
    }
}

pub fn fluid_revmodel_processmix(
    rev: &mut ReverbModel,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    for k in 0..64 {
        let mut out_r = 0f32;
        let mut out_l = out_r;
        let input = unsafe {
            (((2 as i32 as f32 * *in_0.offset(k as isize)) as f64 + 1e-8f64) * rev.gain as f64) as f32
        };

        out_l += rev.comb_l1.process(input);
        out_r += rev.comb_r1.process(input);
        out_l += rev.comb_l2.process(input);
        out_r += rev.comb_r2.process(input);
        out_l += rev.comb_l3.process(input);
        out_r += rev.comb_r3.process(input);
        out_l += rev.comb_l4.process(input);
        out_r += rev.comb_r4.process(input);
        out_l += rev.comb_l5.process(input);
        out_r += rev.comb_r5.process(input);
        out_l += rev.comb_l6.process(input);
        out_r += rev.comb_r6.process(input);
        out_l += rev.comb_l7.process(input);
        out_r += rev.comb_r7.process(input);
        out_l += rev.comb_l8.process(input);
        out_r += rev.comb_r8.process(input);
        
        out_l = rev.allpass_l1.process(out_l);
        out_r = rev.allpass_r1.process(out_r);
        out_l = rev.allpass_l2.process(out_l);
        out_r = rev.allpass_r2.process(out_r);
        out_l = rev.allpass_l3.process(out_l);
        out_r = rev.allpass_r3.process(out_r);
        out_l = rev.allpass_l4.process(out_l);
        out_r = rev.allpass_r4.process(out_r);

        out_l -= DC_OFFSET;
        out_r -= DC_OFFSET;

        unsafe {
            *left_out.offset(k as isize) += out_l * rev.wet1 + out_r * rev.wet2;
            *right_out.offset(k as isize) += out_r * rev.wet1 + out_l * rev.wet2;
        }
    }
}

pub fn fluid_revmodel_update(rev: &mut ReverbModel) {
    rev.wet1 = rev.wet * (rev.width / 2f32 + 0.5f32);
    rev.wet2 = rev.wet * ((1f32 - rev.width) / 2f32);
    rev.comb_l1.set_feedback(rev.roomsize);
    rev.comb_r1.set_feedback(rev.roomsize);
    rev.comb_l2.set_feedback(rev.roomsize);
    rev.comb_r2.set_feedback(rev.roomsize);
    rev.comb_l3.set_feedback(rev.roomsize);
    rev.comb_r3.set_feedback(rev.roomsize);
    rev.comb_l4.set_feedback(rev.roomsize);
    rev.comb_r4.set_feedback(rev.roomsize);
    rev.comb_l5.set_feedback(rev.roomsize);
    rev.comb_r5.set_feedback(rev.roomsize);
    rev.comb_l6.set_feedback(rev.roomsize);
    rev.comb_r6.set_feedback(rev.roomsize);
    rev.comb_l7.set_feedback(rev.roomsize);
    rev.comb_r7.set_feedback(rev.roomsize);
    rev.comb_l8.set_feedback(rev.roomsize);
    rev.comb_r8.set_feedback(rev.roomsize);
    rev.comb_l1.set_feedback(rev.roomsize);
    rev.comb_r1.set_damp(rev.damp);
    rev.comb_l2.set_damp(rev.damp);
    rev.comb_r2.set_damp(rev.damp);
    rev.comb_l3.set_damp(rev.damp);
    rev.comb_r3.set_damp(rev.damp);
    rev.comb_l4.set_damp(rev.damp);
    rev.comb_r4.set_damp(rev.damp);
    rev.comb_l5.set_damp(rev.damp);
    rev.comb_r5.set_damp(rev.damp);
    rev.comb_l6.set_damp(rev.damp);
    rev.comb_r6.set_damp(rev.damp);
    rev.comb_l7.set_damp(rev.damp);
    rev.comb_r7.set_damp(rev.damp);
    rev.comb_l8.set_damp(rev.damp);
    rev.comb_r8.set_damp(rev.damp);
}

pub fn fluid_revmodel_setroomsize(rev: &mut ReverbModel, value: f32) {
    rev.roomsize = value * 0.28f32 + 0.7f32;
    fluid_revmodel_update(rev);
}

pub fn fluid_revmodel_getroomsize(rev: &mut ReverbModel) -> f32 {
    return (rev.roomsize - 0.7f32) / 0.28f32;
}

pub fn fluid_revmodel_setdamp(mut rev: &mut ReverbModel, value: f32) {
    rev.damp = value * 1.0f32;
    fluid_revmodel_update(rev);
}

pub fn fluid_revmodel_getdamp(rev: &ReverbModel) -> f32 {
    return rev.damp / 1.0f32;
}

pub fn fluid_revmodel_setlevel(mut rev: &mut ReverbModel, mut value: f32) {
    value = if value < 0.0f32 {
        0.0f32
    } else if value > 1.0f32 {
        1.0f32
    } else {
        value
    };
    rev.wet = value * 3.0f32;
    fluid_revmodel_update(rev);
}

pub fn fluid_revmodel_getlevel(rev: &ReverbModel) -> f32 {
    return rev.wet / 3.0f32;
}

pub fn fluid_revmodel_setwidth(rev: &mut ReverbModel, value: f32) {
    rev.width = value;
    fluid_revmodel_update(rev);
}

pub fn fluid_revmodel_getwidth(rev: &ReverbModel) -> f32 {
    return rev.width;
}
