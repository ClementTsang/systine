//! Haiku implementation.

use crate::c_helpers;
use crate::{DiskInfo, Error, MemInfo};

/// Get operation system type.
pub fn os_type() -> Result<String, Error> {
    Ok("haiku".to_string())
}

/// Get operation system release version.
pub fn os_release() -> Result<String, Error> {
    // SAFETY: `libc::uname` writes a zeroed `utsname` struct on success.
    // `name.release` is a fixed-size C string (typically 65 bytes) that
    // is null-terminated by `uname`.
    let release: Option<String> = unsafe {
        let mut name: libc::utsname = std::mem::zeroed();
        if libc::uname(&mut name) < 0 {
            None
        } else {
            // SAFETY: `name.release` is a valid C string (<= 65 bytes) written
            // by `uname`. It is null-terminated and contains ASCII.
            let cstr = std::ffi::CStr::from_ptr(name.release.as_mut_ptr());
            Some(cstr.to_string_lossy().to_string())
        }
    };
    match release {
        None => Err(Error::Unknown),
        Some(release) => Ok(release),
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

/// Get current processes quantity.
pub fn proc_total() -> Result<u64, Error> {
    // SAFETY: `get_proc_total` is a C helper compiled via build.rs that returns
    // the current process count, or 0 on failure.
    Ok(unsafe { c_helpers::get_proc_total() })
}

/// Get memory information.
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
    // SAFETY: `sysconf(_SC_HOST_NAME_MAX)` provides a valid buffer size.
    // `gethostname` writes at most `buf_size` bytes into the buffer.
    // `strnlen` reads up to `buf_size` bytes, and the buffer is truncated
    // to the actual string length before converting to a CString.
    let buf_size = unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) as usize };
    let mut buf = Vec::<u8>::with_capacity(buf_size + 1);
    let ret = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf_size) };
    if ret < 0 {
        return Err(Error::IO(std::io::Error::last_os_error()));
    }
    let hostname_len = unsafe { libc::strnlen(buf.as_ptr() as *const libc::c_char, buf_size) };
    // SAFETY: `hostname_len` is validated by `strnlen` to be <= `buf_size`,
    // so setting the vec length is safe and will not cause out-of-bounds access.
    unsafe {
        buf.set_len(hostname_len);
    }
    std::ffi::CString::new(buf)
        .ok()
        .and_then(|cstr| cstr.into_string().ok())
        .ok_or(Error::Unknown)
}

/// Get system boottime.
pub fn boottime() -> Result<libc::timeval, Error> {
    let mut bt = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    // SAFETY: `get_system_info` writes a valid `system_info` struct when it returns
    // `B_OK`. The boot_time field is in microseconds. `libc::time` returns the
    // current time; subtraction is safe since we've already verified `now >= start`.
    unsafe {
        let mut sysinfo: libc::system_info = std::mem::zeroed();
        if libc::get_system_info(&mut sysinfo) == libc::B_OK {
            let mut now: libc::time_t = 0;
            libc::time(&mut now);
            bt.tv_sec = now - (sysinfo.boot_time / 1000000);
            bt.tv_usec = (sysinfo.boot_time % 1000000) as libc::suseconds_t;
            Ok(bt)
        } else {
            Err(Error::IO(std::io::Error::last_os_error()))
        }
    }
}
