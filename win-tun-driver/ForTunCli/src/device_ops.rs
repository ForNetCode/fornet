use std::ffi::c_void;
use std::mem::{size_of, zeroed};
use std::net::IpAddr;
use std::process::Command;

use anyhow::{anyhow, bail, Context};
use cidr_utils::cidr::IpCidr;
use std::thread::sleep;
use std::time::Duration;
use windows::core::{wcslen, GUID, HRESULT, HSTRING, PCWSTR, PWSTR};
use windows::w;
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    CM_Get_DevNode_PropertyW, CM_Get_Device_ID_ListW, CM_Get_Device_ID_List_SizeW,
    CM_Get_Device_Interface_ListW, CM_Get_Device_Interface_List_SizeW, CM_Locate_DevNodeW,
    SetupCopyOEMInfW, SetupDiSetClassInstallParamsW, CM_GETIDLIST_FILTER_CLASS,
    CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES, CM_LOCATE_DEVINST_NORMAL, CM_LOCATE_DEVNODE_PHANTOM,
    CR_NO_SUCH_DEVNODE, CR_SUCCESS, DIF_REMOVE, DI_REMOVEDEVICE_GLOBAL, GUID_DEVCLASS_NET,
    HDEVINFO, SPOST_PATH, SP_CLASSINSTALL_HEADER, SP_COPY_NEWER, SP_DEVINFO_DATA,
    SP_REMOVEDEVICE_PARAMS,
};
use windows::Win32::Devices::Enumeration::Pnp::{
    SWDeviceCapabilitiesDriverRequired, SWDeviceCapabilitiesSilentInstall, SwDeviceClose,
    SwDeviceCreate, HSWDEVICE, SW_DEVICE_CREATE_INFO,
};
use windows::Win32::Devices::Properties::{
    DEVPKEY_Device_ClassGuid, DEVPKEY_Device_FriendlyName, DEVPKEY_Device_HardwareIds,
    DEVPROPCOMPKEY, DEVPROPERTY, DEVPROP_STORE_SYSTEM, DEVPROP_TYPE_GUID, DEVPROP_TYPE_STRING,
};
use windows::Win32::Foundation::{
    CloseHandle, GetLastError, ERROR_NO_MORE_ITEMS, HANDLE, NO_ERROR, WAIT_OBJECT_0,
};
use windows::Win32::NetworkManagement::IpHelper::GetAdapterIndex;
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_SYSTEM, FILE_FLAG_OVERLAPPED, FILE_GENERIC_READ,
    FILE_GENERIC_WRITE, FILE_SHARE_NONE, OPEN_EXISTING,
};
use windows::Win32::System::Registry::{
    RegCloseKey, RegEnumKeyExW, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_LOCAL_MACHINE,
    KEY_ENUMERATE_SUB_KEYS, KEY_READ,
};
use windows::Win32::System::IO::DeviceIoControl;

macro_rules! ctl_code {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ($DeviceType << 16) | ($Access << 14) | ($Function << 2) | $Method
    };
}

pub const FOR_TUN_IOCTL_OPEN_ADAPTER: u32 = ctl_code!(0x00000022, 0x0801, 0, 0);
pub const FOR_TUN_INTERFACE_GUID: &str = "f579d929-6c40-4e5a-8532-180199a4e321";
pub const FOR_TUN_HWID: &str = "ForTun";

// pub const FOR_TUN_IOCTL_OPEN_ADAPTER: u32 = ctl_code!(0x00000022, 6, 0, 0);
// pub const FOR_TUN_INTERFACE_GUID: &str = "CAC88484-7515-4C03-82E6-71A87ABAC361";
// pub const FOR_TUN_HWID: &str = "ovpn-dco";

//const FOR_TUN_ENUMERATOR:PCWSTR = w!("SWD\\ForTun");
pub const FOR_TUN_DEV_CLASS: GUID = GUID_DEVCLASS_NET;

pub struct AdapterDevice {
    pub handler: HSWDEVICE,
    pub instance_id: String,
    pub interface_id: String,
}

impl AdapterDevice {
    pub fn new(handler: HSWDEVICE, instance_id: String, interface_id: String) -> Self {
        Self {
            handler,
            instance_id,
            interface_id,
        }
    }

    pub fn start_adapter(&self) -> anyhow::Result<HANDLE> {
        //println!("interface_id:{}",self.interface_id);
        /*
           let file = OpenOptions::new()
               .write(true)
               .read(true)
               .create(false)
               //.custom_flags(0x40000000)
               //.custom_flags(FILE_FLAG_OVERLAPPED.0|FILE_ATTRIBUTE_SYSTEM.0)
               //.share_mode()
               .open(self.interface_id.clone())?;


           let result = unsafe {
               DeviceIoControl(
                   HANDLE(file.as_raw_handle() as _),
                   FOR_TUN_IOCTL_OPEN_ADAPTER,
                   None,
                   0,
                   None,
                   0,
                   None,
                   None,
               )
               .as_bool()
           };
           if !result {
               bail!("init adapter error: {:?}", unsafe { GetLastError() })
           }

        */

        let file = unsafe {
            let name = HSTRING::from(self.interface_id.clone());
            CreateFileW(
                PCWSTR(name.as_ptr()),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_SYSTEM | FILE_FLAG_OVERLAPPED,
                None,
            )
        }
        .unwrap();
        let result = unsafe {
            DeviceIoControl(
                file,
                FOR_TUN_IOCTL_OPEN_ADAPTER,
                None,
                0,
                None,
                0,
                None,
                None,
            )
            .as_bool()
        };

        if !result {
            unsafe { CloseHandle(file) };
            bail!("init adapter error: {:?}", unsafe { GetLastError() })
        }

        Ok(file)
    }
}

//TODO: inf_path change to compile path
pub fn init_device(
    device_guid: &GUID,
    name: &str,
    inf_path: &str,
) -> anyhow::Result<AdapterDevice> {
    //let inf_path = "C:/DriverTest/Drivers/ForTun.inf";
    let devices = enum_device(&FOR_TUN_DEV_CLASS, FOR_TUN_HWID)?;
    if devices.is_empty() {
        // There is no devices
        install_driver(inf_path)?; // TODO: this may install multiple times. need add more exact function to check if driver installed.
    } else {
        //TODO: compare version, if old exists, stop and remove all devices and reinstall new driver.
    }

    let (device_handler, device_instance_id) =
        create_device(device_guid, FOR_TUN_HWID, name, &FOR_TUN_DEV_CLASS)?;
    // need to wait interface initialize
    sleep(Duration::from_millis(500));
    let interfaces = get_device_interface(&GUID::from(FOR_TUN_INTERFACE_GUID), &device_instance_id)
        .context("get device interface")
        .map_err(|e| {
            unsafe {
                SwDeviceClose(device_handler);
            }
            e
        })?;

    if interfaces.len() == 0 {
        unsafe {
            SwDeviceClose(device_handler);
        }
        bail!("get error count device interfaces: {}", interfaces.len())
    }

    //TODO: interface filter, now the first is ok.
    let interface_id = interfaces.into_iter().next().unwrap();

    let device = AdapterDevice::new(device_handler, device_instance_id, interface_id);

    Ok(device)
}

fn install_driver(inf_path: &str) -> anyhow::Result<()> {
    let inf_path = HSTRING::from(inf_path);
    let inf_path = PCWSTR(inf_path.as_ptr());

    let ret = unsafe {
        SetupCopyOEMInfW(
            inf_path,
            PCWSTR::null(),
            SPOST_PATH,
            SP_COPY_NEWER.0,
            None,
            None,
            None,
        )
    };
    if !ret.as_bool() {
        unsafe {
            return Err(anyhow!(
                "install driver:{} fail, code:{}",
                inf_path.display(),
                GetLastError().0
            ));
        }
    }
    Ok(())
}

// this would copy buf to string
fn get_pcwstr_list(mut buf: Vec<u16>) -> Vec<String> {
    let mut index = 0;
    let mut result = Vec::new();

    let mut string = PCWSTR::from_raw(buf[index..].as_mut_ptr());
    while buf[index] != 0 && !string.is_null() {
        unsafe {
            result.push(string.to_string().unwrap());
        }
        unsafe {
            index += wcslen(string) + 1;
        }
        if index >= buf.len() - 1 {
            break;
        }
        string = PCWSTR::from_raw(buf[index..].as_mut_ptr());
    }
    result
}

struct CreateDeviceContext {
    pub instance_id: String,
    pub event: HANDLE,
    pub success: bool,
}

pub fn create_device(
    device_id: &GUID,
    hwid: &str,
    device_name: &str,
    device_class_guid: &GUID,
) -> anyhow::Result<(HSWDEVICE, String)> {
    let description = HSTRING::from(format!("{device_name} device\0\0"));
    let description = PCWSTR(description.as_ptr());

    let device_id = HSTRING::from(format!("{device_id:?}"));
    let device_id = PCWSTR(device_id.as_ptr());

    let friendly_name = HSTRING::from(format!("{device_name}\0"));
    let friendly_name = PCWSTR(friendly_name.as_ptr());

    let device_name = HSTRING::from(device_name);
    let device_name = PCWSTR(device_name.as_ptr());

    //let enumerator_name = HSTRING::from(format!("{}\0",FOR_TUN_ENUMERATOR));
    //let enumerator_name = PCWSTR::from(&enumerator_name);

    let hwids = HSTRING::from(format!("{hwid}\0"));
    let hwids = PCWSTR(hwids.as_ptr());

    //let hwid = HSTRING::from(hwid);
    //let hwid = PCWSTR::from(&hwid);
    let mut create_info: SW_DEVICE_CREATE_INFO = unsafe { zeroed() };
    create_info.cbSize = size_of::<SW_DEVICE_CREATE_INFO>() as u32;
    create_info.pszzHardwareIds = hwids;
    create_info.pszInstanceId = device_id;
    create_info.pszDeviceDescription = description;
    create_info.CapabilityFlags =
        (SWDeviceCapabilitiesSilentInstall.0 | SWDeviceCapabilitiesDriverRequired.0) as u32;

    let device_properties = vec![
        DEVPROPERTY {
            CompKey: DEVPROPCOMPKEY {
                Key: DEVPKEY_Device_ClassGuid,
                Store: DEVPROP_STORE_SYSTEM,
                LocaleName: PCWSTR::null(),
            },
            Type: DEVPROP_TYPE_GUID,
            Buffer: &mut device_class_guid.clone() as *mut _ as *mut c_void,
            BufferSize: size_of::<GUID>() as u32,
        },
        // could not set it DEVPKEY_Device_EnumeratorName
        // DEVPROPERTY {
        //   CompKey: DEVPROPCOMPKEY {
        //       Key: ,
        //       Store: DEVPROP_STORE_SYSTEM,
        //       LocaleName: PCWSTR::null(),
        //   },
        //     Type: DEVPROP_TYPE_STRING,
        //     Buffer: enumerator_name.as_ptr() as *mut c_void,
        //     BufferSize: ((unsafe { wcslen(enumerator_name) }  + 1 ) * size_of::<u16>())  as _,
        // },
        DEVPROPERTY {
            CompKey: DEVPROPCOMPKEY {
                Key: DEVPKEY_Device_FriendlyName,
                Store: DEVPROP_STORE_SYSTEM,
                LocaleName: PCWSTR::null(),
            },
            Type: DEVPROP_TYPE_STRING,
            Buffer: friendly_name.as_ptr() as *mut c_void,
            BufferSize: ((unsafe { wcslen(friendly_name) } + 1) * size_of::<u16>()) as _,
        },
    ];

    let event: HANDLE = unsafe {
        windows::Win32::System::Threading::CreateEventW(None, false, false, PCWSTR::null())?
    };
    let mut context = CreateDeviceContext {
        event,
        instance_id: String::new(),
        success: false,
    };

    let sw_device = unsafe {
        SwDeviceCreate(
            device_name,
            w!("HTREE\\ROOT\\0"),
            &create_info,
            Some(&device_properties),
            Some(creation_callback),
            Some(&mut context as *mut _ as *const c_void),
        )
    }
    .context("SwDeviceCreate Fail")?;
    let wait_result =
        unsafe { windows::Win32::System::Threading::WaitForSingleObject(event, 20 * 1000) };
    if wait_result != WAIT_OBJECT_0 {
        unsafe {
            return Err(anyhow!(
                "create sw device error: {}, last_error:{}",
                wait_result.0,
                GetLastError().0
            ));
        }
    }
    if !context.success {
        unsafe {
            bail!(
                "create sw device error in callback, last error:{}",
                GetLastError().0
            )
        }
    }

    Ok((HSWDEVICE(sw_device), context.instance_id))
}

unsafe extern "system" fn creation_callback(
    _device: HSWDEVICE,
    _create_result: HRESULT,
    context: *const c_void,
    _device_instance_id: PCWSTR,
) {
    let mut context = &mut *(context as *mut CreateDeviceContext);
    if _create_result.is_ok() {
        context.instance_id = _device_instance_id.to_string().unwrap();
        context.success = true;
        //println!("device creation device_id：{}", _device_instance_id.display());
    } else {
        context.success = false;
        //println!("device creation device_id error, {}",_create_result);
    }
    let ret = windows::Win32::System::Threading::SetEvent(context.event);
    if !ret.as_bool() {
        //println!("set event error, {}", GetLastError().0)
    }
}

pub fn route_set(adapter_index: String, address: &str) -> anyhow::Result<()> {
    let ip_cidr = cidr_utils::cidr::IpCidr::from_str(address)?;
    let netmask = match ip_cidr {
        IpCidr::V4(cidr) => cidr.get_mask_as_ipv4_addr().to_string(),
        IpCidr::V6(cidr) => cidr.get_mask_as_ipv6_addr().to_string(),
    };
    Command::new("netsh")
        .args([
            "interface",
            "ip",
            "set",
            "address",
            &adapter_index,
            "static",
            &ip_cidr.to_string(),
            &netmask,
            //"1",
        ])
        .status()?;
    Ok(())
}

pub fn mtu_set(adapter_index: String, mtu: u32, is_v4: bool) -> anyhow::Result<()> {
    let ip_type = if is_v4 { "ipv4" } else { "ipv6" };
    Command::new("netsh")
        .args([
            "interface",
            ip_type,
            "set",
            "subinterface",
            &adapter_index,
            &format!("mtu={}", mtu),
        ])
        .status()?;
    Ok(())
}

pub fn net_config(
    device_instance_id: String,
    address: &str,
    netmask: &str,
    //gateway: str,
    mtu: u32,
) -> anyhow::Result<()> {
    let index = get_net_index(device_instance_id)?.to_string();
    //netsh interface ip set address 5 static 10.0.0.2 255.255.255.0 10.0.0.1
    tracing::debug!("netsh interface ip set  address {index} static {address} {netmask}");
    Command::new("netsh")
        .args([
            "interface",
            "ip",
            "set",
            "address",
            &index,
            "static",
            address,
            netmask,
            //gateway,
        ])
        .status()?;
    let ip_type = if address.parse::<IpAddr>()?.is_ipv4() {
        "ipv4"
    } else {
        "ipv6"
    };

    tracing::debug!("netsh interface {ip_type} set subinterface {index} mtu={mtu}");
    //netsh interface ipv4 set subinterface 5 mtu=1428
    Command::new("netsh")
        .args([
            "interface",
            ip_type,
            "set",
            "subinterface",
            &index,
            &format!("mtu={}", mtu),
        ])
        .status()?;
    Ok(())
}

pub fn get_net_index(device_instance_id: String) -> anyhow::Result<u32> {
    //key: HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Class\<class>\<id> registry key.

    let mut key = HKEY::default();

    let _display_name_buf: [u16; 1024] = unsafe { zeroed() };

    let key_path_str =
        format!("SYSTEM\\CurrentControlSet\\Control\\Class\\{{{FOR_TUN_DEV_CLASS:?}}}");

    let key_path = HSTRING::from(key_path_str.clone());
    let key_path = PCWSTR(key_path.as_ptr());

    if unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path,
            0,
            KEY_READ | KEY_ENUMERATE_SUB_KEYS,
            &mut key,
        )
    } != NO_ERROR
    {
        unsafe { RegCloseKey(key) };
        bail!("can not open registry key: {}", key_path_str);
    } else {
        let mut value_buffer = vec![0; 200];
        let mut index = 0;
        let mut class_name_buf = [0u16; 256];
        let name_buf = PWSTR::from_raw(class_name_buf.as_mut_ptr());

        loop {
            let mut class_name_length = 256;
            let ret = unsafe {
                RegEnumKeyExW(
                    key,
                    index,
                    name_buf,
                    &mut class_name_length,
                    None,
                    PWSTR::null(),
                    None,
                    None,
                )
            };

            if ret == ERROR_NO_MORE_ITEMS {
                break;
            } else if ret != NO_ERROR {
                bail!(
                    "can not enum registry key: {}, error:{:?}",
                    key_path_str,
                    ret
                )
            }

            let instance_name_key =
                unsafe { format!("{}\\{}", key_path_str, name_buf.to_string().unwrap()) };
            // println!("instance_name_key:{instance_name_key}");
            let instance_name_key = HSTRING::from(instance_name_key);
            let instance_name_key = PCWSTR(instance_name_key.as_ptr());
            let mut instance_key = HKEY::default();
            unsafe {
                RegOpenKeyExW(
                    HKEY_LOCAL_MACHINE,
                    instance_name_key,
                    0,
                    KEY_READ,
                    &mut instance_key,
                )
            };

            let mut size = 200;
            unsafe {
                RegQueryValueExW(
                    instance_key,
                    w!("DeviceInstanceID"),
                    None,
                    None,
                    Some(value_buffer.as_mut_ptr()),
                    Some(&mut size),
                )
            };

            let reg_value =
                unsafe { PCWSTR::from_raw(value_buffer.as_mut_ptr().cast::<u16>()).to_string() }
                    .unwrap();
            if reg_value == device_instance_id {
                let mut size = 200;
                //NetCfgInstanceId
                unsafe {
                    RegQueryValueExW(
                        instance_key,
                        w!("NetCfgInstanceId"),
                        None,
                        None,
                        Some(value_buffer.as_mut_ptr()),
                        Some(&mut size),
                    )
                };
                let net_cfg_instance_id = unsafe {
                    PCWSTR::from_raw(value_buffer.as_mut_ptr().cast::<u16>()).to_string()
                }
                .unwrap();
                let adapter_name = HSTRING::from(format!("\\DEVICE_TCPIP_{net_cfg_instance_id}"));
                let adapter_name = PCWSTR(adapter_name.as_ptr());
                let mut r = 0;
                unsafe {
                    GetAdapterIndex(adapter_name, &mut r);
                }

                unsafe { RegCloseKey(instance_key) };
                return Ok(r);
            } else {
                unsafe {
                    RegCloseKey(instance_key);
                }
                index += 1;
            }
        }
        bail!("could not get device NetCfgInstanceId")
    }
}

pub fn get_device_interface(
    interface_class_guid: &GUID,
    device_instance_id: &str,
) -> anyhow::Result<Vec<String>> {
    let device_instance_id = HSTRING::from(device_instance_id);
    let device_instance_id = PCWSTR(device_instance_id.as_ptr());
    let mut length = 0;
    let flag = CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES;
    let error_ret = unsafe {
        CM_Get_Device_Interface_List_SizeW(
            &mut length,
            interface_class_guid,
            device_instance_id,
            flag,
        )
    };
    if error_ret != CR_SUCCESS {
        return Err(anyhow!(
            "get device interface list size error: {}",
            error_ret.0
        ));
    }
    if length == 0 {
        println!("device list is empty");
        return Ok(Vec::new());
    }
    let mut buffer: Vec<u16> = vec![0; length as usize];
    let cr_ret = unsafe {
        CM_Get_Device_Interface_ListW(interface_class_guid, device_instance_id, &mut buffer, flag)
    };
    if cr_ret != CR_SUCCESS {
        return Err(anyhow!("get device interface list error: {}", cr_ret.0));
    }
    unsafe {
        buffer.set_len(length as usize);
    }
    let buffer = get_pcwstr_list(buffer);
    Ok(buffer)
}

fn remove_device(dev_info: HDEVINFO, dev_info_data: &SP_DEVINFO_DATA) {
    let mut rmd_params = unsafe { zeroed::<SP_REMOVEDEVICE_PARAMS>() };
    rmd_params.ClassInstallHeader.cbSize = size_of::<SP_CLASSINSTALL_HEADER>() as u32;
    rmd_params.ClassInstallHeader.InstallFunction = DIF_REMOVE;
    rmd_params.Scope = DI_REMOVEDEVICE_GLOBAL;
    rmd_params.HwProfile = 0;
    unsafe {
        SetupDiSetClassInstallParamsW(
            dev_info,
            Some(dev_info_data),
            Some(&rmd_params.ClassInstallHeader),
            size_of::<SP_REMOVEDEVICE_PARAMS>() as u32,
        )
    };
}

fn enum_device(device_class_id: &GUID, hwid: &str) -> anyhow::Result<Vec<String>> {
    let mut device_list_len = 0;
    let device_class_id = HSTRING::from(format!("{{{device_class_id:?}}}\0"));
    let device_class_id = PCWSTR(device_class_id.as_ptr());

    let flag = CM_GETIDLIST_FILTER_CLASS;
    let cr = unsafe { CM_Get_Device_ID_List_SizeW(&mut device_list_len, device_class_id, flag) };

    if cr != CR_SUCCESS {
        return Err(anyhow!("Fail to get device list size: {:?}", cr));
    }

    //let mut buffer = Vec::with_capacity(device_list_len as usize);
    let mut buffer = vec![0; device_list_len as usize];

    let cr = unsafe { CM_Get_Device_ID_ListW(device_class_id, &mut buffer, flag) };
    if cr != CR_SUCCESS {
        return Err(anyhow!("Fail to get device list:{:?}", cr));
    }

    let mut dev_inst: u32 = 0;
    let mut property_type: u32 = 0;

    let mut property_value: Vec<u8> = Vec::with_capacity(2048);
    let mut property_value_length = 0;

    let mut index = 0;
    let mut device_id = PCWSTR::from_raw(buffer[index..].as_mut_ptr());

    let mut result: Vec<String> = Vec::new();

    while buffer[index] != 0 && !device_id.is_null() {
        let cr = unsafe {
            CM_Locate_DevNodeW(
                &mut dev_inst,
                device_id,
                CM_LOCATE_DEVINST_NORMAL | CM_LOCATE_DEVNODE_PHANTOM,
            )
        };

        if cr != CR_SUCCESS {
            if cr != CR_NO_SUCH_DEVNODE {
                unsafe {
                    return Err(anyhow!(
                        "Fail to locate dev node: {}, error:{}",
                        device_id.display(),
                        cr.0
                    ));
                }
            }
        } else {
            unsafe {
                CM_Get_DevNode_PropertyW(
                    dev_inst,
                    &DEVPKEY_Device_HardwareIds,
                    &mut property_type,
                    Some(property_value.as_mut_ptr()),
                    &mut property_value_length,
                    0,
                );
            }

            if property_value_length > 0 {
                unsafe {
                    property_value.set_len(property_value_length as usize);
                };

                let name = unsafe {
                    PCWSTR::from_raw(property_value.as_mut_ptr().cast::<u16>()).to_string()
                };

                if let Ok(name) = name {
                    if name == hwid {
                        unsafe {
                            if let Ok(device_id) = device_id.to_string() {
                                result.push(device_id);
                            }
                        }
                    }
                }
            }
        }

        unsafe {
            index += wcslen(device_id) + 1;
        }
        if index >= buffer.len() - 1 {
            break;
        }
        device_id = PCWSTR::from_raw(buffer[index..].as_mut_ptr());
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use windows::core::PCSTR;
    use windows::Win32::Storage::FileSystem::{
        CreateFileA, FILE_ATTRIBUTE_SYSTEM, FILE_FLAG_OVERLAPPED, FILE_GENERIC_READ,
        FILE_GENERIC_WRITE, FILE_SHARE_MODE, FILE_SHARE_NONE, OPEN_EXISTING,
    };

    #[test]
    fn test() {
        let a = PCSTR::from_raw("\\\\.\\ovpn-dco\0".as_ptr());

        let a = unsafe {
            CreateFileA(
                a,
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_SYSTEM | FILE_FLAG_OVERLAPPED,
                None,
            )
        }
        .unwrap();
        println!("finish....");
    }
}
