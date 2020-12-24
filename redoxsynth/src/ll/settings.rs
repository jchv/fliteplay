use super::hash::delete_fluid_hashtable;
use super::hash::fluid_hashtable_insert;
use super::hash::fluid_hashtable_lookup;
use super::hash::fluid_hashtable_replace;
use super::hash::new_fluid_hashtable;
use super::hash::HashTable;
use super::list::delete_fluid_list;
use super::list::List;
use super::synth::fluid_synth_settings;
use super::sys::fluid_strtok;
use std::ffi::CStr;
pub type Settings = HashTable;
pub type SettingsType = i32;
pub const FLUID_SET_TYPE: SettingsType = 3;
pub const FLUID_STR_TYPE: SettingsType = 2;
pub const FLUID_INT_TYPE: SettingsType = 1;
pub const FLUID_NUM_TYPE: SettingsType = 0;
pub const FLUID_NO_TYPE: SettingsType = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StrSetting {
    pub value: *mut libc::c_char,
    pub def: *mut libc::c_char,
    pub hints: i32,
    pub options: *mut List,
    pub update: StrUpdateFn,
    pub data: *mut libc::c_void,
}
pub type StrUpdateFn = Option<
    unsafe fn(_: *mut libc::c_void, _: *const libc::c_char, _: *mut libc::c_char) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IntSetting {
    pub value: i32,
    pub def: i32,
    pub min: i32,
    pub max: i32,
    pub hints: i32,
    pub update: IntUpdateFn,
    pub data: *mut libc::c_void,
}
pub type IntUpdateFn =
    Option<unsafe fn(_: *mut libc::c_void, _: *const libc::c_char, _: i32) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NumSetting {
    pub value: f64,
    pub def: f64,
    pub min: f64,
    pub max: f64,
    pub hints: i32,
    pub update: NumUpdateFn,
    pub data: *mut libc::c_void,
}
pub type NumUpdateFn =
    Option<unsafe fn(_: *mut libc::c_void, _: *const libc::c_char, _: f64) -> i32>;
unsafe fn new_fluid_str_setting(
    value: *const libc::c_char,
    def: *mut libc::c_char,
    hints: i32,
    fun: StrUpdateFn,
    data: *mut libc::c_void,
) -> *mut StrSetting {
    let mut str;
    str = libc::malloc(::std::mem::size_of::<StrSetting>() as libc::size_t) as *mut StrSetting;
    (*str).value = if !value.is_null() {
        libc::strcpy(
            libc::malloc(libc::strlen(value) + 1) as *mut libc::c_char,
            value,
        )
    } else {
        0 as *mut libc::c_char
    };
    (*str).def = if !def.is_null() {
        libc::strcpy(
            libc::malloc(libc::strlen(def) + 1) as *mut libc::c_char,
            def,
        )
    } else {
        0 as *mut libc::c_char
    };
    (*str).hints = hints;
    (*str).options = 0 as *mut List;
    (*str).update = fun;
    (*str).data = data;
    return str;
}
unsafe fn delete_fluid_str_setting(str: *mut StrSetting) {
    if !str.is_null() {
        if !(*str).value.is_null() {
            libc::free((*str).value as *mut libc::c_void);
        }
        if !(*str).def.is_null() {
            libc::free((*str).def as *mut libc::c_void);
        }
        if !(*str).options.is_null() {
            let mut list: *mut List = (*str).options;
            while !list.is_null() {
                libc::free((*list).data);
                list = if !list.is_null() {
                    (*list).next
                } else {
                    0 as *mut List
                }
            }
            delete_fluid_list((*str).options);
        }
        libc::free(str as *mut libc::c_void);
    };
}
unsafe fn new_fluid_num_setting(
    min: f64,
    max: f64,
    def: f64,
    hints: i32,
    fun: NumUpdateFn,
    data: *mut libc::c_void,
) -> *mut NumSetting {
    let mut setting;
    setting = libc::malloc(::std::mem::size_of::<NumSetting>() as libc::size_t) as *mut NumSetting;
    (*setting).value = def;
    (*setting).def = def;
    (*setting).min = min;
    (*setting).max = max;
    (*setting).hints = hints;
    (*setting).update = fun;
    (*setting).data = data;
    return setting;
}
unsafe fn delete_fluid_num_setting(setting: *mut NumSetting) {
    if !setting.is_null() {
        libc::free(setting as *mut libc::c_void);
    };
}
unsafe fn new_fluid_int_setting(
    min: i32,
    max: i32,
    def: i32,
    hints: i32,
    fun: IntUpdateFn,
    data: *mut libc::c_void,
) -> *mut IntSetting {
    let mut setting;
    setting = libc::malloc(::std::mem::size_of::<IntSetting>() as libc::size_t) as *mut IntSetting;
    (*setting).value = def;
    (*setting).def = def;
    (*setting).min = min;
    (*setting).max = max;
    (*setting).hints = hints;
    (*setting).update = fun;
    (*setting).data = data;
    return setting;
}
unsafe fn delete_fluid_int_setting(setting: *mut IntSetting) {
    if !setting.is_null() {
        libc::free(setting as *mut libc::c_void);
    };
}

pub unsafe fn new_fluid_settings() -> *mut Settings {
    let settings: *mut Settings = new_fluid_hashtable(Some(
        fluid_settings_hash_delete as unsafe fn(_: *mut libc::c_void, _: i32) -> (),
    ));
    if settings.is_null() {
        return 0 as *mut Settings;
    }
    fluid_settings_init(settings);
    return settings;
}

pub unsafe fn delete_fluid_settings(settings: *mut Settings) {
    delete_fluid_hashtable(settings);
}
unsafe fn fluid_settings_hash_delete(value: *mut libc::c_void, type_0: i32) {
    match type_0 {
        0 => {
            delete_fluid_num_setting(value as *mut NumSetting);
        }
        1 => {
            delete_fluid_int_setting(value as *mut IntSetting);
        }
        2 => {
            delete_fluid_str_setting(value as *mut StrSetting);
        }
        3 => {
            delete_fluid_hashtable(value as *mut HashTable);
        }
        _ => {}
    };
}
unsafe fn fluid_settings_init(settings: *mut Settings) {
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
    settings: *mut Settings,
    name: *mut *mut libc::c_char,
    len: i32,
    value: *mut *mut libc::c_void,
    type_0: *mut i32,
) -> i32 {
    let mut table: *mut HashTable = settings;
    let mut t: i32 = 0;
    let mut v: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n;
    n = 0 as i32;
    while n < len {
        if table.is_null() {
            return 0 as i32;
        }
        if fluid_hashtable_lookup(table, *name.offset(n as isize), &mut v, &mut t) == 0 {
            return 0 as i32;
        }
        table = if t == FLUID_SET_TYPE as i32 {
            v as *mut HashTable
        } else {
            0 as *mut HashTable
        };
        n += 1
    }
    if !value.is_null() {
        *value = v
    }
    if !type_0.is_null() {
        *type_0 = t
    }
    return 1 as i32;
}
unsafe fn fluid_settings_set(
    settings: *mut Settings,
    name: *mut *mut libc::c_char,
    len: i32,
    value: *mut libc::c_void,
    type_0: i32,
) -> i32 {
    let mut table: *mut HashTable = settings;
    let mut t: i32 = 0;
    let mut v: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n;
    let num: i32 = len - 1 as i32;
    n = 0 as i32;
    while n < num {
        if fluid_hashtable_lookup(table, *name.offset(n as isize), &mut v, &mut t) != 0 {
            if t == FLUID_SET_TYPE as i32 {
                table = v as *mut HashTable
            } else {
                fluid_log!(
                    FLUID_WARN,
                    "\'{}\' is not a node",
                    CStr::from_ptr(*name.offset(n as isize)).to_str().unwrap()
                );
                return 0 as i32;
            }
        } else {
            let tmp;
            tmp = new_fluid_hashtable(Some(
                fluid_settings_hash_delete as unsafe fn(_: *mut libc::c_void, _: i32) -> (),
            ));
            fluid_hashtable_insert(
                table,
                *name.offset(n as isize),
                tmp as *mut libc::c_void,
                FLUID_SET_TYPE as i32,
            );
            table = tmp
        }
        n += 1
    }
    fluid_hashtable_replace(table, *name.offset(num as isize), value, type_0);
    return 1 as i32;
}

pub unsafe fn fluid_settings_register_str(
    settings: *mut Settings,
    name: *const libc::c_char,
    def: *mut libc::c_char,
    hints: i32,
    fun: StrUpdateFn,
    data: *mut libc::c_void,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    let mut setting;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) == 0
    {
        setting = new_fluid_str_setting(def, def, hints, fun, data);
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting as *mut libc::c_void,
            FLUID_STR_TYPE as i32,
        );
    } else if type_0 == FLUID_STR_TYPE as i32 {
        setting = value as *mut StrSetting;
        (*setting).update = fun;
        (*setting).data = data;
        (*setting).def = if !def.is_null() {
            libc::strcpy(
                libc::malloc(libc::strlen(def).wrapping_add(1)) as *mut libc::c_char,
                def,
            )
        } else {
            0 as *mut libc::c_char
        };
        (*setting).hints = hints;
        return 1 as i32;
    } else {
        fluid_log!(
            FLUID_WARN,
            "Type mismatch on setting \'{}\'",
            CStr::from_ptr(name).to_str().unwrap()
        );
        return 1 as i32;
    };
}

pub unsafe fn fluid_settings_register_num(
    settings: *mut Settings,
    name: *const libc::c_char,
    def: f64,
    min: f64,
    max: f64,
    hints: i32,
    fun: NumUpdateFn,
    data: *mut libc::c_void,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) == 0
    {
        let setting;
        setting = new_fluid_num_setting(min, max, def, hints, fun, data);
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting as *mut libc::c_void,
            FLUID_NUM_TYPE as i32,
        );
    } else if type_0 == FLUID_NUM_TYPE as i32 {
        let mut setting_0: *mut NumSetting = value as *mut NumSetting;
        (*setting_0).update = fun;
        (*setting_0).data = data;
        (*setting_0).min = min;
        (*setting_0).max = max;
        (*setting_0).def = def;
        (*setting_0).hints = hints;
        return 1 as i32;
    } else {
        fluid_log!(
            FLUID_WARN,
            "Type mismatch on setting \'{}\'",
            CStr::from_ptr(name).to_str().unwrap()
        );
        return 0 as i32;
    };
}

pub unsafe fn fluid_settings_register_int(
    settings: *mut Settings,
    name: *const libc::c_char,
    def: i32,
    min: i32,
    max: i32,
    hints: i32,
    fun: IntUpdateFn,
    data: *mut libc::c_void,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) == 0
    {
        let setting;
        setting = new_fluid_int_setting(min, max, def, hints, fun, data);
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting as *mut libc::c_void,
            FLUID_INT_TYPE as i32,
        );
    } else if type_0 == FLUID_INT_TYPE as i32 {
        let mut setting_0: *mut IntSetting = value as *mut IntSetting;
        (*setting_0).update = fun;
        (*setting_0).data = data;
        (*setting_0).min = min;
        (*setting_0).max = max;
        (*setting_0).def = def;
        (*setting_0).hints = hints;
        return 1 as i32;
    } else {
        fluid_log!(
            FLUID_WARN,
            "Type mismatch on setting \'{}\'",
            CStr::from_ptr(name).to_str().unwrap()
        );
        return 0 as i32;
    };
}

pub unsafe fn fluid_settings_get_type(
    settings: *mut Settings,
    name: *const libc::c_char,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    return if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        type_0
    } else {
        FLUID_NO_TYPE as i32
    };
}

pub unsafe fn fluid_settings_get_hints(
    settings: *mut Settings,
    name: *const libc::c_char,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 == FLUID_NUM_TYPE as i32 {
            let setting: *mut NumSetting = value as *mut NumSetting;
            return (*setting).hints;
        } else if type_0 == FLUID_STR_TYPE as i32 {
            let setting_0: *mut StrSetting = value as *mut StrSetting;
            return (*setting_0).hints;
        } else {
            return 0 as i32;
        }
    } else {
        return 0 as i32;
    };
}

pub unsafe fn fluid_settings_is_realtime(
    settings: *mut Settings,
    name: *const libc::c_char,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 == FLUID_NUM_TYPE as i32 {
            let setting: *mut NumSetting = value as *mut NumSetting;
            return (*setting).update.is_some() as i32;
        } else if type_0 == FLUID_STR_TYPE as i32 {
            let setting_0: *mut StrSetting = value as *mut StrSetting;
            return (*setting_0).update.is_some() as i32;
        } else {
            return 0 as i32;
        }
    } else {
        return 0 as i32;
    };
}

pub unsafe fn fluid_settings_setstr(
    settings: *mut Settings,
    name: *const libc::c_char,
    str: *const libc::c_char,
) -> i32 {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut setting;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 != FLUID_STR_TYPE as i32 {
            return 0 as i32;
        }
        setting = value as *mut StrSetting;
        if !(*setting).value.is_null() {
            libc::free((*setting).value as *mut libc::c_void);
        }
        (*setting).value = if !str.is_null() {
            libc::strcpy(
                libc::malloc(libc::strlen(str).wrapping_add(1)) as *mut libc::c_char,
                str,
            )
        } else {
            0 as *mut libc::c_char
        };
        if (*setting).update.is_some() {
            Some((*setting).update.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*setting).data, name, (*setting).value
            );
        }
        return 1 as i32;
    } else {
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
            setting_0 as *mut libc::c_void,
            FLUID_STR_TYPE as i32,
        );
    };
}

pub unsafe fn fluid_settings_getstr(
    settings: *mut Settings,
    name: *const libc::c_char,
    str: *mut *mut libc::c_char,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_STR_TYPE as i32
    {
        let setting: *mut StrSetting = value as *mut StrSetting;
        *str = (*setting).value;
        return 1 as i32;
    }
    *str = 0 as *mut libc::c_char;
    return 0 as i32;
}

pub unsafe fn fluid_settings_str_equal(
    settings: *mut Settings,
    name: *const libc::c_char,
    s: *mut libc::c_char,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_STR_TYPE as i32
    {
        let setting: *mut StrSetting = value as *mut StrSetting;
        return (libc::strcmp((*setting).value, s) == 0 as i32) as i32;
    }
    return 0 as i32;
}

pub unsafe fn fluid_settings_getstr_default(
    settings: *mut Settings,
    name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_STR_TYPE as i32
    {
        let setting: *mut StrSetting = value as *mut StrSetting;
        return (*setting).def;
    } else {
        return 0 as *mut libc::c_char;
    };
}

pub unsafe fn fluid_settings_setnum(
    settings: *mut Settings,
    name: *const libc::c_char,
    mut val: f64,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut setting;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 != FLUID_NUM_TYPE as i32 {
            return 0 as i32;
        }
        setting = value as *mut NumSetting;
        if val < (*setting).min {
            val = (*setting).min
        } else if val > (*setting).max {
            val = (*setting).max
        }
        (*setting).value = val;
        if (*setting).update.is_some() {
            Some((*setting).update.expect("non-null function pointer"))
                .expect("non-null function pointer")((*setting).data, name, val);
        }
        return 1 as i32;
    } else {
        let mut setting_0;
        setting_0 = new_fluid_num_setting(
            -1e10f64,
            1e10f64,
            0.0f32 as f64,
            0 as i32,
            None,
            0 as *mut libc::c_void,
        );
        (*setting_0).value = val;
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting_0 as *mut libc::c_void,
            FLUID_NUM_TYPE as i32,
        );
    };
}

pub unsafe fn fluid_settings_getnum(
    settings: *mut Settings,
    name: *const libc::c_char,
    val: *mut f64,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_NUM_TYPE as i32
    {
        let setting: *mut NumSetting = value as *mut NumSetting;
        *val = (*setting).value;
        return 1 as i32;
    }
    return 0 as i32;
}

pub unsafe fn fluid_settings_getnum_range(
    settings: *mut Settings,
    name: *const libc::c_char,
    min: *mut f64,
    max: *mut f64,
) {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_NUM_TYPE as i32
    {
        let setting: *mut NumSetting = value as *mut NumSetting;
        *min = (*setting).min;
        *max = (*setting).max
    };
}

pub unsafe fn fluid_settings_getnum_default(
    settings: *mut Settings,
    name: *const libc::c_char,
) -> f64 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_NUM_TYPE as i32
    {
        let setting: *mut NumSetting = value as *mut NumSetting;
        return (*setting).def;
    } else {
        return 0.0f32 as f64;
    };
}

pub unsafe fn fluid_settings_setint(
    settings: *mut Settings,
    name: *const libc::c_char,
    mut val: i32,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut setting;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 != FLUID_INT_TYPE as i32 {
            return 0 as i32;
        }
        setting = value as *mut IntSetting;
        if val < (*setting).min {
            val = (*setting).min
        } else if val > (*setting).max {
            val = (*setting).max
        }
        (*setting).value = val;
        if (*setting).update.is_some() {
            Some((*setting).update.expect("non-null function pointer"))
                .expect("non-null function pointer")((*setting).data, name, val);
        }
        return 1 as i32;
    } else {
        let mut setting_0;
        setting_0 = new_fluid_int_setting(
            -(2147483647 as i32) - 1 as i32,
            2147483647 as i32,
            0 as i32,
            0 as i32,
            None,
            0 as *mut libc::c_void,
        );
        (*setting_0).value = val;
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting_0 as *mut libc::c_void,
            FLUID_INT_TYPE as i32,
        );
    };
}

pub unsafe fn fluid_settings_getint(
    settings: *mut Settings,
    name: *const libc::c_char,
    val: *mut i32,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_INT_TYPE as i32
    {
        let setting: *mut IntSetting = value as *mut IntSetting;
        *val = (*setting).value;
        return 1 as i32;
    }
    return 0 as i32;
}

pub unsafe fn fluid_settings_getint_range(
    settings: *mut Settings,
    name: *const libc::c_char,
    min: *mut i32,
    max: *mut i32,
) {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_INT_TYPE as i32
    {
        let setting: *mut IntSetting = value as *mut IntSetting;
        *min = (*setting).min;
        *max = (*setting).max
    };
}

pub unsafe fn fluid_settings_getint_default(
    settings: *mut Settings,
    name: *const libc::c_char,
) -> i32 {
    let mut type_0: i32 = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let ntokens;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_INT_TYPE as i32
    {
        let setting: *mut IntSetting = value as *mut IntSetting;
        return (*setting).def;
    } else {
        return 0.0f32 as i32;
    };
}
