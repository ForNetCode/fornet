use cfg_if::cfg_if;

pub(crate) mod sys;

cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "macos"))] {
        mod unix;
        pub(crate) use unix::{create_async_tun, WritePart, ReadPart};
    } else if #[cfg(target_os = "android")] {
        mod android;
        pub(crate) use android::{create_async_tun, WritePart, ReadPart};
    }
    else {
        mod window;
        pub(crate) use window::{create_async_tun, WritePart, ReadPart};
    }
}

