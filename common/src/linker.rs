// code taken from https://github.com/dtolnay/linkme/,
// MIT license

//! name conventions used by the linker on different platforms.
//! This is used to make the "link_section" magic working

/// section name used by the linker on this platform
pub const SECTION: &str = {
    #[cfg(any(
        target_os = "none",
        target_os = "linux",
        target_os = "android",
        target_os = "fuchsia",
        target_os = "psp",
        target_os = "freebsd",
        target_os = "wasm"
    ))]
    {
        "manganis"
    }

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]
    {
        "__DATA,__manganis,regular,no_dead_strip"
    }

    #[cfg(target_os = "windows")]
    {
        ".manganis$b"
    }

    #[cfg(target_os = "illumos")]
    {
        "set_manganis"
    }
};

/// section name used by the linker on this platform
pub const SECTION_START: &str = {
    #[cfg(any(
        target_os = "none",
        target_os = "linux",
        target_os = "android",
        target_os = "fuchsia",
        target_os = "psp",
        target_os = "freebsd",
        target_os = "wasm"
    ))]
    {
        "__start_manganis"
    }

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]
    {
        "\x01section$start$__DATA$__manganis"
    }

    #[cfg(target_os = "windows")]
    {
        ".manganis$a"
    }

    #[cfg(target_os = "illumos")]
    {
        "__start_set_manganis"
    }
};

#[macro_export]
/// expose the manganis section to the rust linker
macro_rules! link_section_start_offset {
    ($name: ident) => {
        extern "Rust" {
            #[cfg_attr(
                any(
                    target_os = "none",
                    target_os = "linux",
                    target_os = "android",
                    target_os = "fuchsia",
                    target_os = "psp",
                    target_os = "freebsd",
                    target_os = "wasm"
                ),
                link_name = "__start_manganis"
            )]
            #[cfg_attr(
                any(target_os = "macos", target_os = "ios", target_os = "tvos"),
                link_name = "\x01section$start$__DATA$__manganis"
            )]
            #[cfg_attr(target_os = "windows", link_name = ".manganis$a")]
            #[cfg_attr(target_os = "illumos", link_name = "__start_set_manganis")]
            static $name: u8;
        }
    };
}
