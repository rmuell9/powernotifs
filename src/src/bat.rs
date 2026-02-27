use std::io::{self, Read};
use std::fs::File;
use std::os::unix::io::AsRawFd;
extern crate battery;
extern crate libc;

pub fn setup_udev_monitor() -> io::Result<File> {
    let socket = unsafe {
        let fd = libc::socket(
            libc::AF_NETLINK,
            libc::SOCK_DGRAM | libc::SOCK_CLOEXEC,
            15, // NETLINK_KOBJECT_UEVENT
        );
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        
        let mut addr: libc::sockaddr_nl =
            std::mem::zeroed();
        addr.nl_family = libc::AF_NETLINK as u16;
        addr.nl_groups = 1;
        addr.nl_pid = 0;
        
        let ret = libc::bind(
            fd,
            &addr as *const _ as *const libc::sockaddr,
            std::mem::size_of::<libc::sockaddr_nl>()
                as u32,
        );
        if ret < 0 {
            libc::close(fd);
            return Err(io::Error::last_os_error());
        }
        
        fd
    };
    
    use std::os::unix::io::FromRawFd;
    Ok(unsafe { File::from_raw_fd(socket) })
}

pub fn wait_for_power_event(monitor: &File) -> io::Result<()> {
    let mut pollfd = libc::pollfd {
        fd: monitor.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    
    loop {
        let ret = unsafe {
            libc::poll(&mut pollfd, 1, -1)
        };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        
        if pollfd.revents & libc::POLLIN != 0 {
            let mut buf = [0u8; 4096];
            let mut file = monitor.try_clone()?;
            let n = file.read(&mut buf)?;
            let data =
                String::from_utf8_lossy(&buf[..n]);
            if data.contains("power_supply") {
                return Ok(());
            }
        }
    }
}
