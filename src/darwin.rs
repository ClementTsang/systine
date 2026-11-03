//! Darwin (macOS/iOS) implementation.

use std::ffi;
use std::mem::size_of_val;
use std::ptr::null_mut;

use crate::c_helpers;
use crate::{DiskInfo, Error, MemInfo};

static OS_CTL_KERN: libc::c_int = 1;
static OS_KERN_BOOTTIME: libc::c_int = 21;

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
    unsafe { Ok(c_helpers::get_cpu_num()) }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
#[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
pub fn cpu_speed() -> Result<u64, Error> {
    unsafe { Ok(c_helpers::get_cpu_speed()) }
}

/// Get system load average value.
pub fn loadavg() -> Result<crate::LoadAvg, Error> {
    let mut l: [f64; 3] = [0f64; 3];
    if unsafe { libc::getloadavg(l.as_mut_ptr(), l.len() as libc::c_int) } < 3 {
        Err(Error::Unknown)
    } else {
        Ok(crate::LoadAvg {
            one: l[0],
            five: l[1],
            fifteen: l[2],
        })
    }
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
    unsafe {
        let buf_size = libc::sysconf(libc::_SC_HOST_NAME_MAX) as usize;
        let mut buf = Vec::<u8>::with_capacity(buf_size + 1);
        if libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf_size) < 0 {
            return Err(Error::IO(std::io::Error::last_os_error()));
        }
        let hostname_len = libc::strnlen(buf.as_ptr() as *const libc::c_char, buf_size);
        buf.set_len(hostname_len);
        let cstr = ffi::CString::new(buf).map_err(Error::ExecFailed)?;
        cstr.into_string().map_err(Error::ExecFailed)
    }
}

/// Get system boottime.
pub fn boottime() -> Result<libc::timeval, Error> {
    let mut bt = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    let mut mib = [OS_CTL_KERN, OS_KERN_BOOTTIME];
    let mut size: libc::size_t = size_of_val(&bt) as libc::size_t;
    unsafe {
        if libc::sysctl(
            &mut mib[0],
            2,
            &mut bt as *mut libc::timeval as *mut libc::c_void,
            &mut size,
            null_mut(),
            0,
        ) == -1
        {
            Err(Error::IO(std::io::Error::last_os_error()))
        } else {
            Ok(bt)
        }
    }
}
