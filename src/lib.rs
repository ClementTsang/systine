//! # Introduction
//! This crate focuses on getting system information.
//!
//! It supports Linux, macOS (Darwin), Windows, FreeBSD, OpenBSD, NetBSD,
//! Solaris, Illumos, and Haiku.

use std::fmt;
use std::fs::File;
use std::io::{self, Read};

mod c_helpers;
#[cfg(target_vendor = "apple")]
mod darwin;
#[cfg(target_os = "freebsd")]
mod freebsd;
#[cfg(target_os = "haiku")]
mod haiku;
#[cfg(target_os = "illumos")]
mod illumos;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "netbsd")]
mod netbsd;
#[cfg(target_os = "openbsd")]
mod openbsd;
#[cfg(target_os = "solaris")]
mod solaris;
#[cfg(target_os = "windows")]
mod windows;

// Re-export platform functions at crate root.

#[cfg(target_vendor = "apple")]
pub use darwin::{
    boottime, cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type,
    proc_total,
};
#[cfg(target_os = "freebsd")]
pub use freebsd::{
    boottime, cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type,
    proc_total,
};
#[cfg(target_os = "haiku")]
pub use haiku::{
    boottime, cpu_num, cpu_speed, disk_info, hostname, mem_info, os_release, os_type, proc_total,
};
#[cfg(target_os = "illumos")]
pub use illumos::{
    boottime, cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type,
    proc_total,
};
#[cfg(target_os = "linux")]
pub use linux::{
    boottime, cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type,
    proc_total,
};
#[cfg(target_os = "netbsd")]
pub use netbsd::{
    boottime, cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type,
    proc_total,
};
#[cfg(target_os = "openbsd")]
pub use openbsd::{
    boottime, cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type,
    proc_total,
};
#[cfg(target_os = "solaris")]
pub use solaris::{
    boottime, cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type,
    proc_total,
};
#[cfg(target_os = "windows")]
pub use windows::{
    cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type, proc_total,
};

/// System load average value.
#[repr(C)]
#[derive(Debug)]
pub struct LoadAvg {
    /// Average load within one minutes.
    pub one: f64,
    /// Average load within five minutes.
    pub five: f64,
    /// Average load within fifteen minutes.
    pub fifteen: f64,
}

/// System memory information.
#[repr(C)]
#[derive(Debug)]
pub struct MemInfo {
    /// Total physical memory.
    pub total: u64,
    pub free: u64,
    pub avail: u64,

    pub buffers: u64,
    pub cached: u64,

    /// Total swap memory.
    pub swap_total: u64,
    pub swap_free: u64,
}

/// The os release info of Linux.
///
/// See [man os-release](https://www.freedesktop.org/software/systemd/man/os-release.html).
#[derive(Debug, Default)]
pub struct LinuxOSReleaseInfo {
    /// A lower-case string (no spaces or other characters outside of 0–9, a–z, ".", "_" and "-")
    /// identifying the operating system, excluding any version information and suitable for
    /// processing by scripts or usage in generated filenames.
    pub id: Option<String>,
    /// A space-separated list of operating system identifiers.
    pub id_like: Option<String>,
    /// A string identifying the operating system, without a version component.
    pub name: Option<String>,
    /// A pretty operating system name suitable for presentation to the user.
    pub pretty_name: Option<String>,
    /// A string identifying the operating system version.
    pub version: Option<String>,
    /// A lower-case string identifying the operating system version.
    pub version_id: Option<String>,
    /// A lower-case string identifying the operating system release code name.
    pub version_codename: Option<String>,
    /// A suggested presentation color for the operating system name.
    pub ansi_color: Option<String>,
    /// An icon name for the operating system.
    pub logo: Option<String>,
    /// A CPE name for the operating system.
    pub cpe_name: Option<String>,
    /// A string uniquely identifying the system image.
    pub build_id: Option<String>,
    /// A string identifying a specific variant or edition.
    pub variant: Option<String>,
    /// A lower-case string identifying a specific variant or edition.
    pub variant_id: Option<String>,
    /// The homepage URL.
    pub home_url: Option<String>,
    /// The documentation URL.
    pub documentation_url: Option<String>,
    /// The support URL.
    pub support_url: Option<String>,
    /// The bug report URL.
    pub bug_report_url: Option<String>,
    /// The privacy policy URL.
    pub privacy_policy_url: Option<String>,
}

macro_rules! os_release_defaults {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis fn $field:ident => $default:literal
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis fn $field(&self) -> &str {
                match self.$field.as_ref() {
                    Some(value) => value,
                    None => $default,
                }
            }
        )*
    }
}

impl LinuxOSReleaseInfo {
    os_release_defaults!(
        /// Returns the value of `self.id` or, if `None`, "linux" (the default value).
        pub fn id => "linux"
        /// Returns the value of `self.name` or, if `None`, "Linux" (the default value).
        pub fn name => "Linux"
        /// Returns the value of `self.pretty_name` or, if `None`, "Linux" (the default value).
        pub fn pretty_name => "Linux"
    );
}

/// Disk information.
#[repr(C)]
#[derive(Debug)]
pub struct DiskInfo {
    pub total: u64,
    pub free: u64,
}

/// Error types.
#[derive(Debug)]
pub enum Error {
    UnsupportedSystem,
    ExecFailed(io::Error),
    IO(io::Error),
    SystemTime(std::time::SystemTimeError),
    General(String),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::UnsupportedSystem => write!(f, "System is not supported"),
            Error::ExecFailed(ref e) => write!(f, "Execution failed: {}", e),
            Error::IO(ref e) => write!(f, "IO error: {}", e),
            Error::SystemTime(ref e) => write!(f, "System time error: {}", e),
            Error::General(ref e) => write!(f, "Error: {}", e),
            Error::Unknown => write!(f, "An unknown error occurred"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnsupportedSystem => "unsupported system",
            Error::ExecFailed(_) => "execution failed",
            Error::IO(_) => "io error",
            Error::SystemTime(_) => "system time",
            Error::General(_) => "general error",
            Error::Unknown => "unknown error",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::UnsupportedSystem => None,
            Error::ExecFailed(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            Error::SystemTime(ref e) => Some(e),
            Error::General(_) => None,
            Error::Unknown => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(e: std::time::SystemTimeError) -> Error {
        Error::SystemTime(e)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Error {
        Error::General(e.to_string())
    }
}

/// Get the os release note of Linux.
///
/// Information in /etc/os-release, such as name and version of distribution.
///
/// See `LinuxOSReleaseInfo` for more documentation.
pub fn linux_os_release() -> Result<LinuxOSReleaseInfo, Error> {
    if !cfg!(target_os = "linux") {
        return Err(Error::UnsupportedSystem);
    }

    let mut s = String::new();
    File::open("/etc/os-release")?.read_to_string(&mut s)?;

    let mut info: LinuxOSReleaseInfo = Default::default();
    for l in s.split('\n') {
        if let Some((key, value)) = parse_line_for_linux_os_release(l.trim().to_string()) {
            match (key.as_ref(), value) {
                ("ID", val) => info.id = Some(val),
                ("ID_LIKE", val) => info.id_like = Some(val),
                ("NAME", val) => info.name = Some(val),
                ("PRETTY_NAME", val) => info.pretty_name = Some(val),
                ("VERSION", val) => info.version = Some(val),
                ("VERSION_ID", val) => info.version_id = Some(val),
                ("VERSION_CODENAME", val) => info.version_codename = Some(val),
                ("ANSI_COLOR", val) => info.ansi_color = Some(val),
                ("LOGO", val) => info.logo = Some(val),
                ("CPE_NAME", val) => info.cpe_name = Some(val),
                ("BUILD_ID", val) => info.build_id = Some(val),
                ("VARIANT", val) => info.variant = Some(val),
                ("VARIANT_ID", val) => info.variant_id = Some(val),
                ("HOME_URL", val) => info.home_url = Some(val),
                ("BUG_REPORT_URL", val) => info.bug_report_url = Some(val),
                ("SUPPORT_URL", val) => info.support_url = Some(val),
                ("DOCUMENTATION_URL", val) => info.documentation_url = Some(val),
                ("PRIVACY_POLICY_URL", val) => info.privacy_policy_url = Some(val),
                _ => {}
            }
        }
    }

    Ok(info)
}

fn parse_line_for_linux_os_release(l: String) -> Option<(String, String)> {
    let words: Vec<&str> = l.splitn(2, '=').collect();
    if words.len() < 2 {
        return None;
    }
    let mut trim_value = String::from(words[1]);

    if trim_value.starts_with('"') {
        trim_value.remove(0);
    }
    if trim_value.ends_with('"') {
        let len = trim_value.len();
        trim_value.remove(len - 1);
    }

    Some((String::from(words[0]), trim_value))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::len_zero)]
mod test {
    use crate::{
        boottime, cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type,
        proc_total,
    };

    #[test]
    pub fn test_os_type() {
        let typ = os_type().unwrap();
        assert!(!typ.is_empty());
        println!("os_type(): {}", typ);
    }

    #[test]
    pub fn test_os_release() {
        let release = os_release().unwrap();
        assert!(!release.is_empty());
        println!("os_release(): {}", release);
    }

    #[test]
    pub fn test_cpu_num() {
        let num = cpu_num().unwrap();
        assert!(num > 0);
        println!("cpu_num(): {}", num);
    }

    #[test]
    #[cfg(not(all(target_vendor = "apple", target_arch = "aarch64")))]
    pub fn test_cpu_speed() {
        let speed = cpu_speed().unwrap();
        assert!(speed > 0);
        println!("cpu_speed(): {}", speed);
    }

    #[test]
    pub fn test_loadavg() {
        let load = loadavg().unwrap();
        println!("loadavg(): {:?}", load);
    }

    #[test]
    pub fn test_proc_total() {
        let procs = proc_total().unwrap();
        assert!(procs > 0);
        println!("proc_total(): {}", procs);
    }

    #[test]
    pub fn test_mem_info() {
        let mem = mem_info().unwrap();
        assert!(mem.total > 0);
        println!("mem_info(): {:?}", mem);
    }

    #[test]
    #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
    pub fn test_disk_info() {
        let info = disk_info().unwrap();
        println!("disk_info(): {:?}", info);
    }

    #[test]
    pub fn test_hostname() {
        let host = hostname().unwrap();
        assert!(!host.is_empty());
        println!("hostname(): {}", host);
    }

    #[test]
    #[cfg(not(windows))]
    pub fn test_boottime() {
        let bt = boottime().unwrap();
        println!("boottime(): {} {}", bt.tv_sec, bt.tv_usec);
        assert!(bt.tv_sec > 0 || bt.tv_usec > 0);
    }

    #[test]
    #[cfg(target_os = "linux")]
    pub fn test_linux_os_release() {
        let os_release = crate::linux_os_release().unwrap();
        println!("linux_os_release(): {:?}", os_release.name)
    }
}
