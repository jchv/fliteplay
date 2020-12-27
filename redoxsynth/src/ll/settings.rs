use super::synth::fluid_synth_settings;
use super::sys::fluid_strtok;
use std::{collections::HashMap, ffi::CStr};
pub type SettingsType = i32;
pub const FLUID_SET_TYPE: SettingsType = 3;
pub const FLUID_STR_TYPE: SettingsType = 2;
pub const FLUID_INT_TYPE: SettingsType = 1;
pub const FLUID_NUM_TYPE: SettingsType = 0;
pub const FLUID_NO_TYPE: SettingsType = -1;

pub enum Setting {
    Int(IntSetting),
    Str(StrSetting),
    Num(NumSetting),
    Set(HashMap<String, Setting>),
}

#[derive(Clone)]
pub struct StrSetting {
    value: *mut libc::c_char,
    def: *mut libc::c_char,
    hints: i32,
    update: StrUpdateFn,
    data: *mut libc::c_void,
}
pub type StrUpdateFn = Option<
    unsafe fn(_: *mut libc::c_void, _: *const libc::c_char, _: *mut libc::c_char) -> i32,
>;
#[derive(Clone)]
pub struct IntSetting {
    value: i32,
    def: i32,
    min: i32,
    max: i32,
    hints: i32,
    update: IntUpdateFn,
    data: *mut libc::c_void,
}
pub type IntUpdateFn =
    Option<unsafe fn(_: *mut libc::c_void, _: *const libc::c_char, _: i32) -> i32>;
#[derive(Clone)]
pub struct NumSetting {
    value: f64,
    def: f64,
    min: f64,
    max: f64,
    hints: i32,
    update: NumUpdateFn,
    data: *mut libc::c_void,
}

pub type NumUpdateFn =
    Option<unsafe fn(_: *mut libc::c_void, _: *const libc::c_char, _: f64) -> i32>;

unsafe fn new_fluid_str_setting(
    value: *const libc::c_char,
    def: *mut libc::c_char,
    hints: i32,
    fun: StrUpdateFn,
    data: *mut libc::c_void,
) -> StrSetting {
    return StrSetting{
        value: if !value.is_null() {
            libc::strcpy(
                libc::malloc(libc::strlen(value) + 1) as *mut libc::c_char,
                value,
            )
        } else {
            0 as *mut libc::c_char
        },
        def: if !def.is_null() {
            libc::strcpy(
                libc::malloc(libc::strlen(def) + 1) as *mut libc::c_char,
                def,
            )
        } else {
            0 as *mut libc::c_char
        },
        hints,
        update: fun,
        data,
    };
}

impl Drop for StrSetting {
    fn drop(&mut self) {
        unsafe {
            if !self.value.is_null() {
                libc::free(self.value as *mut libc::c_void);
            }
            if !self.def.is_null() {
                libc::free(self.def as *mut libc::c_void);
            }
        }
    }
}

unsafe fn new_fluid_num_setting(
    min: f64,
    max: f64,
    def: f64,
    hints: i32,
    fun: NumUpdateFn,
    data: *mut libc::c_void,
) -> NumSetting {
    return NumSetting{
        value: def,
        def,
        min,
        max,
        hints,
        update: fun,
        data,
    };
}

unsafe fn new_fluid_int_setting(
    min: i32,
    max: i32,
    def: i32,
    hints: i32,
    fun: IntUpdateFn,
    data: *mut libc::c_void,
) -> IntSetting {
    return IntSetting{
        value: def,
        def,
        min,
        max,
        hints,
        update: fun,
        data,
    };
}

pub struct Settings {
    table: HashMap<String, Setting>
}

pub unsafe fn new_fluid_settings() -> Settings {
    let mut settings = Settings{ table: HashMap::new() };
    fluid_settings_init(&mut settings);
    return settings;
}

unsafe fn fluid_settings_init(settings: &mut Settings) {
    fluid_synth_settings(settings);
}
unsafe fn fluid_settings_tokenize(
    s: *const libc::c_char,
    buf: *mut libc::c_char,
    ptr: *mut *mut libc::c_char,
) -> i32 {
    let mut tokstr;
    let mut tok;
    let mut n: i32 = 0 as i32;
    if libc::strlen(s) > 256 {
        fluid_log!(
            FLUID_ERR,
            "Setting variable name exceeded max length of {} chars",
            256
        );
        return 0 as i32;
    }
    libc::strcpy(buf, s);
    tokstr = buf;
    loop {
        tok = fluid_strtok(
            &mut tokstr,
            b".\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if tok.is_null() {
            break;
        }
        if n > 8 as i32 {
            fluid_log!(
                FLUID_ERR,
                "Setting variable name exceeded max token count of {}",
                8
            );
            return 0 as i32;
        }
        let fresh0 = n;
        n = n + 1;
        let ref mut fresh1 = *ptr.offset(fresh0 as isize);
        *fresh1 = tok
    }
    return n;
}

unsafe fn fluid_settings_get(
    settings: &Settings,
    name: *mut *mut libc::c_char,
    len: i32,
) -> Option<&Setting> {
    let mut table = &settings.table;
    for n in 0..len - 1 {
        match table.get(CStr::from_ptr(*name.offset(n as isize)).to_str().unwrap()) {
            Some(Setting::Set(t)) => table = t,
            _ => return None
        }
    }
    return table.get(CStr::from_ptr(*name.offset(len as isize - 1)).to_str().unwrap());
}

unsafe fn fluid_settings_get_mut(
    settings: &mut Settings,
    name: *mut *mut libc::c_char,
    len: i32,
) -> Option<&mut Setting> {
    let mut table = &mut settings.table;
    for n in 0..len - 1 {
        match table.get_mut(CStr::from_ptr(*name.offset(n as isize)).to_str().unwrap()) {
            Some(Setting::Set(t)) => table = t,
            _ => return None
        }
    }
    return table.get_mut(CStr::from_ptr(*name.offset(len as isize - 1)).to_str().unwrap());
}

unsafe fn fluid_settings_set(
    settings: &mut Settings,
    name: *mut *mut libc::c_char,
    len: i32,
    value: Setting,
) -> i32 {
    let mut table = &mut settings.table;
    for n in 0..len - 1 {
        let key = CStr::from_ptr(*name.offset(n as isize)).to_str().unwrap();
        if table.get(key).is_none() {
            let t = HashMap::new();
            table.insert(key.to_string(), Setting::Set(t));
        }
        table = match table.get_mut(&key.to_string()) {
            Some(Setting::Set(t)) => t,
            _ => return 0
        };
    }
    table.insert(CStr::from_ptr(*name.offset(len as isize - 1)).to_str().unwrap().to_string(), value);
    return 1;
}

pub unsafe fn fluid_settings_register_str(
    settings: &mut Settings,
    name: *const libc::c_char,
    def: *mut libc::c_char,
    hints: i32,
    fun: StrUpdateFn,
    data: *mut libc::c_void,
) -> i32 {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    match fluid_settings_get_mut(
        settings,
        tokens.as_mut_ptr(),
        ntokens
    ) {
        None => {
            return fluid_settings_set(
                settings,
                tokens.as_mut_ptr(),
                ntokens,
                Setting::Str(new_fluid_str_setting(def, def, hints, fun, data)),
            );
        },
        Some(Setting::Str(setting)) => {
            setting.update = fun;
            setting.data = data;
            setting.def = if !def.is_null() {
                libc::strcpy(
                    libc::malloc(libc::strlen(def).wrapping_add(1)) as *mut libc::c_char,
                    def,
                )
            } else {
                0 as *mut libc::c_char
            };
            setting.hints = hints;
            return 1 as i32;
        },
        _ => {
            fluid_log!(
                FLUID_WARN,
                "Type mismatch on setting \'{}\'",
                CStr::from_ptr(name).to_str().unwrap()
            );
            return 1 as i32;
        }
    }
}

pub fn fluid_settings_register_num(
    settings: &mut Settings,
    name: *const libc::c_char,
    def: f64,
    min: f64,
    max: f64,
    hints: i32,
    fun: NumUpdateFn,
    data: *mut libc::c_void,
) -> i32 {
    unsafe {
        let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
        let mut buf: [libc::c_char; 257] = [0; 257];
        let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
        match fluid_settings_get_mut(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
        ) {
            None => {
                return fluid_settings_set(
                    settings,
                    tokens.as_mut_ptr(),
                    ntokens,
                    Setting::Num(new_fluid_num_setting(min, max, def, hints, fun, data))
                );
            },
            Some(Setting::Num(setting)) => {
                setting.update = fun;
                setting.data = data;
                setting.min = min;
                setting.max = max;
                setting.def = def;
                setting.hints = hints;
                return 1 as i32;
            },
            _ => {
                fluid_log!(
                    FLUID_WARN,
                    "Type mismatch on setting \'{}\'",
                    CStr::from_ptr(name).to_str().unwrap()
                );
                return 0 as i32;
            }
        };
    }
}

pub fn fluid_settings_register_int(
    settings: &mut Settings,
    name: *const libc::c_char,
    def: i32,
    min: i32,
    max: i32,
    hints: i32,
    fun: IntUpdateFn,
    data: *mut libc::c_void,
) -> i32 {
    unsafe {
        let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
        let mut buf: [libc::c_char; 257] = [0; 257];
        let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
        match fluid_settings_get_mut(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
        ) {
            Some(Setting::Int(setting)) => {
                setting.update = fun;
                setting.data = data;
                setting.min = min;
                setting.max = max;
                setting.def = def;
                setting.hints = hints;
                return 1 as i32;
            },
            None => {
                return fluid_settings_set(
                    settings,
                    tokens.as_mut_ptr(),
                    ntokens,
                    Setting::Int(new_fluid_int_setting(min, max, def, hints, fun, data))
                );
            },
            _ => {
                return 0;
            }
        }
    }
}

pub unsafe fn fluid_settings_get_hints(
    settings: &Settings,
    name: *const libc::c_char,
) -> i32 {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    return match fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens
    ) {
        Some(Setting::Num(s)) => s.hints,
        Some(Setting::Str(s)) => s.hints,
        _ => 0
    }
}

pub unsafe fn fluid_settings_is_realtime(
    settings: &Settings,
    name: *const libc::c_char,
) -> bool {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    return match fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
    ) {
        Some(Setting::Num(s)) => s.update.is_some(),
        Some(Setting::Str(s)) => s.update.is_some(),
        _ => false
    }
}

pub unsafe fn fluid_settings_setstr(
    settings: &mut Settings,
    name: *const libc::c_char,
    str: *const libc::c_char,
) -> i32 {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    match fluid_settings_get_mut(
        settings,
        tokens.as_mut_ptr(),
        ntokens
    ) {
        Some(Setting::Str(setting)) => {
            if !setting.value.is_null() {
                libc::free(setting.value as *mut libc::c_void);
            }
            setting.value = if !str.is_null() {
                libc::strcpy(
                    libc::malloc(libc::strlen(str).wrapping_add(1)) as *mut libc::c_char,
                    str,
                )
            } else {
                0 as *mut libc::c_char
            };
            if setting.update.is_some() {
                Some(setting.update.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    setting.data, name, setting.value
                );
            }
            return 1;
        },
        None => {
            let setting_0;
            setting_0 = new_fluid_str_setting(
                str,
                0 as *mut libc::c_char,
                0 as i32,
                None,
                0 as *mut libc::c_void,
            );
            return fluid_settings_set(
                settings,
                tokens.as_mut_ptr(),
                ntokens,
                Setting::Str(setting_0)
            );
        },
        _ => return 0
    }
}

pub unsafe fn fluid_settings_getstr(
    settings: &Settings,
    name: *const libc::c_char,
) -> Option<*mut libc::c_char> {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    
    return match fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
    ) {
        Some(Setting::Str(s)) => Some(s.value),
        _ => None,
    }
}

pub fn fluid_settings_str_equal(
    settings: &Settings,
    name: *const libc::c_char,
    s: *mut libc::c_char,
) -> bool {
    unsafe {
        let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
        let mut buf: [libc::c_char; 257] = [0; 257];
        let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
        return match fluid_settings_get(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
        ) {
            Some(Setting::Str(setting)) => libc::strcmp(setting.value, s) == 0,
            _ => false,
        };
    }
}

pub unsafe fn fluid_settings_getstr_default(
    settings: &Settings,
    name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    
    return match fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
    ) {
        Some(Setting::Str(s)) => s.def,
        _ => 0 as _,
    }
}

pub unsafe fn fluid_settings_setnum(
    settings: &mut Settings,
    name: *const libc::c_char,
    mut val: f64,
) -> i32 {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    match fluid_settings_get_mut(
        settings,
        tokens.as_mut_ptr(),
        ntokens
    ) {
        Some(Setting::Num(setting)) => {
            if val < setting.min {
                val = setting.min
            } else if val > setting.max {
                val = setting.max
            }
            setting.value = val;
            if setting.update.is_some() {
                Some(setting.update.expect("non-null function pointer"))
                    .expect("non-null function pointer")(setting.data, name, val);
            }
            return 1;
        },
        None => {
            let mut setting_0;
            setting_0 = new_fluid_num_setting(
                -1e10f64,
                1e10f64,
                0.0f32 as f64,
                0 as i32,
                None,
                0 as *mut libc::c_void,
            );
            setting_0.value = val;
            return fluid_settings_set(
                settings,
                tokens.as_mut_ptr(),
                ntokens,
                Setting::Num(setting_0),
            );
        },
        _ => return 0
    }
}

pub fn fluid_settings_getnum(
    settings: &Settings,
    name: *const libc::c_char
) -> Option<f64> {
    unsafe {
        let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
        let mut buf: [libc::c_char; 257] = [0; 257];
        let ntokens;
        ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
        
        return match fluid_settings_get(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
        ) {
            Some(Setting::Num(s)) => Some(s.value),
            _ => None,
        }
    }
}

pub struct Range<T> {
    pub min: T,
    pub max: T,
}

pub unsafe fn fluid_settings_getnum_range(
    settings: &Settings,
    name: *const libc::c_char
) -> Option<Range<f64>> {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    
    return match fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
    ) {
        Some(Setting::Num(s)) => Some(Range{min: s.min, max: s.max}),
        _ => None,
    }
}

pub unsafe fn fluid_settings_getnum_default(
    settings: &Settings,
    name: *const libc::c_char,
) -> f64 {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    
    return match fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
    ) {
        Some(Setting::Num(s)) => s.def,
        _ => 0f64,
    }
}

pub fn fluid_settings_setint(
    settings: &mut Settings,
    name: *const libc::c_char,
    mut val: i32,
) -> i32 {
    unsafe {
        let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
        let mut buf: [libc::c_char; 257] = [0; 257];
        let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
        match fluid_settings_get_mut(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
        ) {
            Some(Setting::Int(setting)) => {
                if val < (*setting).min {
                    val = (*setting).min
                } else if val > (*setting).max {
                    val = (*setting).max
                }
                setting.value = val;
                if setting.update.is_some() {
                    Some(setting.update.expect("non-null function pointer"))
                        .expect("non-null function pointer")(setting.data, name, val);
                }
                return 1;
            },
            None => {
                let mut setting_0;
                setting_0 = new_fluid_int_setting(
                    -(2147483647 as i32) - 1 as i32,
                    2147483647 as i32,
                    0 as i32,
                    0 as i32,
                    None,
                    0 as *mut libc::c_void,
                );
                setting_0.value = val;
                return fluid_settings_set(
                    settings,
                    tokens.as_mut_ptr(),
                    ntokens,
                    Setting::Int(setting_0)
                );
            }
            _ => {
                return 0;
            }
        }
    }
}

pub fn fluid_settings_getint(
    settings: &Settings,
    name: *const libc::c_char,
) -> Option<i32> {
    unsafe {
        let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
        let mut buf: [libc::c_char; 257] = [0; 257];
        let ntokens;
        ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
        
        return match fluid_settings_get(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
        ) {
            Some(Setting::Int(s)) => Some(s.value),
            _ => None,
        };
    }
}

pub unsafe fn fluid_settings_getint_range(
    settings: &Settings,
    name: *const libc::c_char,
) -> Option<Range<i32>> {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    
    return match fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
    ) {
        Some(Setting::Int(s)) => Some(Range{min: s.min, max: s.max}),
        _ => None,
    }
}

pub unsafe fn fluid_settings_getint_default(
    settings: &Settings,
    name: *const libc::c_char,
) -> i32 {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    
    return match fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
    ) {
        Some(Setting::Int(s)) => s.def,
        _ => 0,
    }
}

pub fn fluid_settings_get_type(settings: &Settings, name: *const libc::c_char) -> SettingsType  {
    unsafe {
        let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
        let mut buf: [libc::c_char; 257] = [0; 257];
        let ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
        return match fluid_settings_get(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
        ) {
            Some(Setting::Num(_)) => FLUID_SET_TYPE,
            Some(Setting::Str(_)) => FLUID_STR_TYPE,
            Some(Setting::Int(_)) => FLUID_INT_TYPE,
            Some(Setting::Set(_)) => FLUID_NUM_TYPE,
            _ => FLUID_NO_TYPE
        }
    }
}
