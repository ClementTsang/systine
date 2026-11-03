//! Solaris implementation.

use std::time::SystemTime;

use crate::c_helpers;
use crate::{DiskInfo, Error, LoadAvg, MemInfo};

#[path = "kstat.rs"]
mod kstat;

/// Get operation system type.
pub fn os_type() -> Result<String, Error> {
    Ok("solaris".to_string())
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
    Ok(kstat::cpu_mhz()?)
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
    Ok(kstat::nproc()?)
}

fn pagesize() -> Result<u32, Error> {
    let ret = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if ret < 1 || ret as i64 > u32::MAX as i64 {
        Err(Error::Unknown)
    } else {
        Ok(ret as u32)
    }
}

/// Get memory information.
pub fn mem_info() -> Result<MemInfo, Error> {
    let pagesize = pagesize()? as u64;
    let pages = kstat::pages()?;
    Ok(MemInfo {
        total: pages.physmem * pagesize / 1024,
        avail: 0,
        free: pages.freemem * pagesize / 1024,
        cached: 0,
        buffers: 0,
        swap_total: 0,
        swap_free: 0,
    })
}

/// Get disk information.
///
/// Not supported on Solaris.
pub fn disk_info() -> Result<DiskInfo, Error> {
    Err(Error::UnsupportedSystem)
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

    let start = kstat::boot_time()?;
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let now = now.as_secs();
    if now < start {
        return Err(Error::General("time went backwards".into()));
    }
    bt.tv_sec = (now - start) as i64;
    Ok(bt)
}
