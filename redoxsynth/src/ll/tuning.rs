#[derive(Clone)]
pub struct Tuning {
    name: Vec<u8>,
    pub(crate) bank: i32,
    pub(crate) prog: i32,
    pub(crate) pitch: [f64; 128],
}

pub unsafe fn new_fluid_tuning(
    name: &[u8],
    bank: i32,
    prog: i32,
) -> *mut Tuning {
    let mut tuning;
    let mut i;
    tuning = libc::malloc(::std::mem::size_of::<Tuning>() as libc::size_t) as *mut Tuning;
    if tuning.is_null() {
        fluid_log!(FLUID_PANIC as i32, "Out of memory",);
        return 0 as *mut Tuning;
    }
    libc::memset(tuning as _, 0, std::mem::size_of::<Tuning>() as _);
    (*tuning).name = name.to_vec();
    (*tuning).bank = bank;
    (*tuning).prog = prog;
    i = 0 as i32;
    while i < 128 as i32 {
        (*tuning).pitch[i as usize] = i as f64 * 100.0f64;
        i += 1
    }
    return tuning;
}

pub unsafe fn fluid_tuning_set_name(tuning: &mut Tuning, name: &[u8]) {
    tuning.name = name.to_vec();
}

pub unsafe fn fluid_tuning_get_name(tuning: &Tuning) -> &[u8] {
    return &tuning.name;
}

pub unsafe fn fluid_tuning_set_octave(tuning: &mut Tuning, pitch_deriv: &[f64; 12]) {
    let mut i;
    i = 0 as i32;
    while i < 128 as i32 {
        tuning.pitch[i as usize] =
            i as f64 * 100.0f64 + pitch_deriv[i as usize % 12];
        i += 1
    }
}

pub unsafe fn fluid_tuning_set_all(tuning: &mut Tuning, pitch: *mut f64) {
    let mut i;
    i = 0 as i32;
    while i < 128 as i32 {
        tuning.pitch[i as usize] = *pitch.offset(i as isize);
        i += 1
    }
}

pub unsafe fn fluid_tuning_set_pitch(tuning: &mut Tuning, key: i32, pitch: f64) {
    if key >= 0 as i32 && key < 128 as i32 {
        tuning.pitch[key as usize] = pitch
    };
}
