//! FreeBSD implementation.

use std::ffi;
use std::mem::size_of_val;
use std::ptr::null_mut;

use crate::c_helpers;
use crate::{DiskInfo, Error, LoadAvg, MemInfo};

static OS_CTL_KERN: libc::c_int = 1;
static OS_KERN_BOOTTIME: libc::c_int = 21;

/// Get operation system type.
pub fn os_type() -> Result<String, Error> {
    Ok("freebsd".to_string())
}

/// Get operation system release version.
pub fn os_release() -> Result<String, Error> {
    let rp = unsafe { c_helpers::get_os_release() as *const libc::c_char };
    if rp.is_null() {
        Err(Error::Unknown)
    } else {
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
    let res: u64 = unsafe { c_helpers::get_cpu_speed() };
    match res {
        0 => Err(Error::IO(std::io::Error::last_os_error())),
        _ => Ok(res),
    }
}

/// Get system load average value.
pub fn loadavg() -> Result<LoadAvg, Error> {
    let mut l: [f64; 3] = [0f64; 3];
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
    let res: i32 = unsafe { c_helpers::get_disk_info_bsd(&mut di) };
    match res {
        -1 => Err(Error::IO(std::io::Error::last_os_error())),
        0 => Ok(di),
        _ => Err(Error::Unknown),
    }
}

/// Get hostname.
pub fn hostname() -> Result<String, Error> {
    let buf_size = unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) as usize };
    let mut buf = Vec::<u8>::with_capacity(buf_size + 1);
    let ret = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf_size) };
    if ret < 0 {
        return Err(Error::IO(std::io::Error::last_os_error()));
    }
    let hostname_len = unsafe { libc::strnlen(buf.as_ptr() as *const libc::c_char, buf_size) };
    buf.set_len(hostname_len);
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
