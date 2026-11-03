//! Linux implementation.

use std::collections::HashMap;
use std::ffi;
use std::fs::File;
use std::io::{self, Read};

use crate::c_helpers;
use crate::{DiskInfo, Error, LoadAvg, MemInfo};

/// Get operation system type.
pub fn os_type() -> Result<String, Error> {
    let mut s = String::new();
    File::open("/proc/sys/kernel/ostype")?.read_to_string(&mut s)?;
    s.pop(); // pop '\n'
    Ok(s)
}

/// Get operation system release version.
pub fn os_release() -> Result<String, Error> {
    let mut s = String::new();
    File::open("/proc/sys/kernel/osrelease")?.read_to_string(&mut s)?;
    s.pop(); // pop '\n'
    Ok(s)
}

/// Get cpu num quantity.
///
/// Notice, it returns the logical cpu quantity.
pub fn cpu_num() -> Result<u32, Error> {
    let ret = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
    if ret < 1 || ret as i64 > u32::MAX as i64 {
        Err(Error::IO(io::Error::last_os_error()))
    } else {
        Ok(ret as u32)
    }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
pub fn cpu_speed() -> Result<u64, Error> {
    let mut s = String::new();
    File::open("/proc/cpuinfo")?.read_to_string(&mut s)?;

    let find_cpu_mhz = s.split('\n').find(|line| {
        line.starts_with("cpu MHz\t")
            || line.starts_with("BogoMIPS")
            || line.starts_with("clock\t")
            || line.starts_with("bogomips per cpu")
    });

    find_cpu_mhz
        .and_then(|line| line.split(':').next_back())
        .and_then(|val| val.replace("MHz", "").trim().parse::<f64>().ok())
        .map(|speed| speed as u64)
        .ok_or(Error::Unknown)
}

/// Get system load average value.
pub fn loadavg() -> Result<LoadAvg, Error> {
    let mut s = String::new();
    File::open("/proc/loadavg")?.read_to_string(&mut s)?;
    let loads: Vec<f64> = s
        .trim()
        .split(' ')
        .take(3)
        .map(|val| {
            val.parse::<f64>()
                .expect("loadavg should be valid f64 values")
        })
        .collect();
    Ok(LoadAvg {
        one: loads[0],
        five: loads[1],
        fifteen: loads[2],
    })
}

/// Get current processes quantity.
pub fn proc_total() -> Result<u64, Error> {
    let mut s = String::new();
    File::open("/proc/loadavg")?.read_to_string(&mut s)?;
    s.split(' ')
        .nth(3)
        .and_then(|val| val.split('/').next_back())
        .and_then(|val| val.parse::<u64>().ok())
        .ok_or(Error::Unknown)
}

/// Get memory information.
pub fn mem_info() -> Result<MemInfo, Error> {
    let mut s = String::new();
    File::open("/proc/meminfo")?.read_to_string(&mut s)?;
    let mut meminfo_hashmap = HashMap::new();
    for line in s.lines() {
        let mut split_line = line.split_whitespace();
        let label = split_line.next();
        let value = split_line.next();
        if let (Some(label), Some(value)) = (label, value) {
            let label = label.split(':').next().ok_or(Error::Unknown)?;
            let value = value.parse::<u64>().ok().ok_or(Error::Unknown)?;
            meminfo_hashmap.insert(label, value);
        }
    }
    let total = *meminfo_hashmap.get("MemTotal").ok_or(Error::Unknown)?;
    let free = *meminfo_hashmap.get("MemFree").ok_or(Error::Unknown)?;
    let buffers = *meminfo_hashmap.get("Buffers").ok_or(Error::Unknown)?;
    let cached = *meminfo_hashmap.get("Cached").ok_or(Error::Unknown)?;
    let avail = meminfo_hashmap
        .get("MemAvailable")
        .copied()
        .or_else(|| {
            let sreclaimable = *meminfo_hashmap.get("SReclaimable")?;
            let shmem = *meminfo_hashmap.get("Shmem")?;
            Some(free + buffers + cached + sreclaimable - shmem)
        })
        .ok_or(Error::Unknown)?;
    let swap_total = *meminfo_hashmap.get("SwapTotal").ok_or(Error::Unknown)?;
    let swap_free = *meminfo_hashmap.get("SwapFree").ok_or(Error::Unknown)?;
    Ok(MemInfo {
        total,
        free,
        avail,
        buffers,
        cached,
        swap_total,
        swap_free,
    })
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
            return Err(Error::IO(io::Error::last_os_error()));
        }
        let hostname_len = libc::strnlen(buf.as_ptr() as *const libc::c_char, buf_size);
        buf.set_len(hostname_len);
        Ok(ffi::CString::new(buf)
            .expect("hostname contains invalid data")
            .into_string()
            .expect("hostname is not valid UTF-8"))
    }
}

/// Get system boottime.
pub fn boottime() -> Result<libc::timeval, Error> {
    let mut bt = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    let mut s = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut s)?;
    let secs: Vec<f64> = s
        .trim()
        .split(' ')
        .take(2)
        .map(|val| {
            val.parse::<f64>()
                .expect("uptime should be valid f64 values")
        })
        .collect();
    #[cfg_attr(
        any(target_env = "musl"),
        expect(
            deprecated,
            reason = "libc::time_t and libc::suseconds_t are deprecated in libc 0.2 because musl 1.2.0 changes them to 64-bit; libc::timeval fields must remain compatible with the C struct layout"
        )
    )]
    {
        bt.tv_sec = secs[0] as libc::time_t;
        bt.tv_usec = secs[1] as libc::suseconds_t;
    }
    Ok(bt)
}
