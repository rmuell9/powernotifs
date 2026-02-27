use std::io;

use crate::bat;
use crate::when;

use std::process::Command;
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


pub fn start() -> battery::Result<()> {

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

    let monitor = bat::setup_udev_monitor()
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
    } else if prev_percent <= when::CRITICAL_LEVEL {
        let _ = notify(
            "Battery Critical",
            &message,
            "critical",
        );
    } else if prev_percent <= when::WARNING_LEVEL {
        let _ = notify(
            "Battery Warning",
            &message,
            "normal",
        );
    } else {
        let _ =notify(
            "Battery",
            &message,
            "normal",
        );
    }

    loop {
        let _ = bat::wait_for_power_event(&monitor);
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
                if current_percent <= when::CRITICAL_LEVEL {
                    let _ = notify(
                        "Battery Unplugged \
                         - CRITICAL",
                        &message,
                        "critical",
                    );
                } else if current_percent
                    <= when::WARNING_LEVEL
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
            if current_percent == when::WARNING_LEVEL {
                let _ = notify(
                    "Battery Warning",
                    &message,
                    "normal",
                );
            } else if current_percent
                <= when::CRITICAL_LEVEL
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

