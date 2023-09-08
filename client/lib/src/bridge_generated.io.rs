use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_get_config_path(port_: i64) {
    wire_get_config_path_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_init_runtime(
    port_: i64,
    config_path: *mut wire_uint_8_list,
    work_thread: usize,
    log_level: *mut wire_uint_8_list,
) {
    wire_init_runtime_impl(port_, config_path, work_thread, log_level)
}

#[no_mangle]
pub extern "C" fn wire_join_network(port_: i64, invite_code: *mut wire_uint_8_list) {
    wire_join_network_impl(port_, invite_code)
}

#[no_mangle]
pub extern "C" fn wire_list_network(port_: i64) {
    wire_list_network_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_start(port_: i64, network_id: *mut wire_uint_8_list, raw_fd: i32) {
    wire_start_impl(port_, network_id, raw_fd)
}

#[no_mangle]
pub extern "C" fn wire_version(port_: i64) {
    wire_version_impl(port_)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
