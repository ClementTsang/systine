//! Windows implementation.

use std::ffi;
use std::process::Command;

use crate::c_helpers;
use crate::{DiskInfo, Error, LoadAvg, MemInfo};

/// Get operation system type.
pub fn os_type() -> Result<String, Error> {
    // SAFETY: `get_os_type` returns a null-terminated UTF-8 C string.
    let typ =
        unsafe { ffi::CStr::from_ptr(c_helpers::get_os_type() as *const libc::c_char).to_bytes() };
    Ok(String::from_utf8_lossy(typ).into_owned())
}

/// Get operation system release version.
pub fn os_release() -> Result<String, Error> {
    // SAFETY: `get_os_release` returns a null-terminated UTF-8 C string, or NULL on failure.
    unsafe {
        let rp = c_helpers::get_os_release() as *const libc::c_char;
        if rp.is_null() {
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
    // SAFETY: `get_cpu_num` is a C helper compiled via build.rs that returns the
    // logical CPU count, validated to be >= 1.
    unsafe { Ok(c_helpers::get_cpu_num()) }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
pub fn cpu_speed() -> Result<u64, Error> {
    // SAFETY: `get_cpu_speed` is a C helper compiled via build.rs that returns
    // the CPU speed in MHz, or 0 on failure.
    unsafe { Ok(c_helpers::get_cpu_speed()) }
}

/// Get system load average value.
///
/// Notice, on windows, one/five/fifteen of the LoadAvg returned are the current load.
pub fn loadavg() -> Result<LoadAvg, Error> {
    // SAFETY: `get_loadavg` is a C helper compiled via build.rs that returns
    // a valid `LoadAvg` struct.
    Ok(unsafe { c_helpers::get_loadavg() })
}

/// Get current processes quantity.
pub fn proc_total() -> Result<u64, Error> {
    // SAFETY: `get_proc_total` is a C helper compiled via build.rs that returns
    // the current process count, or 0 on failure.
    Ok(unsafe { c_helpers::get_proc_total() })
}

/// Get memory information.
///
/// The buffers and cached variables of the MemInfo returned are zero.
pub fn mem_info() -> Result<MemInfo, Error> {
    // SAFETY: `get_mem_info` is a C helper compiled via build.rs that returns
    // a valid `MemInfo` struct.
    Ok(unsafe { c_helpers::get_mem_info() })
}

/// Get disk information.
pub fn disk_info() -> Result<DiskInfo, Error> {
    // SAFETY: `get_disk_info` is a C helper compiled via build.rs that returns
    // a valid `DiskInfo` struct.
    Ok(unsafe { c_helpers::get_disk_info() })
}

/// Get hostname.
pub fn hostname() -> Result<String, Error> {
    Command::new("hostname")
        .output()
        .map_err(Error::ExecFailed)
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .map_err(|e| Error::General(e.to_string()))
                .map(|s| s.trim().to_string())
        })
}
