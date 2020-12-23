#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::hash::delete_fluid_hashtable;
use crate::hash::fluid_hashtable_insert;
use crate::hash::fluid_hashtable_lookup;
use crate::hash::fluid_hashtable_replace;
use crate::hash::fluid_hashtable_t;
use crate::hash::new_fluid_hashtable;
use crate::list::delete_fluid_list;
use crate::list::fluid_list_append;
use crate::list::fluid_list_remove_link;
use crate::list::fluid_list_t;
use crate::synth::fluid_synth_settings;
use crate::sys::fluid_strtok;
use std::ffi::CStr;
pub type fluid_settings_t = fluid_hashtable_t;
pub type fluid_types_enum = libc::c_int;
pub const FLUID_SET_TYPE: fluid_types_enum = 3;
pub const FLUID_STR_TYPE: fluid_types_enum = 2;
pub const FLUID_INT_TYPE: fluid_types_enum = 1;
pub const FLUID_NUM_TYPE: fluid_types_enum = 0;
pub const FLUID_NO_TYPE: fluid_types_enum = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_str_setting_t {
    pub value: *mut libc::c_char,
    pub def: *mut libc::c_char,
    pub hints: libc::c_int,
    pub options: *mut fluid_list_t,
    pub update: fluid_str_update_t,
    pub data: *mut libc::c_void,
}
pub type fluid_str_update_t = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: *mut libc::c_char,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_int_setting_t {
    pub value: libc::c_int,
    pub def: libc::c_int,
    pub min: libc::c_int,
    pub max: libc::c_int,
    pub hints: libc::c_int,
    pub update: fluid_int_update_t,
    pub data: *mut libc::c_void,
}
pub type fluid_int_update_t = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_num_setting_t {
    pub value: libc::c_double,
    pub def: libc::c_double,
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub hints: libc::c_int,
    pub update: fluid_num_update_t,
    pub data: *mut libc::c_void,
}
pub type fluid_num_update_t = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: libc::c_double,
    ) -> libc::c_int,
>;
pub type fluid_hash_delete_t =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ()>;
pub const FLUID_ERR: fluid_log_level = 1;
pub const FLUID_WARN: fluid_log_level = 2;
pub type fluid_log_level = libc::c_uint;
pub const LAST_LOG_LEVEL: fluid_log_level = 5;
pub const FLUID_DBG: fluid_log_level = 4;
pub const FLUID_INFO: fluid_log_level = 3;
pub const FLUID_PANIC: fluid_log_level = 0;
unsafe extern "C" fn new_fluid_str_setting(
    mut value: *const libc::c_char,
    mut def: *mut libc::c_char,
    mut hints: libc::c_int,
    mut fun: fluid_str_update_t,
    mut data: *mut libc::c_void,
) -> *mut fluid_str_setting_t {
    let mut str: *mut fluid_str_setting_t = 0 as *mut fluid_str_setting_t;
    str = libc::malloc(::std::mem::size_of::<fluid_str_setting_t>() as libc::size_t)
        as *mut fluid_str_setting_t;
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
    (*str).options = 0 as *mut fluid_list_t;
    (*str).update = fun;
    (*str).data = data;
    return str;
}
unsafe extern "C" fn delete_fluid_str_setting(mut str: *mut fluid_str_setting_t) {
    if !str.is_null() {
        if !(*str).value.is_null() {
            libc::free((*str).value as *mut libc::c_void);
        }
        if !(*str).def.is_null() {
            libc::free((*str).def as *mut libc::c_void);
        }
        if !(*str).options.is_null() {
            let mut list: *mut fluid_list_t = (*str).options;
            while !list.is_null() {
                libc::free((*list).data);
                list = if !list.is_null() {
                    (*list).next
                } else {
                    0 as *mut fluid_list_t
                }
            }
            delete_fluid_list((*str).options);
        }
        libc::free(str as *mut libc::c_void);
    };
}
unsafe extern "C" fn new_fluid_num_setting(
    mut min: libc::c_double,
    mut max: libc::c_double,
    mut def: libc::c_double,
    mut hints: libc::c_int,
    mut fun: fluid_num_update_t,
    mut data: *mut libc::c_void,
) -> *mut fluid_num_setting_t {
    let mut setting: *mut fluid_num_setting_t = 0 as *mut fluid_num_setting_t;
    setting = libc::malloc(::std::mem::size_of::<fluid_num_setting_t>() as libc::size_t)
        as *mut fluid_num_setting_t;
    (*setting).value = def;
    (*setting).def = def;
    (*setting).min = min;
    (*setting).max = max;
    (*setting).hints = hints;
    (*setting).update = fun;
    (*setting).data = data;
    return setting;
}
unsafe extern "C" fn delete_fluid_num_setting(mut setting: *mut fluid_num_setting_t) {
    if !setting.is_null() {
        libc::free(setting as *mut libc::c_void);
    };
}
unsafe extern "C" fn new_fluid_int_setting(
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut def: libc::c_int,
    mut hints: libc::c_int,
    mut fun: fluid_int_update_t,
    mut data: *mut libc::c_void,
) -> *mut fluid_int_setting_t {
    let mut setting: *mut fluid_int_setting_t = 0 as *mut fluid_int_setting_t;
    setting = libc::malloc(::std::mem::size_of::<fluid_int_setting_t>() as libc::size_t)
        as *mut fluid_int_setting_t;
    (*setting).value = def;
    (*setting).def = def;
    (*setting).min = min;
    (*setting).max = max;
    (*setting).hints = hints;
    (*setting).update = fun;
    (*setting).data = data;
    return setting;
}
unsafe extern "C" fn delete_fluid_int_setting(mut setting: *mut fluid_int_setting_t) {
    if !setting.is_null() {
        libc::free(setting as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_settings() -> *mut fluid_settings_t {
    let mut settings: *mut fluid_settings_t = new_fluid_hashtable(Some(
        fluid_settings_hash_delete
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    ));
    if settings.is_null() {
        return 0 as *mut fluid_settings_t;
    }
    fluid_settings_init(settings);
    return settings;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_settings(mut settings: *mut fluid_settings_t) {
    delete_fluid_hashtable(settings);
}
unsafe extern "C" fn fluid_settings_hash_delete(
    mut value: *mut libc::c_void,
    mut type_0: libc::c_int,
) {
    match type_0 {
        0 => {
            delete_fluid_num_setting(value as *mut fluid_num_setting_t);
        }
        1 => {
            delete_fluid_int_setting(value as *mut fluid_int_setting_t);
        }
        2 => {
            delete_fluid_str_setting(value as *mut fluid_str_setting_t);
        }
        3 => {
            delete_fluid_hashtable(value as *mut fluid_hashtable_t);
        }
        _ => {}
    };
}
unsafe extern "C" fn fluid_settings_init(mut settings: *mut fluid_settings_t) {
    fluid_synth_settings(settings);
}
unsafe extern "C" fn fluid_settings_tokenize(
    mut s: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut ptr: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tokstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0 as libc::c_int;
    if libc::strlen(s) > 256 {
        fluid_log!(
            FLUID_ERR,
            "Setting variable name exceeded max length of {} chars",
            256
        );
        return 0 as libc::c_int;
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
        if n > 8 as libc::c_int {
            fluid_log!(
                FLUID_ERR,
                "Setting variable name exceeded max token count of {}",
                8
            );
            return 0 as libc::c_int;
        }
        let fresh0 = n;
        n = n + 1;
        let ref mut fresh1 = *ptr.offset(fresh0 as isize);
        *fresh1 = tok
    }
    return n;
}
unsafe extern "C" fn fluid_settings_get(
    mut settings: *mut fluid_settings_t,
    mut name: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut value: *mut *mut libc::c_void,
    mut type_0: *mut libc::c_int,
) -> libc::c_int {
    let mut table: *mut fluid_hashtable_t = settings;
    let mut t: libc::c_int = 0;
    let mut v: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < len {
        if table.is_null() {
            return 0 as libc::c_int;
        }
        if fluid_hashtable_lookup(table, *name.offset(n as isize), &mut v, &mut t) == 0 {
            return 0 as libc::c_int;
        }
        table = if t == FLUID_SET_TYPE as libc::c_int {
            v as *mut fluid_hashtable_t
        } else {
            0 as *mut fluid_hashtable_t
        };
        n += 1
    }
    if !value.is_null() {
        *value = v
    }
    if !type_0.is_null() {
        *type_0 = t
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fluid_settings_set(
    mut settings: *mut fluid_settings_t,
    mut name: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut value: *mut libc::c_void,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut table: *mut fluid_hashtable_t = settings;
    let mut t: libc::c_int = 0;
    let mut v: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n: libc::c_int = 0;
    let mut num: libc::c_int = len - 1 as libc::c_int;
    n = 0 as libc::c_int;
    while n < num {
        if fluid_hashtable_lookup(table, *name.offset(n as isize), &mut v, &mut t) != 0 {
            if t == FLUID_SET_TYPE as libc::c_int {
                table = v as *mut fluid_hashtable_t
            } else {
                fluid_log!(
                    FLUID_WARN,
                    "\'{}\' is not a node",
                    CStr::from_ptr(*name.offset(n as isize)).to_str().unwrap()
                );
                return 0 as libc::c_int;
            }
        } else {
            let mut tmp: *mut fluid_hashtable_t = 0 as *mut fluid_hashtable_t;
            tmp = new_fluid_hashtable(Some(
                fluid_settings_hash_delete
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
            ));
            fluid_hashtable_insert(
                table,
                *name.offset(n as isize),
                tmp as *mut libc::c_void,
                FLUID_SET_TYPE as libc::c_int,
            );
            table = tmp
        }
        n += 1
    }
    fluid_hashtable_replace(table, *name.offset(num as isize), value, type_0);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_register_str(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut def: *mut libc::c_char,
    mut hints: libc::c_int,
    mut fun: fluid_str_update_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    let mut setting: *mut fluid_str_setting_t = 0 as *mut fluid_str_setting_t;
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
            FLUID_STR_TYPE as libc::c_int,
        );
    } else if type_0 == FLUID_STR_TYPE as libc::c_int {
        setting = value as *mut fluid_str_setting_t;
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
        return 1 as libc::c_int;
    } else {
        fluid_log!(
            FLUID_WARN,
            "Type mismatch on setting \'{}\'",
            CStr::from_ptr(name).to_str().unwrap()
        );
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_register_num(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut def: libc::c_double,
    mut min: libc::c_double,
    mut max: libc::c_double,
    mut hints: libc::c_int,
    mut fun: fluid_num_update_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) == 0
    {
        let mut setting: *mut fluid_num_setting_t = 0 as *mut fluid_num_setting_t;
        setting = new_fluid_num_setting(min, max, def, hints, fun, data);
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting as *mut libc::c_void,
            FLUID_NUM_TYPE as libc::c_int,
        );
    } else if type_0 == FLUID_NUM_TYPE as libc::c_int {
        let mut setting_0: *mut fluid_num_setting_t = value as *mut fluid_num_setting_t;
        (*setting_0).update = fun;
        (*setting_0).data = data;
        (*setting_0).min = min;
        (*setting_0).max = max;
        (*setting_0).def = def;
        (*setting_0).hints = hints;
        return 1 as libc::c_int;
    } else {
        fluid_log!(
            FLUID_WARN,
            "Type mismatch on setting \'{}\'",
            CStr::from_ptr(name).to_str().unwrap()
        );
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_register_int(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut def: libc::c_int,
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut hints: libc::c_int,
    mut fun: fluid_int_update_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) == 0
    {
        let mut setting: *mut fluid_int_setting_t = 0 as *mut fluid_int_setting_t;
        setting = new_fluid_int_setting(min, max, def, hints, fun, data);
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting as *mut libc::c_void,
            FLUID_INT_TYPE as libc::c_int,
        );
    } else if type_0 == FLUID_INT_TYPE as libc::c_int {
        let mut setting_0: *mut fluid_int_setting_t = value as *mut fluid_int_setting_t;
        (*setting_0).update = fun;
        (*setting_0).data = data;
        (*setting_0).min = min;
        (*setting_0).max = max;
        (*setting_0).def = def;
        (*setting_0).hints = hints;
        return 1 as libc::c_int;
    } else {
        fluid_log!(
            FLUID_WARN,
            "Type mismatch on setting \'{}\'",
            CStr::from_ptr(name).to_str().unwrap()
        );
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_get_type(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
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
        FLUID_NO_TYPE as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_get_hints(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 == FLUID_NUM_TYPE as libc::c_int {
            let mut setting: *mut fluid_num_setting_t = value as *mut fluid_num_setting_t;
            return (*setting).hints;
        } else if type_0 == FLUID_STR_TYPE as libc::c_int {
            let mut setting_0: *mut fluid_str_setting_t = value as *mut fluid_str_setting_t;
            return (*setting_0).hints;
        } else {
            return 0 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_is_realtime(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 == FLUID_NUM_TYPE as libc::c_int {
            let mut setting: *mut fluid_num_setting_t = value as *mut fluid_num_setting_t;
            return (*setting).update.is_some() as libc::c_int;
        } else if type_0 == FLUID_STR_TYPE as libc::c_int {
            let mut setting_0: *mut fluid_str_setting_t = value as *mut fluid_str_setting_t;
            return (*setting_0).update.is_some() as libc::c_int;
        } else {
            return 0 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_setstr(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut setting: *mut fluid_str_setting_t = 0 as *mut fluid_str_setting_t;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 != FLUID_STR_TYPE as libc::c_int {
            return 0 as libc::c_int;
        }
        setting = value as *mut fluid_str_setting_t;
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
        return 1 as libc::c_int;
    } else {
        let mut setting_0: *mut fluid_str_setting_t = 0 as *mut fluid_str_setting_t;
        setting_0 = new_fluid_str_setting(
            str,
            0 as *mut libc::c_char,
            0 as libc::c_int,
            None,
            0 as *mut libc::c_void,
        );
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting_0 as *mut libc::c_void,
            FLUID_STR_TYPE as libc::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_getstr(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut str: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_STR_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_str_setting_t = value as *mut fluid_str_setting_t;
        *str = (*setting).value;
        return 1 as libc::c_int;
    }
    *str = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_str_equal(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_STR_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_str_setting_t = value as *mut fluid_str_setting_t;
        return (libc::strcmp((*setting).value, s) == 0 as libc::c_int) as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_getstr_default(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_STR_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_str_setting_t = value as *mut fluid_str_setting_t;
        return (*setting).def;
    } else {
        return 0 as *mut libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_add_option(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_STR_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_str_setting_t = value as *mut fluid_str_setting_t;
        let mut copy: *mut libc::c_char = libc::strcpy(
            libc::malloc(libc::strlen(s).wrapping_add(1)) as *mut libc::c_char,
            s,
        );
        (*setting).options = fluid_list_append((*setting).options, copy as *mut libc::c_void);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_remove_option(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_STR_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_str_setting_t = value as *mut fluid_str_setting_t;
        let mut list: *mut fluid_list_t = (*setting).options;
        while !list.is_null() {
            let mut option: *mut libc::c_char = if !list.is_null() {
                (*list).data
            } else {
                0 as *mut libc::c_void
            } as *mut libc::c_char;
            if libc::strcmp(s, option) == 0 as libc::c_int {
                libc::free(option as *mut libc::c_void);
                (*setting).options = fluid_list_remove_link((*setting).options, list);
                return 1 as libc::c_int;
            }
            list = if !list.is_null() {
                (*list).next
            } else {
                0 as *mut fluid_list_t
            }
        }
        return 0 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_setnum(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut val: libc::c_double,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut setting: *mut fluid_num_setting_t = 0 as *mut fluid_num_setting_t;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 != FLUID_NUM_TYPE as libc::c_int {
            return 0 as libc::c_int;
        }
        setting = value as *mut fluid_num_setting_t;
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
        return 1 as libc::c_int;
    } else {
        let mut setting_0: *mut fluid_num_setting_t = 0 as *mut fluid_num_setting_t;
        setting_0 = new_fluid_num_setting(
            -1e10f64,
            1e10f64,
            0.0f32 as libc::c_double,
            0 as libc::c_int,
            None,
            0 as *mut libc::c_void,
        );
        (*setting_0).value = val;
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting_0 as *mut libc::c_void,
            FLUID_NUM_TYPE as libc::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_getnum(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_NUM_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_num_setting_t = value as *mut fluid_num_setting_t;
        *val = (*setting).value;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_getnum_range(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut min: *mut libc::c_double,
    mut max: *mut libc::c_double,
) {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_NUM_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_num_setting_t = value as *mut fluid_num_setting_t;
        *min = (*setting).min;
        *max = (*setting).max
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_getnum_default(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
) -> libc::c_double {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_NUM_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_num_setting_t = value as *mut fluid_num_setting_t;
        return (*setting).def;
    } else {
        return 0.0f32 as libc::c_double;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_setint(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut val: libc::c_int,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut setting: *mut fluid_int_setting_t = 0 as *mut fluid_int_setting_t;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
    {
        if type_0 != FLUID_INT_TYPE as libc::c_int {
            return 0 as libc::c_int;
        }
        setting = value as *mut fluid_int_setting_t;
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
        return 1 as libc::c_int;
    } else {
        let mut setting_0: *mut fluid_int_setting_t = 0 as *mut fluid_int_setting_t;
        setting_0 = new_fluid_int_setting(
            -(2147483647 as libc::c_int) - 1 as libc::c_int,
            2147483647 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            None,
            0 as *mut libc::c_void,
        );
        (*setting_0).value = val;
        return fluid_settings_set(
            settings,
            tokens.as_mut_ptr(),
            ntokens,
            setting_0 as *mut libc::c_void,
            FLUID_INT_TYPE as libc::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_getint(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut val: *mut libc::c_int,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_INT_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_int_setting_t = value as *mut fluid_int_setting_t;
        *val = (*setting).value;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_getint_range(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
    mut min: *mut libc::c_int,
    mut max: *mut libc::c_int,
) {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_INT_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_int_setting_t = value as *mut fluid_int_setting_t;
        *min = (*setting).min;
        *max = (*setting).max
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_settings_getint_default(
    mut settings: *mut fluid_settings_t,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tokens: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    let mut buf: [libc::c_char; 257] = [0; 257];
    let mut ntokens: libc::c_int = 0;
    ntokens = fluid_settings_tokenize(name, buf.as_mut_ptr(), tokens.as_mut_ptr());
    if fluid_settings_get(
        settings,
        tokens.as_mut_ptr(),
        ntokens,
        &mut value,
        &mut type_0,
    ) != 0
        && type_0 == FLUID_INT_TYPE as libc::c_int
    {
        let mut setting: *mut fluid_int_setting_t = value as *mut fluid_int_setting_t;
        return (*setting).def;
    } else {
        return 0.0f32 as libc::c_int;
    };
}
