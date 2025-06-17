use std::{env, ffi::{c_char, CStr}, sync::Mutex};
use once_cell::sync::Lazy;

/// cbindgen:define
pub const TEXT_SIZE: usize = 256;

///
/// Version information about the library.
/// 
#[repr(C)]
pub struct Version {
    /// Library name
    pub name: [c_char; TEXT_SIZE],
    /// Major version number
    pub major: u8,
    /// Minor version number
    pub minor: u8,
    /// Patch version number
    pub patch: u8,
    /// Description of the library
    pub description: [c_char; TEXT_SIZE],
    /// Author of the library
    pub author: [c_char; TEXT_SIZE],
    /// System the library is built for
    pub system: [c_char; TEXT_SIZE],
    /// Architecture the library is built for
    pub architecture: [c_char; TEXT_SIZE],
}

///
/// Inform the sex of the person.
/// 
#[repr(C)]
pub enum Sex {
    MALE,
    FEMALE,
}

///
/// Informatoin about a Person to store.
/// 
#[repr(C)]
pub struct Person {
    /// ID auto generated
    pub id:  u32,
    /// Name of the person
    pub name: [c_char; TEXT_SIZE],
    /// Age of the person
    pub age: u16,
    /// Sex of the person
    pub sex: Sex,
}

impl Default for Version {
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

// A global vector protected by a Mutex for thread-safe access
pub static PEOPLE: Lazy<Mutex<Vec<Person>>> = Lazy::new(|| Mutex::new(Vec::new()));

// A ID generator.
pub static ID: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

///
/// Retrieve version information about the library.
/// 
#[unsafe(no_mangle)]
pub extern "C" fn csdemo_version() -> *const Version {
    static VERSION: Lazy<Version> = Lazy::new(|| Version::default());
    &*VERSION as *const Version
}

///
/// Store information about a person.
/// 
#[unsafe(no_mangle)]
pub extern "C" fn csdemo_person_push(name: *const c_char, age: u16, sex: Sex) -> *const Person {
    let rname = unsafe { CStr::from_ptr(name) }.to_str().unwrap().to_owned();
    let len = rname.len().min(TEXT_SIZE);

    // Generate a new ID
    let mut id_guard = ID.lock().unwrap();
    *id_guard += 1;
    let id = *id_guard;

    // Convert name to [c_char; TEXT_SIZE]
    let mut name_arr = [0 as c_char; TEXT_SIZE];
    for (i, &b) in rname.as_bytes().iter().take(len).enumerate() {
        name_arr[i] = b as c_char;
    }

    let person = Person { id, name: name_arr, age, sex };
    
    let mut people = PEOPLE.lock().unwrap();
    people.push(person);

    &people[people.len() - 1] as *const Person
}

///
/// Count of People stored.
/// 
#[unsafe(no_mangle)]
pub extern "C" fn csdemo_person_count() -> usize {
    PEOPLE.lock().unwrap().len()
}

///
/// Retrieve information about last person stored and remove it from storage.
/// 
#[unsafe(no_mangle)]
pub extern "C" fn csdemo_person_pop() -> *mut Person {
    let person = PEOPLE.lock().unwrap().pop();
    match person {
        Some(p) => Box::into_raw(Box::new(p)),
        None => std::ptr::null_mut(),
    }
}

///
/// Free the pointer allocated by Rust.
/// 
#[unsafe(no_mangle)]
pub extern "C" fn csdemo_person_free(person: *mut Person) {
    let _ = unsafe { Box::from_raw(person) };
}