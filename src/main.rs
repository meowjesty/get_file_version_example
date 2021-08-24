// windows crate: https://github.com/microsoft/windows-rs
// examples: https://github.com/microsoft/windows-samples-rs
mod bindings {
    windows::include_bindings!();
}

use std::{ffi::c_void, mem::MaybeUninit};

use bindings::{
    Windows::Win32::Foundation::HINSTANCE,
    Windows::Win32::Foundation::PSTR,
    // https://docs.microsoft.com/en-us/windows/win32/api/winver/nf-winver-getfileversioninfoa
    Windows::Win32::Storage::FileSystem::GetFileVersionInfoA,

    // https://docs.microsoft.com/en-us/windows/win32/api/winver/nf-winver-getfileversioninfosizea
    Windows::Win32::Storage::FileSystem::GetFileVersionInfoSizeA,

    // https://docs.microsoft.com/en-us/windows/win32/api/winver/nf-winver-verqueryvaluea
    Windows::Win32::Storage::FileSystem::VerQueryValueA,

    // https://docs.microsoft.com/en-us/windows/win32/api/verrsrc/ns-verrsrc-vs_fixedfileinfo
    Windows::Win32::Storage::FileSystem::VS_FIXEDFILEINFO,

    // https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulefilenamea
    Windows::Win32::System::LibraryLoader::GetModuleFileNameA,
};

fn main() {
    let max_path = 1500;
    let mut version_file = String::with_capacity(max_path);

    let module_file = unsafe {
        GetModuleFileNameA(
            HINSTANCE::NULL,
            *(version_file.as_mut_ptr() as *mut PSTR),
            version_file.len() as u32,
        )
    };
    println!("module_file {}", module_file);
    println!("version_file {}", version_file);

    let mut version_handle = 0;
    let version_info_size =
        unsafe { GetFileVersionInfoSizeA(version_file.clone(), &mut version_handle) };
    println!("version_info_size {}", version_info_size);

    if version_info_size > 0 {
        let mut version_data = vec![0u8; version_info_size as usize];
        let version_info = unsafe {
            GetFileVersionInfoA(
                version_file,
                0,
                version_info_size,
                version_data.as_mut_ptr() as *mut c_void,
            )
        };

        if version_info.as_bool() {
            let mut buffer = MaybeUninit::zeroed();
            let mut buffer_size = 0;
            let version_exists = unsafe {
                VerQueryValueA(
                    version_data.as_ptr() as *const c_void,
                    "\\",
                    buffer.as_mut_ptr(),
                    &mut buffer_size,
                )
            };

            if version_exists.as_bool() {
                let data = unsafe { buffer.assume_init() };
                let fixed_file_info: *mut VS_FIXEDFILEINFO = data.cast();

                unsafe {
                    if (*fixed_file_info).dwSignature == 0xfeef04bd {
                        println!(
                            "File version: {} {} {} {}",
                            ((*fixed_file_info).dwFileVersionMS >> 16) & 0xffff,
                            ((*fixed_file_info).dwFileVersionMS >> 0) & 0xffff,
                            ((*fixed_file_info).dwFileVersionLS >> 16) & 0xffff,
                            ((*fixed_file_info).dwFileVersionLS >> 16) & 0xffff,
                        )
                    }
                }
            }
        }
    }
}
