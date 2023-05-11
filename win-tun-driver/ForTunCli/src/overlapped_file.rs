use std::io;
use windows::Win32::Foundation::{CloseHandle, GetLastError, BOOL, ERROR_IO_PENDING, HANDLE};
use windows::Win32::Storage::FileSystem::{ReadFile, WriteFile};
use windows::Win32::System::IO::{CancelIoEx, DeviceIoControl, GetOverlappedResult, OVERLAPPED};

pub struct WinOverlappedFile {
    pub file: HANDLE,
}

impl WinOverlappedFile {
    pub fn new(file: HANDLE) -> anyhow::Result<Self> {
        //let mut overlapped = OVERLAPPED::default();
        //overlapped.hEvent = unsafe {CreateEventW(None, false, false, None)}?;

        Ok(Self {
            file,
            //read_overlapped: overlapped,
        })
    }

    pub fn device_io_control_sync(
        &mut self,
        dwiocontrolcode: u32,
        lpinbuffer: Option<*const ::core::ffi::c_void>,
        ninbuffersize: u32,
        lpoutbuffer: Option<*mut ::core::ffi::c_void>,
        noutbuffersize: u32,
        lpbytesreturned: Option<*mut u32>,
    ) -> BOOL {
        unsafe {
            DeviceIoControl(
                self.file,
                dwiocontrolcode,
                lpinbuffer,
                ninbuffersize,
                lpoutbuffer,
                noutbuffersize,
                lpbytesreturned,
                None,
            )
        }
    }

    // driver would wait
    pub fn read(&self, buf: &mut [u8], overlapped: &mut OVERLAPPED) -> io::Result<usize> {
        let mut size = 0;
        let ret = unsafe {
            ReadFile(
                self.file,
                Some(buf.as_mut_ptr() as _),
                buf.len() as u32,
                Some(&mut size),
                Some(overlapped),
            )
        };
        return if ret.as_bool() {
            //success
            Ok(size as usize)
        } else {
            let last_error = unsafe { GetLastError() };
            if last_error == ERROR_IO_PENDING {
                let r = unsafe { GetOverlappedResult(self.file, overlapped, &mut size, true) };
                if r.as_bool() {
                    Ok(size as usize)
                } else {
                    Err(io::Error::from_raw_os_error(last_error.0 as i32))
                }
            } else {
                Err(io::Error::from_raw_os_error(last_error.0 as i32))
            }
        };
    }
    /*
    pub fn read_result(&self, overlapped: &mut OVERLAPPED) -> io::Result<Option<usize>> {
        let mut size = 0;
        let r = unsafe {
            GetOverlappedResult(
                self.file,
                overlapped,
                &mut size,
                false,
            )
        };
        if r.as_bool() {
            Ok(Some(size as usize))
        } else {
            let last_error = unsafe { GetLastError() };
            if last_error == ERROR_IO_PENDING {
                Ok(None)
            } else {
                Err(io::Error::from_raw_os_error(last_error.0 as i32))
            }
        }
    }*/

    // this is quick , so block
    pub fn write(&self, buf: &[u8], overlapped: &mut OVERLAPPED) -> io::Result<usize> {
        let mut size = 0;
        let r = unsafe {
            WriteFile(
                self.file,
                Some(buf.as_ptr() as _),
                buf.len() as u32,
                Some(&mut size),
                Some(overlapped),
            )
        };
        return if r.as_bool() {
            Ok(size as usize)
        } else {
            let last_error = unsafe { GetLastError() };
            if last_error == ERROR_IO_PENDING {
                let r = unsafe { GetOverlappedResult(self.file, overlapped, &mut size, true) };
                if r.as_bool() {
                    Ok(size as usize)
                } else {
                    Err(io::Error::from_raw_os_error(last_error.0 as i32))
                }
            } else {
                Err(io::Error::from_raw_os_error(last_error.0 as i32))
            }
        };
    }

    pub fn cancel_io(&self, overlapped: &mut OVERLAPPED) -> io::Result<()> {
        if unsafe { CancelIoEx(self.file, Some(overlapped)) }.as_bool() {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
            //Err(io::Error::from_raw_os_error(last_error.0 as i32))
            //Err(anyhow!("read file fail:{:?}", last_error))
            //Err(anyhow!("cancel file fail:{:?}", unsafe {GetLastError()}))
        }
    }
}

impl Drop for WinOverlappedFile {
    fn drop(&mut self) {
        if !self.file.is_invalid() {
            unsafe { CloseHandle(self.file) };
        }
    }
}
