#[derive(Copy, Clone)]
pub struct List {
    pub(crate) data: *mut libc::c_void,
    pub(crate) next: *mut List,
}

pub unsafe fn new_fluid_list() -> *mut List {
    let mut list: *mut List;
    list = libc::malloc(::std::mem::size_of::<List>() as libc::size_t) as *mut List;
    (*list).data = 0 as *mut libc::c_void;
    (*list).next = 0 as *mut List;
    return list;
}

pub unsafe fn delete_fluid_list(mut list: *mut List) {
    let mut next: *mut List;
    while !list.is_null() {
        next = (*list).next;
        libc::free(list as *mut libc::c_void);
        list = next
    }
}

pub unsafe fn delete1_fluid_list(list: *mut List) {
    if !list.is_null() {
        libc::free(list as *mut libc::c_void);
    };
}

pub unsafe fn fluid_list_prepend(list: *mut List, data: *mut libc::c_void) -> *mut List {
    let mut new_list: *mut List;
    new_list = new_fluid_list();
    (*new_list).data = data;
    (*new_list).next = list;
    return new_list;
}

pub unsafe fn fluid_list_remove(mut list: *mut List, data: *mut libc::c_void) -> *mut List {
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

pub unsafe fn fluid_list_remove_link(mut list: *mut List, link: *mut List) -> *mut List {
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
