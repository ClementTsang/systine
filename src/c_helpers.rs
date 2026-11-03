// SAFETY: This is a C binding, it's inherently unsafe.
// These declarations match the C functions compiled via build.rs.
unsafe extern "C" {
    #[cfg(any(target_vendor = "apple", target_os = "windows"))]
    pub fn get_os_type() -> *const i8;

    #[cfg(any(
        target_vendor = "apple",
        target_os = "windows",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    pub fn get_os_release() -> *const i8;

    #[cfg(all(
        not(any(
            target_os = "solaris",
            target_os = "illumos",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        )),
        any(unix, windows)
    ))]
    #[allow(dead_code)]
    pub fn get_cpu_num() -> u32;

    #[cfg(any(
        all(
            target_vendor = "apple",
            not(any(target_arch = "aarch64", target_arch = "arm"))
        ),
        target_os = "windows",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "haiku"
    ))]
    pub fn get_cpu_speed() -> u64;

    #[cfg(target_os = "windows")]
    pub fn get_loadavg() -> super::LoadAvg;

    #[cfg(any(
        target_vendor = "apple",
        target_os = "windows",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "haiku"
    ))]
    pub fn get_proc_total() -> u64;

    #[cfg(any(target_vendor = "apple", target_os = "windows", target_os = "haiku"))]
    pub fn get_mem_info() -> super::MemInfo;

    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    pub fn get_mem_info_bsd(mi: &mut super::MemInfo) -> i32;

    #[cfg(any(
        target_os = "linux",
        target_vendor = "apple",
        target_os = "windows",
        target_os = "haiku"
    ))]
    pub fn get_disk_info() -> super::DiskInfo;

    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    pub fn get_disk_info_bsd(di: &mut super::DiskInfo) -> i32;
}
