use cfg_if::cfg_if;
cfg_if! {
     if #[cfg(target_os="macos")] {
        mod macos;
        pub use macos::AutoLaunch;
    }
}

//TODO: test other platform with auto-launch-extra, be careful about root permission!