#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_list_t {
    pub data: *mut libc::c_void,
    pub next: *mut fluid_list_t,
}
pub type fluid_list_t = _fluid_list_t;
pub type fluid_compare_func_t =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_int>;

#[no_mangle]
pub unsafe extern "C" fn new_fluid_list() -> *mut fluid_list_t {
    let mut list: *mut fluid_list_t = 0 as *mut fluid_list_t;
    list = libc::malloc(::std::mem::size_of::<fluid_list_t>() as libc::size_t) as *mut fluid_list_t;
    (*list).data = 0 as *mut libc::c_void;
    (*list).next = 0 as *mut fluid_list_t;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_list(mut list: *mut fluid_list_t) {
    let mut next: *mut fluid_list_t = 0 as *mut fluid_list_t;
    while !list.is_null() {
        next = (*list).next;
        libc::free(list as *mut libc::c_void);
        list = next
    }
}
#[no_mangle]
pub unsafe extern "C" fn delete1_fluid_list(mut list: *mut fluid_list_t) {
    if !list.is_null() {
        libc::free(list as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_append(
    mut list: *mut fluid_list_t,
    mut data: *mut libc::c_void,
) -> *mut fluid_list_t {
    let mut new_list: *mut fluid_list_t = 0 as *mut fluid_list_t;
    let mut last: *mut fluid_list_t = 0 as *mut fluid_list_t;
    new_list = new_fluid_list();
    (*new_list).data = data;
    if !list.is_null() {
        last = fluid_list_last(list);
        (*last).next = new_list;
        return list;
    } else {
        return new_list;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_prepend(
    mut list: *mut fluid_list_t,
    mut data: *mut libc::c_void,
) -> *mut fluid_list_t {
    let mut new_list: *mut fluid_list_t = 0 as *mut fluid_list_t;
    new_list = new_fluid_list();
    (*new_list).data = data;
    (*new_list).next = list;
    return new_list;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_nth(
    mut list: *mut fluid_list_t,
    mut n: libc::c_int,
) -> *mut fluid_list_t {
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 > 0 as libc::c_int && !list.is_null()) {
            break;
        }
        list = (*list).next
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_remove(
    mut list: *mut fluid_list_t,
    mut data: *mut libc::c_void,
) -> *mut fluid_list_t {
    let mut tmp: *mut fluid_list_t = 0 as *mut fluid_list_t;
    let mut prev: *mut fluid_list_t = 0 as *mut fluid_list_t;
    prev = 0 as *mut fluid_list_t;
    tmp = list;
    while !tmp.is_null() {
        if (*tmp).data == data {
            if !prev.is_null() {
                (*prev).next = (*tmp).next
            }
            if list == tmp {
                list = (*list).next
            }
            (*tmp).next = 0 as *mut fluid_list_t;
            delete_fluid_list(tmp);
            break;
        } else {
            prev = tmp;
            tmp = (*tmp).next
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_remove_link(
    mut list: *mut fluid_list_t,
    mut link: *mut fluid_list_t,
) -> *mut fluid_list_t {
    let mut tmp: *mut fluid_list_t = 0 as *mut fluid_list_t;
    let mut prev: *mut fluid_list_t = 0 as *mut fluid_list_t;
    prev = 0 as *mut fluid_list_t;
    tmp = list;
    while !tmp.is_null() {
        if tmp == link {
            if !prev.is_null() {
                (*prev).next = (*tmp).next
            }
            if list == tmp {
                list = (*list).next
            }
            (*tmp).next = 0 as *mut fluid_list_t;
            break;
        } else {
            prev = tmp;
            tmp = (*tmp).next
        }
    }
    return list;
}
unsafe extern "C" fn fluid_list_sort_merge(
    mut l1: *mut fluid_list_t,
    mut l2: *mut fluid_list_t,
    mut compare_func: fluid_compare_func_t,
) -> *mut fluid_list_t {
    let mut list: fluid_list_t = fluid_list_t {
        data: 0 as *mut libc::c_void,
        next: 0 as *mut fluid_list_t,
    };
    let mut l: *mut fluid_list_t = 0 as *mut fluid_list_t;
    l = &mut list;
    while !l1.is_null() && !l2.is_null() {
        if compare_func.expect("non-null function pointer")((*l1).data, (*l2).data)
            < 0 as libc::c_int
        {
            (*l).next = l1;
            l = (*l).next;
            l1 = (*l1).next
        } else {
            (*l).next = l2;
            l = (*l).next;
            l2 = (*l2).next
        }
    }
    (*l).next = if !l1.is_null() { l1 } else { l2 };
    return list.next;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_sort(
    mut list: *mut fluid_list_t,
    mut compare_func: fluid_compare_func_t,
) -> *mut fluid_list_t {
    let mut l1: *mut fluid_list_t = 0 as *mut fluid_list_t;
    let mut l2: *mut fluid_list_t = 0 as *mut fluid_list_t;
    if list.is_null() {
        return 0 as *mut fluid_list_t;
    }
    if (*list).next.is_null() {
        return list;
    }
    l1 = list;
    l2 = (*list).next;
    loop {
        l2 = (*l2).next;
        if l2.is_null() {
            break;
        }
        l2 = (*l2).next;
        if l2.is_null() {
            break;
        }
        l1 = (*l1).next
    }
    l2 = (*l1).next;
    (*l1).next = 0 as *mut fluid_list_t;
    return fluid_list_sort_merge(
        fluid_list_sort(list, compare_func),
        fluid_list_sort(l2, compare_func),
        compare_func,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_last(mut list: *mut fluid_list_t) -> *mut fluid_list_t {
    if !list.is_null() {
        while !(*list).next.is_null() {
            list = (*list).next
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_size(mut list: *mut fluid_list_t) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    while !list.is_null() {
        n += 1;
        list = (*list).next
    }
    return n;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_list_insert_at(
    mut list: *mut fluid_list_t,
    mut n: libc::c_int,
    mut data: *mut libc::c_void,
) -> *mut fluid_list_t {
    let mut new_list: *mut fluid_list_t = 0 as *mut fluid_list_t;
    let mut cur: *mut fluid_list_t = 0 as *mut fluid_list_t;
    let mut prev: *mut fluid_list_t = 0 as *mut fluid_list_t;
    new_list = new_fluid_list();
    (*new_list).data = data;
    cur = list;
    loop {
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 > 0 as libc::c_int && !cur.is_null()) {
            break;
        }
        prev = cur;
        cur = (*cur).next
    }
    (*new_list).next = cur;
    if !prev.is_null() {
        (*prev).next = new_list;
        return list;
    } else {
        return new_list;
    };
}
