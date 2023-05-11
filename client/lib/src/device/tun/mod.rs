use cfg_if::cfg_if;

pub(crate) mod sys;

cfg_if! {
    if #[cfg(not(target_os = "windows"))] {
        mod unix;
        pub(crate) use unix::{create_async_tun, WritePart, ReadPart};
    } else {
        mod window;
        pub(crate) use window::{create_async_tun, WritePart, ReadPart};

    }
}

