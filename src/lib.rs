use std::{env, ffi::c_char};

/// cbindgen:define
pub const TEXT_SIZE: usize = 256;

#[repr(C)]
pub struct VersionInfo {
    pub name: [c_char; TEXT_SIZE],
    pub major: i8,
    pub minor: i8,
    pub patch: i8,
    pub description: [c_char; TEXT_SIZE],
    pub author: [c_char; TEXT_SIZE],
    pub system: [c_char; TEXT_SIZE],
    pub architecture: [c_char; TEXT_SIZE],
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self {
            name: {
                let name_bytes: &[u8] = env!("CARGO_PKG_NAME").as_bytes();
                let mut name = [0i8; TEXT_SIZE];
                let len = name_bytes.len().min(TEXT_SIZE);
                for (i, &b) in name_bytes.iter().take(len).enumerate() {
                    name[i] = b as i8;
                }
                name
            },
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
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
                let auth_bytes: &[u8] = env!("CARGO_PKG_AUTHORS").as_bytes();
                let mut auth = [0i8; TEXT_SIZE];
                let len = auth_bytes.len().min(TEXT_SIZE);
                for (i, &b) in auth_bytes.iter().take(len).enumerate() {
                    auth[i] = b as i8;
                }
                auth
            },
            system: {
                let system_bytes: &[u8] = std::env::consts::OS.as_bytes();
                let mut system = [0i8; TEXT_SIZE];
                let len = system_bytes.len().min(TEXT_SIZE);
                for (i, &b) in system_bytes.iter().take(len).enumerate() {
                    system[i] = b as i8;
                }
                system
            },
            architecture : {
                let architecture_bytes: &[u8] = std::env::consts::ARCH.as_bytes();
                let mut architecture = [0i8; TEXT_SIZE];
                let len = architecture_bytes.len().min(TEXT_SIZE);
                for (i, &b) in architecture_bytes.iter().take(len).enumerate() {
                    architecture[i] = b as i8;
                }
                architecture 
            },
        }
    }
}

use once_cell::sync::Lazy;

#[unsafe(no_mangle)]
pub extern "C" fn csdemo_version() -> *const VersionInfo {
    static VERSION: Lazy<VersionInfo> = Lazy::new(|| VersionInfo::default());
    &*VERSION as *const VersionInfo
}