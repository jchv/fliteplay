#[derive(Clone)]
pub struct Tuning {
    name: Vec<u8>,
    pub(crate) bank: i32,
    pub(crate) prog: i32,
    pub(crate) pitch: [f64; 128],
}

pub fn new_fluid_tuning(name: &[u8], bank: i32, prog: i32) -> Tuning {
    let mut tuning = Tuning {
        name: name.to_vec(),
        bank,
        prog,
        pitch: [0f64; 128],
    };
    for i in 0..128 {
        tuning.pitch[i] = i as f64 * 100.0f64;
    }
    return tuning;
}

pub fn fluid_tuning_set_name(tuning: &mut Tuning, name: &[u8]) {
    tuning.name = name.to_vec();
}

pub fn fluid_tuning_get_name(tuning: &Tuning) -> &[u8] {
    return &tuning.name;
}

pub fn fluid_tuning_set_octave(tuning: &mut Tuning, pitch_deriv: &[f64; 12]) {
    let mut i;
    i = 0 as i32;
    while i < 128 as i32 {
        tuning.pitch[i as usize] = i as f64 * 100.0f64 + pitch_deriv[i as usize % 12];
        i += 1
    }
}

pub fn fluid_tuning_set_all(tuning: &mut Tuning, pitch: &[f64; 128]) {
    for i in 0..128 {
        tuning.pitch[i] = pitch[i];
    }
}

pub fn fluid_tuning_set_pitch(tuning: &mut Tuning, key: i32, pitch: f64) {
    if key >= 0 as i32 && key < 128 as i32 {
        tuning.pitch[key as usize] = pitch
    };
}
