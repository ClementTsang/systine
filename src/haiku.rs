//! Haiku implementation.

use crate::c_helpers;
use crate::{DiskInfo, Error, MemInfo};

/// Get operation system type.
pub fn os_type() -> Result<String, Error> {
    Ok("haiku".to_string())
}

/// Get operation system release version.
pub fn os_release() -> Result<String, Error> {
    let release: Option<String> = unsafe {
        let mut name: libc::utsname = std::mem::zeroed();
        if libc::uname(&mut name) < 0 {
            None
        } else {
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
    unsafe { Ok(c_helpers::get_cpu_num()) }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
pub fn cpu_speed() -> Result<u64, Error> {
    unsafe { Ok(c_helpers::get_cpu_speed()) }
}

/// Get current processes quantity.
pub fn proc_total() -> Result<u64, Error> {
    Ok(unsafe { c_helpers::get_proc_total() })
}

/// Get memory information.
pub fn mem_info() -> Result<MemInfo, Error> {
    Ok(unsafe { c_helpers::get_mem_info() })
}

/// Get disk information.
pub fn disk_info() -> Result<DiskInfo, Error> {
    Ok(unsafe { c_helpers::get_disk_info() })
}

/// Get hostname.
pub fn hostname() -> Result<String, Error> {
    unsafe {
        let buf_size = libc::sysconf(libc::_SC_HOST_NAME_MAX) as usize;
        let mut buf = Vec::<u8>::with_capacity(buf_size + 1);
        if libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf_size) < 0 {
            return Err(Error::IO(std::io::Error::last_os_error()));
        }
        let hostname_len = libc::strnlen(buf.as_ptr() as *const libc::c_char, buf_size);
        buf.set_len(hostname_len);
        Ok(std::ffi::CString::new(buf).unwrap().into_string().unwrap())
    }
}

/// Get system boottime.
pub fn boottime() -> Result<libc::timeval, Error> {
    let mut bt = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

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
