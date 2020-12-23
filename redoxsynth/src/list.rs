#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub data: *mut libc::c_void,
    pub next: *mut List,
}
pub type CompareFn =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_int>;
#[no_mangle]
pub unsafe extern "C" fn new_fluid_list() -> *mut List {
    let mut list: *mut List;
    list = libc::malloc(::std::mem::size_of::<List>() as libc::size_t) as *mut List;
    (*list).data = 0 as *mut libc::c_void;
    (*list).next = 0 as *mut List;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_list(mut list: *mut List) {
    let mut next: *mut List;
    while !list.is_null() {
        next = (*list).next;
        libc::free(list as *mut libc::c_void);
        list = next
    }
}
#[no_mangle]
pub unsafe extern "C" fn delete1_fluid_list(list: *mut List) {
    if !list.is_null() {
        libc::free(list as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_append(
    list: *mut List,
    data: *mut libc::c_void,
) -> *mut List {
    let mut new_list: *mut List;
    let mut last: *mut List;
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
    list: *mut List,
    data: *mut libc::c_void,
) -> *mut List {
    let mut new_list: *mut List;
    new_list = new_fluid_list();
    (*new_list).data = data;
    (*new_list).next = list;
    return new_list;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_nth(
    mut list: *mut List,
    mut n: libc::c_int,
) -> *mut List {
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
    mut list: *mut List,
    data: *mut libc::c_void,
) -> *mut List {
    let mut tmp: *mut List;
    let mut prev: *mut List;
    prev = 0 as *mut List;
    tmp = list;
    while !tmp.is_null() {
        if (*tmp).data == data {
            if !prev.is_null() {
                (*prev).next = (*tmp).next
            }
            if list == tmp {
                list = (*list).next
            }
            (*tmp).next = 0 as *mut List;
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
    mut list: *mut List,
    link: *mut List,
) -> *mut List {
    let mut tmp: *mut List;
    let mut prev: *mut List;
    prev = 0 as *mut List;
    tmp = list;
    while !tmp.is_null() {
        if tmp == link {
            if !prev.is_null() {
                (*prev).next = (*tmp).next
            }
            if list == tmp {
                list = (*list).next
            }
            (*tmp).next = 0 as *mut List;
            break;
        } else {
            prev = tmp;
            tmp = (*tmp).next
        }
    }
    return list;
}
unsafe extern "C" fn fluid_list_sort_merge(
    mut l1: *mut List,
    mut l2: *mut List,
    compare_func: CompareFn,
) -> *mut List {
    let mut list: List = List {
        data: 0 as *mut libc::c_void,
        next: 0 as *mut List,
    };
    let mut l: *mut List;
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
    list: *mut List,
    compare_func: CompareFn,
) -> *mut List {
    let mut l1: *mut List;
    let mut l2: *mut List;
    if list.is_null() {
        return 0 as *mut List;
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
    (*l1).next = 0 as *mut List;
    return fluid_list_sort_merge(
        fluid_list_sort(list, compare_func),
        fluid_list_sort(l2, compare_func),
        compare_func,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_last(mut list: *mut List) -> *mut List {
    if !list.is_null() {
        while !(*list).next.is_null() {
            list = (*list).next
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_size(mut list: *mut List) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    while !list.is_null() {
        n += 1;
        list = (*list).next
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_list_insert_at(
    list: *mut List,
    mut n: libc::c_int,
    data: *mut libc::c_void,
) -> *mut List {
    let mut new_list: *mut List;
    let mut cur: *mut List;
    let mut prev: *mut List = 0 as *mut List;
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
