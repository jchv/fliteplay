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


#[derive(Clone)]
pub struct Comb {
    pub feedback: f32,
    pub filterstore: f32,
    pub damp1: f32,
    pub damp2: f32,
    pub buffer: Vec<f32>,
    pub bufidx: usize,
}

impl Comb {
    pub fn new(size: usize) -> Self {
        return Self{
            feedback: 0f32,
            filterstore: 0f32,
            damp1: 0f32,
            damp2: 0f32,
            buffer: vec![DC_OFFSET; size],
            bufidx: 0,
        };
    }

    pub fn set_damp(&mut self, val: f32) {
        self.damp1 = val;
        self.damp2 = 1f32 - val;
    }
    
    pub fn set_feedback(&mut self, val: f32) {
        self.feedback = val;
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let mut _tmp = self.buffer[self.bufidx];
        self.filterstore = _tmp * self.damp2 + self.filterstore * self.damp1;
        self.buffer[self.bufidx] = input + self.filterstore * self.feedback;
        self.bufidx += 1;
        if self.bufidx >= self.buffer.len() {
            self.bufidx = 0
        }
        return _tmp;
    }
}

#[derive(Clone)]
pub struct AllPass {
    pub feedback: f32,
    pub buffer: Vec<f32>,
    pub bufidx: usize,
}

impl AllPass {
    pub fn new(size: usize, feedback: f32) -> Self {
        return Self{
            feedback,
            buffer: vec![DC_OFFSET; size],
            bufidx: 0,
        };
    }

    pub fn set_feedback(self: &mut Self, val: f32) {
        self.feedback = val;
    }

    pub fn process(self: &mut Self, input: f32) -> f32 {
        let bufout: f32 = self.buffer[self.bufidx];
        let output: f32 = bufout - input;
        self.buffer[self.bufidx] = input + bufout * self.feedback;
        self.bufidx += 1;
        if self.bufidx >= self.buffer.len() {
            self.bufidx = 0
        }
        return output;
    }
}

#[derive(Clone)]
pub struct ReverbModel {
    pub roomsize: f32,
    pub damp: f32,
    pub wet: f32,
    pub wet1: f32,
    pub wet2: f32,
    pub width: f32,
    pub gain: f32,
    pub comb_l1: Comb,
    pub comb_r1: Comb,
    pub comb_l2: Comb,
    pub comb_r2: Comb,
    pub comb_l3: Comb,
    pub comb_r3: Comb,
    pub comb_l4: Comb,
    pub comb_r4: Comb,
    pub comb_l5: Comb,
    pub comb_r5: Comb,
    pub comb_l6: Comb,
    pub comb_r6: Comb,
    pub comb_l7: Comb,
    pub comb_r7: Comb,
    pub comb_l8: Comb,
    pub comb_r8: Comb,
    pub allpass_l1: AllPass,
    pub allpass_r1: AllPass,
    pub allpass_l2: AllPass,
    pub allpass_r2: AllPass,
    pub allpass_l3: AllPass,
    pub allpass_r3: AllPass,
    pub allpass_l4: AllPass,
    pub allpass_r4: AllPass,
}

impl ReverbModel {
    pub fn new() -> Self {
        let mut rev = Self {
            roomsize: 0.5f32 * 0.28f32 + 0.7f32,
            damp: 0.2f32 * 1.0f32,
            wet: 1f32 * 3.0f32,
            wet1: 0f32,
            wet2: 0f32,
            width: 1f32,
            gain: 0.015f32,
            comb_l1: Comb::new(COMBTUNING_L1),
            comb_r1: Comb::new(COMBTUNING_R1),
            comb_l2: Comb::new(COMBTUNING_L2),
            comb_r2: Comb::new(COMBTUNING_R2),
            comb_l3: Comb::new(COMBTUNING_L3),
            comb_r3: Comb::new(COMBTUNING_R3),
            comb_l4: Comb::new(COMBTUNING_L4),
            comb_r4: Comb::new(COMBTUNING_R4),
            comb_l5: Comb::new(COMBTUNING_L5),
            comb_r5: Comb::new(COMBTUNING_R5),
            comb_l6: Comb::new(COMBTUNING_L6),
            comb_r6: Comb::new(COMBTUNING_R6),
            comb_l7: Comb::new(COMBTUNING_L7),
            comb_r7: Comb::new(COMBTUNING_R7),
            comb_l8: Comb::new(COMBTUNING_L8),
            comb_r8: Comb::new(COMBTUNING_R8),
            allpass_l1: AllPass::new(ALLPASSTUNING_L1, 0.5f32),
            allpass_r1: AllPass::new(ALLPASSTUNING_R1, 0.5f32),
            allpass_l2: AllPass::new(ALLPASSTUNING_L2, 0.5f32),
            allpass_r2: AllPass::new(ALLPASSTUNING_R2, 0.5f32),
            allpass_l3: AllPass::new(ALLPASSTUNING_L3, 0.5f32),
            allpass_r3: AllPass::new(ALLPASSTUNING_R3, 0.5f32),
            allpass_l4: AllPass::new(ALLPASSTUNING_L4, 0.5f32),
            allpass_r4: AllPass::new(ALLPASSTUNING_R4, 0.5f32),
        };
        rev.update();
        return rev;
    }

    pub fn reset(self: &mut Self) {
        self.comb_l1 = Comb::new(COMBTUNING_L1);
        self.comb_r1 = Comb::new(COMBTUNING_R1);
        self.comb_l2 = Comb::new(COMBTUNING_L2);
        self.comb_r2 = Comb::new(COMBTUNING_R2);
        self.comb_l3 = Comb::new(COMBTUNING_L3);
        self.comb_r3 = Comb::new(COMBTUNING_R3);
        self.comb_l4 = Comb::new(COMBTUNING_L4);
        self.comb_r4 = Comb::new(COMBTUNING_R4);
        self.comb_l5 = Comb::new(COMBTUNING_L5);
        self.comb_r5 = Comb::new(COMBTUNING_R5);
        self.comb_l6 = Comb::new(COMBTUNING_L6);
        self.comb_r6 = Comb::new(COMBTUNING_R6);
        self.comb_l7 = Comb::new(COMBTUNING_L7);
        self.comb_r7 = Comb::new(COMBTUNING_R7);
        self.comb_l8 = Comb::new(COMBTUNING_L8);
        self.comb_r8 = Comb::new(COMBTUNING_R8);
        self.allpass_l1 = AllPass::new(ALLPASSTUNING_L1, 0.5f32);
        self.allpass_r1 = AllPass::new(ALLPASSTUNING_R1, 0.5f32);
        self.allpass_l2 = AllPass::new(ALLPASSTUNING_L2, 0.5f32);
        self.allpass_r2 = AllPass::new(ALLPASSTUNING_R2, 0.5f32);
        self.allpass_l3 = AllPass::new(ALLPASSTUNING_L3, 0.5f32);
        self.allpass_r3 = AllPass::new(ALLPASSTUNING_R3, 0.5f32);
        self.allpass_l4 = AllPass::new(ALLPASSTUNING_L4, 0.5f32);
        self.allpass_r4 = AllPass::new(ALLPASSTUNING_R4, 0.5f32);
    }

    pub fn process_replace(
        &mut self,
        in_0: *mut f32,
        left_out: *mut f32,
        right_out: *mut f32,
    ) {
        for k in 0..64 {
            let mut out_r = 0f32;
            let mut out_l = 0f32;

            let input = unsafe {
                ((2f32 * *in_0.offset(k)) + DC_OFFSET) * self.gain
            };

            out_l += self.comb_l1.process(input);
            out_r += self.comb_r1.process(input);
            out_l += self.comb_l2.process(input);
            out_r += self.comb_r2.process(input);
            out_l += self.comb_l3.process(input);
            out_r += self.comb_r3.process(input);
            out_l += self.comb_l4.process(input);
            out_r += self.comb_r4.process(input);
            out_l += self.comb_l5.process(input);
            out_r += self.comb_r5.process(input);
            out_l += self.comb_l6.process(input);
            out_r += self.comb_r6.process(input);
            out_l += self.comb_l7.process(input);
            out_r += self.comb_r7.process(input);
            out_l += self.comb_l8.process(input);
            out_r += self.comb_r8.process(input);
            
            out_l = self.allpass_l1.process(out_l);
            out_r = self.allpass_r1.process(out_r);
            out_l = self.allpass_l2.process(out_l);
            out_r = self.allpass_r2.process(out_r);
            out_l = self.allpass_l3.process(out_l);
            out_r = self.allpass_r3.process(out_r);
            out_l = self.allpass_l4.process(out_l);
            out_r = self.allpass_r4.process(out_r);

            out_l -= DC_OFFSET;
            out_r -= DC_OFFSET;

            unsafe {
                *left_out.offset(k) = out_l * self.wet1 + out_r * self.wet2;
                *right_out.offset(k) = out_r * self.wet1 + out_l * self.wet2;
            }
        }
    }

    pub fn process_mix(
        &mut self,
        in_0: *mut f32,
        left_out: *mut f32,
        right_out: *mut f32,
    ) {
        for k in 0..64 {
            let mut out_r = 0f32;
            let mut out_l = out_r;
            let input = unsafe {
                ((2f32 * *in_0.offset(k as isize)) + DC_OFFSET) * self.gain
            };

            out_l += self.comb_l1.process(input);
            out_r += self.comb_r1.process(input);
            out_l += self.comb_l2.process(input);
            out_r += self.comb_r2.process(input);
            out_l += self.comb_l3.process(input);
            out_r += self.comb_r3.process(input);
            out_l += self.comb_l4.process(input);
            out_r += self.comb_r4.process(input);
            out_l += self.comb_l5.process(input);
            out_r += self.comb_r5.process(input);
            out_l += self.comb_l6.process(input);
            out_r += self.comb_r6.process(input);
            out_l += self.comb_l7.process(input);
            out_r += self.comb_r7.process(input);
            out_l += self.comb_l8.process(input);
            out_r += self.comb_r8.process(input);
            
            out_l = self.allpass_l1.process(out_l);
            out_r = self.allpass_r1.process(out_r);
            out_l = self.allpass_l2.process(out_l);
            out_r = self.allpass_r2.process(out_r);
            out_l = self.allpass_l3.process(out_l);
            out_r = self.allpass_r3.process(out_r);
            out_l = self.allpass_l4.process(out_l);
            out_r = self.allpass_r4.process(out_r);

            out_l -= DC_OFFSET;
            out_r -= DC_OFFSET;

            unsafe {
                *left_out.offset(k as isize) += out_l * self.wet1 + out_r * self.wet2;
                *right_out.offset(k as isize) += out_r * self.wet1 + out_l * self.wet2;
            }
        }
    }

    pub fn update(&mut self) {
        self.wet1 = self.wet * (self.width / 2f32 + 0.5f32);
        self.wet2 = self.wet * ((1f32 - self.width) / 2f32);
        self.comb_l1.set_feedback(self.roomsize);
        self.comb_r1.set_feedback(self.roomsize);
        self.comb_l2.set_feedback(self.roomsize);
        self.comb_r2.set_feedback(self.roomsize);
        self.comb_l3.set_feedback(self.roomsize);
        self.comb_r3.set_feedback(self.roomsize);
        self.comb_l4.set_feedback(self.roomsize);
        self.comb_r4.set_feedback(self.roomsize);
        self.comb_l5.set_feedback(self.roomsize);
        self.comb_r5.set_feedback(self.roomsize);
        self.comb_l6.set_feedback(self.roomsize);
        self.comb_r6.set_feedback(self.roomsize);
        self.comb_l7.set_feedback(self.roomsize);
        self.comb_r7.set_feedback(self.roomsize);
        self.comb_l8.set_feedback(self.roomsize);
        self.comb_r8.set_feedback(self.roomsize);
        self.comb_l1.set_feedback(self.roomsize);
        self.comb_r1.set_damp(self.damp);
        self.comb_l2.set_damp(self.damp);
        self.comb_r2.set_damp(self.damp);
        self.comb_l3.set_damp(self.damp);
        self.comb_r3.set_damp(self.damp);
        self.comb_l4.set_damp(self.damp);
        self.comb_r4.set_damp(self.damp);
        self.comb_l5.set_damp(self.damp);
        self.comb_r5.set_damp(self.damp);
        self.comb_l6.set_damp(self.damp);
        self.comb_r6.set_damp(self.damp);
        self.comb_l7.set_damp(self.damp);
        self.comb_r7.set_damp(self.damp);
        self.comb_l8.set_damp(self.damp);
        self.comb_r8.set_damp(self.damp);
    }

    pub fn set_room_size(&mut self, value: f32) {
        self.roomsize = value * 0.28f32 + 0.7f32;
        self.update();
    }

    pub fn get_room_size(&self) -> f32 {
        return (self.roomsize - 0.7f32) / 0.28f32;
    }

    pub fn set_damp(&mut self, value: f32) {
        self.damp = value * 1.0f32;
        self.update();
    }

    pub fn get_damp(&self) -> f32 {
        return self.damp / 1.0f32;
    }

    pub fn set_level(&mut self, mut value: f32) {
        value = if value < 0.0f32 {
            0.0f32
        } else if value > 1.0f32 {
            1.0f32
        } else {
            value
        };
        self.wet = value * 3.0f32;
        self.update();
    }

    pub fn get_level(&self) -> f32 {
        return self.wet / 3.0f32;
    }

    pub fn set_width(&mut self, value: f32) {
        self.width = value;
        self.update();
    }

    pub fn get_width(&self) -> f32 {
        return self.width;
    }
}
