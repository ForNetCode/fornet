use cfg_if::cfg_if;

cfg_if! {
     if #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::{set_route,set_alias, remove_route};
    } else if #[cfg(target_os = "linux")]{
        mod linux;
        pub use linux::{set_address};
    }
}
