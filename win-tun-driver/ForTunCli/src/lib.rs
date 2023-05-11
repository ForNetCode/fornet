mod device_ops;
pub mod overlapped_file;

use crate::overlapped_file::WinOverlappedFile;
use anyhow::bail;
pub use device_ops::get_net_index;
pub use device_ops::init_device;
pub use device_ops::net_config;
pub use device_ops::AdapterDevice;
use std::sync::Arc;
use tokio::io;
use windows::core::GUID;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::IO::OVERLAPPED;

pub struct OverlappedWrap {
    pub overlapped: OVERLAPPED,
}

//unsafe impl Send for OverlappedWrap {}

pub struct ReadFile {
    file: Arc<WinOverlappedFile>,
    overlapped: OVERLAPPED,
}

unsafe impl Send for ReadFile {}

impl ReadFile {
    // this would block...
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf, &mut self.overlapped)
    }
}

impl Drop for ReadFile {
    fn drop(&mut self) {
        let _ = self.file.cancel_io(&mut self.overlapped);
    }
}

pub struct WriteFile {
    file: Arc<WinOverlappedFile>,
    overlapped: OVERLAPPED,
}

unsafe impl Send for WriteFile {}

impl WriteFile {
    //this would Finish quick
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf, &mut self.overlapped)
    }
}

impl Drop for WriteFile {
    fn drop(&mut self) {
        let _ = self.file.cancel_io(&mut self.overlapped);
    }
}

pub type TunSocket = (ReadFile, WriteFile, AdapterDevice);

pub fn create_async_tun(device_id: &GUID, name: &str) -> anyhow::Result<TunSocket> {
    let device = init_device(device_id, name, "C:/DriverTest/Drivers/ForTun.inf")?;
    let file = device.start_adapter()?;
    let file = match WinOverlappedFile::new(file) {
        Ok(file) => Arc::new(file),
        Err(err) => {
            unsafe { CloseHandle(file) };
            bail!("create winOverlappedFile error:{}", err)
        }
    };

    Ok((
        ReadFile {
            file: file.clone(),
            overlapped: OVERLAPPED::default(),
        },
        WriteFile {
            file: file.clone(),
            overlapped: OVERLAPPED::default(),
        },
        device,
    ))
}

/*
pub struct TunSocket {
    pub file: tokio::fs::File,
    pub device: AdapterDevice,
}

pub fn create_async_tun(device_id: &GUID, name: &str) -> anyhow::Result<TunSocket> {
    let device = init_device(device_id, name, "C:/DriverTest/Drivers/ForTun.inf")?;
    let file = device.start_adapter()?;

    let file = tokio::fs::File::from_std(file);
    Ok(TunSocket { file, device })
}


impl AsyncRead for TunSocket {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().file).poll_read(cx, buf)
    }
}

impl AsyncWrite for TunSocket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut self.get_mut().file).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut self.get_mut().file).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut self.get_mut().file).poll_shutdown(cx)
    }
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut self.get_mut().file).poll_write_vectored(cx, bufs)
    }
}
*/
