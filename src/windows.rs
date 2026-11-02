//! Windows implementation.

use std::ffi;
use std::process::Command;

use crate::c_helpers;
use crate::{DiskInfo, Error, LoadAvg, MemInfo};

/// Get operation system type.
pub fn os_type() -> Result<String, Error> {
    let typ =
        unsafe { ffi::CStr::from_ptr(c_helpers::get_os_type() as *const libc::c_char).to_bytes() };
    Ok(String::from_utf8_lossy(typ).into_owned())
}

/// Get operation system release version.
pub fn os_release() -> Result<String, Error> {
    unsafe {
        let rp = c_helpers::get_os_release() as *const libc::c_char;
        if rp == std::ptr::null() {
            Err(Error::Unknown)
        } else {
            let typ = ffi::CStr::from_ptr(rp).to_bytes();
            Ok(String::from_utf8_lossy(typ).into_owned())
        }
    }
}

/// Get cpu num quantity.
///
/// Notice, it returns the logical cpu quantity.
pub fn cpu_num() -> Result<u32, Error> {
    unsafe { Ok(c_helpers::get_cpu_num()) }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
pub fn cpu_speed() -> Result<u64, Error> {
    unsafe { Ok(c_helpers::get_cpu_speed()) }
}

/// Get system load average value.
///
/// Notice, on windows, one/five/fifteen of the LoadAvg returned are the current load.
pub fn loadavg() -> Result<LoadAvg, Error> {
    Ok(unsafe { c_helpers::get_loadavg() })
}

/// Get current processes quantity.
pub fn proc_total() -> Result<u64, Error> {
    Ok(unsafe { c_helpers::get_proc_total() })
}

/// Get memory information.
///
/// The buffers and cached variables of the MemInfo returned are zero.
pub fn mem_info() -> Result<MemInfo, Error> {
    Ok(unsafe { c_helpers::get_mem_info() })
}

/// Get disk information.
pub fn disk_info() -> Result<DiskInfo, Error> {
    Ok(unsafe { c_helpers::get_disk_info() })
}

/// Get hostname.
pub fn hostname() -> Result<String, Error> {
    Command::new("hostname")
        .output()
        .map_err(Error::ExecFailed)
        .map(|output| String::from_utf8(output.stdout).unwrap().trim().to_string())
}
