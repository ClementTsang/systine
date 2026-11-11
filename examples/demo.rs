use systine::*;

fn main() {
    println!(
        "os: {} {}",
        os_type().expect("failed to get OS type"),
        os_release().expect("failed to get OS release")
    );
    println!(
        "cpu: {} cores, {} MHz",
        cpu_num().expect("failed to get CPU count"),
        cpu_speed().expect("failed to get CPU speed")
    );
    println!(
        "proc total: {}",
        proc_total().expect("failed to get process count")
    );
    let load = loadavg().expect("failed to get load average");
    println!("load: {} {} {}", load.one, load.five, load.fifteen);
    let mem = mem_info().expect("failed to get memory info");
    println!(
        "mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
        mem.total, mem.free, mem.avail, mem.buffers, mem.cached
    );
    println!(
        "swap: total {} KB, free {} KB",
        mem.swap_total, mem.swap_free
    );
    #[cfg(not(target_os = "solaris"))]
    {
        let disk = disk_info().expect("failed to get disk info");
        println!("disk: total {} KB, free {} KB", disk.total, disk.free);
    }
    println!("hostname: {}", hostname().expect("failed to get hostname"));
    #[cfg(not(target_os = "windows"))]
    {
        let t = boottime().expect("failed to get boot time");
        println!("boottime {} sec, {} usec", t.tv_sec, t.tv_usec);
    }
    #[cfg(target_os = "linux")]
    println!(
        "/etc/os-release: {:?}",
        linux_os_release().expect("failed to get OS release")
    );
}
