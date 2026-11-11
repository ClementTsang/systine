//! OpenBSD implementation.

use std::ffi;
use std::mem::size_of_val;
use std::ptr::null_mut;

use crate::c_helpers;
use crate::{DiskInfo, Error, LoadAvg, MemInfo};

static OS_CTL_KERN: libc::c_int = 1;
static OS_KERN_BOOTTIME: libc::c_int = 21;

/// Get operation system type.
pub fn os_type() -> Result<String, Error> {
    Ok("openbsd".to_string())
}

/// Get operation system release version.
pub fn os_release() -> Result<String, Error> {
    // SAFETY: `get_os_release` returns a null-terminated UTF-8 C string, or NULL on failure.
    let rp = unsafe { c_helpers::get_os_release() as *const libc::c_char };
    if rp.is_null() {
        Err(Error::Unknown)
    } else {
        // SAFETY: `rp` is a valid null-terminated C string returned by `get_os_release`.
        Ok(unsafe {
            let typ = ffi::CStr::from_ptr(rp).to_bytes();
            String::from_utf8_lossy(typ).into_owned()
        })
    }
}

/// Get cpu num quantity.
///
/// Notice, it returns the logical cpu quantity.
pub fn cpu_num() -> Result<u32, Error> {
    // SAFETY: `libc::sysconf` returns the number of online processors.
    // The return value is validated (>= 1, <= u32::MAX) below before use.
    let ret = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
    if ret < 1 || ret as i64 > u32::MAX as i64 {
        Err(Error::IO(std::io::Error::last_os_error()))
    } else {
        Ok(ret as u32)
    }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
pub fn cpu_speed() -> Result<u64, Error> {
    // SAFETY: `get_cpu_speed` is a C helper compiled via build.rs that returns
    // the CPU speed in MHz, or 0 on failure.
    let res: u64 = unsafe { c_helpers::get_cpu_speed() };
    match res {
        0 => Err(Error::IO(std::io::Error::last_os_error())),
        _ => Ok(res),
    }
}

/// Get system load average value.
pub fn loadavg() -> Result<LoadAvg, Error> {
    let mut l: [f64; 3] = [0f64; 3];
    // SAFETY: `l` has exactly 3 elements and `getloadavg` writes at most 3 f64s
    // into it. Returns < 3 on failure, which is handled below.
    if unsafe { libc::getloadavg(l.as_mut_ptr(), l.len() as libc::c_int) } < 3 {
        Err(Error::Unknown)
    } else {
        Ok(LoadAvg {
            one: l[0],
            five: l[1],
            fifteen: l[2],
        })
    }
}

/// Get current processes quantity.
pub fn proc_total() -> Result<u64, Error> {
    // SAFETY: `get_proc_total` is a C helper compiled via build.rs that returns
    // the current process count, or 0 on failure.
    let res: u64 = unsafe { c_helpers::get_proc_total() };
    match res {
        0 => Err(Error::IO(std::io::Error::last_os_error())),
        _ => Ok(res),
    }
}

/// Get memory information.
pub fn mem_info() -> Result<MemInfo, Error> {
    let mut mi: MemInfo = MemInfo {
        total: 0,
        free: 0,
        avail: 0,
        buffers: 0,
        cached: 0,
        swap_total: 0,
        swap_free: 0,
    };
    // SAFETY: `get_mem_info_bsd` is a C helper compiled via build.rs that writes
    // memory information into the provided `MemInfo` struct. Returns 0 on success,
    // -1 on failure.
    let res: i32 = unsafe { c_helpers::get_mem_info_bsd(&mut mi) };
    match res {
        -1 => Err(Error::IO(std::io::Error::last_os_error())),
        0 => Ok(mi),
        _ => Err(Error::Unknown),
    }
}

/// Get disk information.
pub fn disk_info() -> Result<DiskInfo, Error> {
    let mut di: DiskInfo = DiskInfo { total: 0, free: 0 };
    // SAFETY: `get_disk_info_bsd` is a C helper compiled via build.rs that writes
    // disk information into the provided `DiskInfo` struct. Returns 0 on success,
    // -1 on failure.
    let res: i32 = unsafe { c_helpers::get_disk_info_bsd(&mut di) };
    match res {
        -1 => Err(Error::IO(std::io::Error::last_os_error())),
        0 => Ok(di),
        _ => Err(Error::Unknown),
    }
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
    ffi::CString::new(buf)
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

    let mib = [OS_CTL_KERN, OS_KERN_BOOTTIME];
    let mut size: libc::size_t = size_of_val(&bt) as libc::size_t;
    let ret = unsafe {
        // SAFETY: `mib` is a valid 2-element array. `bt` is a valid `timeval`
        // passed by reference. `sysctl` writes at most `size` bytes into `bt`.
        libc::sysctl(
            mib.as_ptr(),
            2,
            &mut bt as *mut libc::timeval as *mut libc::c_void,
            &mut size,
            null_mut(),
            0,
        )
    };
    if ret == -1 {
        Err(Error::IO(std::io::Error::last_os_error()))
    } else {
        Ok(bt)
    }
}
