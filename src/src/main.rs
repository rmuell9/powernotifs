use std::io::{self, Read};
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::process::Command;
use std::env;

extern crate battery;
extern crate libc;

use battery::units::ratio::percent;
use battery::State;

fn notify(
    title: &str,
    message: &str,
    urgency: &str,
) -> Result<(), std::io::Error> {
    Command::new("notify-send")
        .args(["-u", urgency, title, message])
        .spawn()?;
    Ok(())
}

fn setup_udev_monitor() -> io::Result<File> {
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

fn wait_for_power_event(monitor: &File) -> io::Result<()> {
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

const WARNING_LEVEL: u8 = 20;
const CRITICAL_LEVEL: u8 = 10;

fn warning() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    let battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(
                io::Error::from(
                    io::ErrorKind::NotFound
                ).into()
            );
        }
    };
    let current_percent = battery
        .state_of_charge()
        .get::<percent>()
        .floor() as u8;
    if current_percent <= CRITICAL_LEVEL {
        println!("󰁺");
    } else if current_percent <= WARNING_LEVEL {
        println!("󰁼");
    }
    Ok(())
}

fn usage() {
    eprintln!("Usage: powernotifs <command>");
    eprintln!();
    eprintln!("Commands:");
    eprintln!(
        "  start    Start monitoring battery events"
    );
}

fn start() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next()
    {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!(
                "Unable to access battery information"
            );
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(
                io::Error::from(
                    io::ErrorKind::NotFound
                ).into(),
            );
        }
    };

    let monitor = setup_udev_monitor()
        .expect("Failed to setup udev monitor");

    let mut prev_plugged =
        battery.state() != State::Discharging;
    let mut prev_percent = battery
        .state_of_charge()
        .get::<percent>()
        .floor() as u8;

    let message =
        format!("Current level: {}%", prev_percent);
    if prev_plugged {
        if prev_percent == 100 {
            let _ = notify(
                "Battery",
                "Plugged In: battery full",
                "normal",
            );
        } else {
            let _ = notify(
                "Battery Charging",
                &message,
                "normal",
            );
        }
    } else if prev_percent <= CRITICAL_LEVEL {
        let _ = notify(
            "Battery Critical",
            &message,
            "critical",
        );
    } else if prev_percent <= WARNING_LEVEL {
        let _ = notify(
            "Battery Warning",
            &message,
            "normal",
        );
    }

    loop {
        let _ = wait_for_power_event(&monitor);
        manager.refresh(&mut battery)?;

        let plugged =
            battery.state() != State::Discharging;
        let current_percent = battery
            .state_of_charge()
            .get::<percent>()
            .floor() as u8;
        let message = format!(
            "Current level: {}%",
            current_percent
        );

        if plugged != prev_plugged {
            if plugged {
                if current_percent == 100 {
                    let _ = notify(
                        "Battery",
                        "Plugged In: battery full",
                        "normal",
                    );
                } else {
                    let _ = notify(
                        "Battery Plugged In",
                        &format!(
                            "Charging: {}%",
                            current_percent
                        ),
                        "normal",
                    );
                }
            } else {
                if current_percent <= CRITICAL_LEVEL {
                    let _ = notify(
                        "Battery Unplugged \
                         - CRITICAL",
                        &message,
                        "critical",
                    );
                } else if current_percent
                    <= WARNING_LEVEL
                {
                    let _ = notify(
                        "Battery Unplugged \
                         - WARNING",
                        &message,
                        "normal",
                    );
                } else {
                    let _ = notify(
                        "Battery Unplugged",
                        &message,
                        "normal",
                    );
                }
            }
            prev_plugged = plugged;
        }

        if current_percent != prev_percent && !plugged
        {
            if current_percent == WARNING_LEVEL {
                let _ = notify(
                    "Battery Warning",
                    &message,
                    "normal",
                );
            } else if current_percent
                <= CRITICAL_LEVEL
            {
                let _ = notify(
                    "Battery Critical",
                    &message,
                    "critical",
                );
            }
            prev_percent = current_percent;
        } else {
            prev_percent = current_percent;
        }
    }
}

fn main() -> battery::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "start" => start(),
        "warning" => warning(),
        _ => {
            usage();
            std::process::exit(1);
        }
    }
}
