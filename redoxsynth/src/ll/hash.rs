#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashTable {
    pub size: libc::c_uint,
    pub nnodes: libc::c_uint,
    pub nodes: *mut *mut HashNode,
    pub del: DeleteFn,
}
pub type DeleteFn =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashNode {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_void,
    pub type_0: libc::c_int,
    pub next: *mut HashNode,
}

pub type IterFn = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_char,
        _: *mut libc::c_void,
        _: libc::c_int,
        _: *mut libc::c_void,
    ) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn new_fluid_hashtable(
    del: DeleteFn,
) -> *mut HashTable {
    let mut hash_table: *mut HashTable;
    let mut i: libc::c_uint;
    hash_table = libc::malloc(::std::mem::size_of::<HashTable>() as libc::size_t)
        as *mut HashTable;
    (*hash_table).size = 7 as libc::c_int as libc::c_uint;
    (*hash_table).nnodes = 0 as libc::c_int as libc::c_uint;
    (*hash_table).nodes = libc::malloc(
        ((*hash_table).size as libc::size_t)
            .wrapping_mul(::std::mem::size_of::<*mut HashNode>() as libc::size_t),
    ) as *mut *mut HashNode;
    (*hash_table).del = del;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*hash_table).size {
        let ref mut fresh0 = *(*hash_table).nodes.offset(i as isize);
        *fresh0 = 0 as *mut HashNode;
        i = i.wrapping_add(1)
    }
    return hash_table;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_hashtable(hash_table: *mut HashTable) {
    let mut i: libc::c_uint;
    if hash_table.is_null() {
        return;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*hash_table).size {
        delete_fluid_hashnodes(*(*hash_table).nodes.offset(i as isize), (*hash_table).del);
        i = i.wrapping_add(1)
    }
    libc::free((*hash_table).nodes as *mut libc::c_void);
    libc::free(hash_table as *mut libc::c_void);
}
unsafe extern "C" fn fluid_hashtable_lookup_node(
    hash_table: *mut HashTable,
    key: *mut libc::c_char,
) -> *mut *mut HashNode {
    let mut node: *mut *mut HashNode;
    node = &mut *(*hash_table).nodes.offset(
        (fluid_str_hash as unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_uint)(key)
            .wrapping_rem((*hash_table).size) as isize,
    ) as *mut *mut HashNode;
    while !(*node).is_null() && libc::strcmp((**node).key, key) != 0 as libc::c_int {
        node = &mut (**node).next
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_hashtable_lookup(
    hash_table: *mut HashTable,
    key: *mut libc::c_char,
    value: *mut *mut libc::c_void,
    type_0: *mut libc::c_int,
) -> libc::c_int {
    let node: *mut HashNode;
    node = *fluid_hashtable_lookup_node(hash_table, key);
    if !node.is_null() {
        if !value.is_null() {
            *value = (*node).value
        }
        if !type_0.is_null() {
            *type_0 = (*node).type_0
        }
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_hashtable_insert(
    mut hash_table: *mut HashTable,
    key: *mut libc::c_char,
    value: *mut libc::c_void,
    type_0: libc::c_int,
) {
    let mut node: *mut *mut HashNode;
    node = fluid_hashtable_lookup_node(hash_table, key);
    if !(*node).is_null() {
        (**node).value = value;
        (**node).type_0 = type_0
    } else {
        *node = new_fluid_hashnode(key, value, type_0);
        (*hash_table).nnodes = (*hash_table).nnodes.wrapping_add(1);
        if (3 as libc::c_int as libc::c_uint).wrapping_mul((*hash_table).size)
            <= (*hash_table).nnodes
            && (*hash_table).size < 13845163 as libc::c_int as libc::c_uint
        {
            fluid_hashtable_resize(hash_table);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_hashtable_replace(
    mut hash_table: *mut HashTable,
    key: *mut libc::c_char,
    value: *mut libc::c_void,
    type_0: libc::c_int,
) {
    let mut node: *mut *mut HashNode;
    node = fluid_hashtable_lookup_node(hash_table, key);
    if !(*node).is_null() {
        if (*hash_table).del.is_some() {
            (*hash_table).del.expect("non-null function pointer")((**node).value, (**node).type_0);
        }
        (**node).value = value
    } else {
        *node = new_fluid_hashnode(key, value, type_0);
        (*hash_table).nnodes = (*hash_table).nnodes.wrapping_add(1);
        if (3 as libc::c_int as libc::c_uint).wrapping_mul((*hash_table).size)
            <= (*hash_table).nnodes
            && (*hash_table).size < 13845163 as libc::c_int as libc::c_uint
        {
            fluid_hashtable_resize(hash_table);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_hashtable_remove(
    mut hash_table: *mut HashTable,
    key: *mut libc::c_char,
) -> libc::c_int {
    let node: *mut *mut HashNode;
    let dest: *mut HashNode;
    node = fluid_hashtable_lookup_node(hash_table, key);
    if !(*node).is_null() {
        dest = *node;
        *node = (*dest).next;
        delete_fluid_hashnode(dest, (*hash_table).del);
        (*hash_table).nnodes = (*hash_table).nnodes.wrapping_sub(1);
        if (3 as libc::c_int as libc::c_uint).wrapping_mul((*hash_table).size)
            <= (*hash_table).nnodes
            && (*hash_table).size < 13845163 as libc::c_int as libc::c_uint
        {
            fluid_hashtable_resize(hash_table);
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_hashtable_foreach(
    hash_table: *mut HashTable,
    func: IterFn,
    data: *mut libc::c_void,
) {
    let mut node: *mut HashNode;
    let mut i: libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*hash_table).size {
        node = *(*hash_table).nodes.offset(i as isize);
        while !node.is_null() {
            Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
                (*node).key,
                (*node).value,
                (*node).type_0,
                data,
            );
            node = (*node).next
        }
        i = i.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_hashtable_size(
    hash_table: *mut HashTable,
) -> libc::c_uint {
    return (*hash_table).nnodes;
}
unsafe extern "C" fn fluid_hashtable_resize(mut hash_table: *mut HashTable) {
    let new_nodes: *mut *mut HashNode;
    let mut node: *mut HashNode;
    let mut next: *mut HashNode;
    let mut hash_val: libc::c_uint;
    let mut new_size: libc::c_int;
    let mut i: libc::c_uint;
    new_size = (3 as libc::c_int as libc::c_uint)
        .wrapping_mul((*hash_table).size)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    new_size = if new_size > 13845163 as libc::c_int {
        13845163 as libc::c_int
    } else {
        new_size
    };
    new_nodes = libc::malloc(
        (new_size as libc::size_t)
            .wrapping_mul(::std::mem::size_of::<*mut HashNode>() as libc::size_t),
    ) as *mut *mut HashNode;
    libc::memset(
        new_nodes as *mut libc::c_void,
        0 as libc::c_int,
        (new_size as libc::size_t)
            .wrapping_mul(::std::mem::size_of::<*mut HashNode>() as libc::size_t),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*hash_table).size {
        node = *(*hash_table).nodes.offset(i as isize);
        while !node.is_null() {
            next = (*node).next;
            hash_val = fluid_str_hash((*node).key).wrapping_rem(new_size as libc::c_uint);
            (*node).next = *new_nodes.offset(hash_val as isize);
            let ref mut fresh1 = *new_nodes.offset(hash_val as isize);
            *fresh1 = node;
            node = next
        }
        i = i.wrapping_add(1)
    }
    libc::free((*hash_table).nodes as *mut libc::c_void);
    (*hash_table).nodes = new_nodes;
    (*hash_table).size = new_size as libc::c_uint;
}
unsafe extern "C" fn new_fluid_hashnode(
    key: *mut libc::c_char,
    value: *mut libc::c_void,
    type_0: libc::c_int,
) -> *mut HashNode {
    let mut hash_node: *mut HashNode;
    hash_node = libc::malloc(::std::mem::size_of::<HashNode>() as libc::size_t)
        as *mut HashNode;
    (*hash_node).key = libc::strcpy(
        libc::malloc(libc::strlen(key).wrapping_add(1 as libc::c_int as libc::size_t))
            as *mut libc::c_char,
        key,
    );
    (*hash_node).value = value;
    (*hash_node).type_0 = type_0;
    (*hash_node).next = 0 as *mut HashNode;
    return hash_node;
}
unsafe extern "C" fn delete_fluid_hashnode(
    hash_node: *mut HashNode,
    del: DeleteFn,
) {
    if del.is_some() {
        Some(del.expect("non-null function pointer")).expect("non-null function pointer")(
            (*hash_node).value,
            (*hash_node).type_0,
        );
    }
    if !(*hash_node).key.is_null() {
        libc::free((*hash_node).key as *mut libc::c_void);
    }
    libc::free(hash_node as *mut libc::c_void);
}
unsafe extern "C" fn delete_fluid_hashnodes(
    mut hash_node: *mut HashNode,
    del: DeleteFn,
) {
    while !hash_node.is_null() {
        let next: *mut HashNode = (*hash_node).next;
        delete_fluid_hashnode(hash_node, del);
        hash_node = next
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_str_hash(key: *mut libc::c_char) -> libc::c_uint {
    let mut p: *mut libc::c_char = key;
    let mut h: libc::c_uint = *p as libc::c_uint;
    if h != 0 {
        p = p.offset(1 as libc::c_int as isize);
        while *p as libc::c_int != '\u{0}' as i32 {
            h = (h << 5 as libc::c_int)
                .wrapping_sub(h)
                .wrapping_add(*p as libc::c_uint);
            p = p.offset(1)
        }
    }
    return h;
}