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
pub extern "C" fn wire_version(port_: i64) {
    wire_version_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_param(port_: i64, client_message: *mut wire_ClientMessage) {
    wire_test_param_impl(port_, client_message)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_client_info_0() -> *mut wire_ClientInfo {
    support::new_leak_box_ptr(wire_ClientInfo::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_client_message_0() -> *mut wire_ClientMessage {
    support::new_leak_box_ptr(wire_ClientMessage::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_interface_0() -> *mut wire_Interface {
    support::new_leak_box_ptr(wire_Interface::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u32_0(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_wr_config_0() -> *mut wire_WrConfig {
    support::new_leak_box_ptr(wire_WrConfig::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_peer_0(len: i32) -> *mut wire_list_peer {
    let wrap = wire_list_peer {
        ptr: support::new_leak_vec_ptr(<wire_Peer>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

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
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<ClientInfo> for *mut wire_ClientInfo {
    fn wire2api(self) -> ClientInfo {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ClientInfo>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ClientMessage> for *mut wire_ClientMessage {
    fn wire2api(self) -> ClientMessage {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ClientMessage>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Interface> for *mut wire_Interface {
    fn wire2api(self) -> Interface {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Interface>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<WrConfig> for *mut wire_WrConfig {
    fn wire2api(self) -> WrConfig {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<WrConfig>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ClientInfo> for wire_ClientInfo {
    fn wire2api(self) -> ClientInfo {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Config);
                ClientInfo::Config(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Status);
                ClientInfo::Status(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<ClientMessage> for wire_ClientMessage {
    fn wire2api(self) -> ClientMessage {
        ClientMessage {
            network_id: self.network_id.wire2api(),
            info: self.info.wire2api(),
        }
    }
}

impl Wire2Api<Interface> for wire_Interface {
    fn wire2api(self) -> Interface {
        Interface {
            name: self.name.wire2api(),
            address: self.address.wire2api(),
            listen_port: self.listen_port.wire2api(),
            dns: self.dns.wire2api(),
            mtu: self.mtu.wire2api(),
            pre_up: self.pre_up.wire2api(),
            post_up: self.post_up.wire2api(),
            pre_down: self.pre_down.wire2api(),
            post_down: self.post_down.wire2api(),
            protocol: self.protocol.wire2api(),
        }
    }
}
impl Wire2Api<Vec<Peer>> for *mut wire_list_peer {
    fn wire2api(self) -> Vec<Peer> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Peer> for wire_Peer {
    fn wire2api(self) -> Peer {
        Peer {
            endpoint: self.endpoint.wire2api(),
            allowed_ip: self.allowed_ip.wire2api(),
            public_key: self.public_key.wire2api(),
            persistence_keep_alive: self.persistence_keep_alive.wire2api(),
            address: self.address.wire2api(),
        }
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

impl Wire2Api<WrConfig> for wire_WrConfig {
    fn wire2api(self) -> WrConfig {
        WrConfig {
            interface: self.interface.wire2api(),
            peers: self.peers.wire2api(),
            typ: self.typ.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ClientMessage {
    network_id: *mut wire_uint_8_list,
    info: *mut wire_ClientInfo,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Interface {
    name: *mut wire_uint_8_list,
    address: *mut wire_StringList,
    listen_port: i32,
    dns: *mut wire_StringList,
    mtu: *mut u32,
    pre_up: *mut wire_uint_8_list,
    post_up: *mut wire_uint_8_list,
    pre_down: *mut wire_uint_8_list,
    post_down: *mut wire_uint_8_list,
    protocol: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_peer {
    ptr: *mut wire_Peer,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Peer {
    endpoint: *mut wire_uint_8_list,
    allowed_ip: *mut wire_StringList,
    public_key: *mut wire_uint_8_list,
    persistence_keep_alive: u32,
    address: *mut wire_StringList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_WrConfig {
    interface: *mut wire_Interface,
    peers: *mut wire_list_peer,
    typ: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ClientInfo {
    tag: i32,
    kind: *mut ClientInfoKind,
}

#[repr(C)]
pub union ClientInfoKind {
    Config: *mut wire_ClientInfo_Config,
    Status: *mut wire_ClientInfo_Status,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ClientInfo_Config {
    field0: *mut wire_WrConfig,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ClientInfo_Status {
    field0: i32,
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

impl Default for wire_ClientInfo {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_ClientInfo {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_ClientInfo_Config() -> *mut ClientInfoKind {
    support::new_leak_box_ptr(ClientInfoKind {
        Config: support::new_leak_box_ptr(wire_ClientInfo_Config {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_ClientInfo_Status() -> *mut ClientInfoKind {
    support::new_leak_box_ptr(ClientInfoKind {
        Status: support::new_leak_box_ptr(wire_ClientInfo_Status {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_ClientMessage {
    fn new_with_null_ptr() -> Self {
        Self {
            network_id: core::ptr::null_mut(),
            info: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_ClientMessage {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_Interface {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            address: core::ptr::null_mut(),
            listen_port: Default::default(),
            dns: core::ptr::null_mut(),
            mtu: core::ptr::null_mut(),
            pre_up: core::ptr::null_mut(),
            post_up: core::ptr::null_mut(),
            pre_down: core::ptr::null_mut(),
            post_down: core::ptr::null_mut(),
            protocol: Default::default(),
        }
    }
}

impl Default for wire_Interface {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_Peer {
    fn new_with_null_ptr() -> Self {
        Self {
            endpoint: core::ptr::null_mut(),
            allowed_ip: core::ptr::null_mut(),
            public_key: core::ptr::null_mut(),
            persistence_keep_alive: Default::default(),
            address: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_Peer {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_WrConfig {
    fn new_with_null_ptr() -> Self {
        Self {
            interface: core::ptr::null_mut(),
            peers: core::ptr::null_mut(),
            typ: Default::default(),
        }
    }
}

impl Default for wire_WrConfig {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
