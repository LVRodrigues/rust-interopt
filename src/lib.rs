use std::{env, ffi::c_char};

/// cbindgen:define
pub const TEXT_SIZE: usize = 256;

#[repr(C)]
pub struct VersionInfo {
    pub major: i8,
    pub minor: i8,
    pub build: i8,
    pub description: [c_char; TEXT_SIZE],
    pub author: [c_char; TEXT_SIZE],
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            build: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
            description: {
                let desc_bytes: &[u8] = env!("CARGO_PKG_DESCRIPTION").as_bytes();
                let mut desc = [0i8; TEXT_SIZE];
                let len = desc_bytes.len().min(TEXT_SIZE);
                for (i, &b) in desc_bytes.iter().take(len).enumerate() {
                    desc[i] = b as i8;
                }
                desc
            },
            author: {
                let auth_bytes =env!("CARGO_PKG_AUTHORS").as_bytes();
                let mut auth = [0i8; TEXT_SIZE];
                let len = auth_bytes.len().min(TEXT_SIZE);
                for (i, &b) in auth_bytes.iter().take(len).enumerate() {
                    auth[i] = b as i8;
                }
                auth
            },
        }
    }
}

// use once_cell::sync::Lazy;

#[unsafe(no_mangle)]
pub extern "C" fn csdemo_version() ->  VersionInfo {
    // static VERSION: Lazy<VersionInfo> = Lazy::new(|| VersionInfo::default());
    // &*VERSION as *const VersionInfo
    VersionInfo::default()
}