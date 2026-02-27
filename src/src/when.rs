use std::io;

extern crate battery;
extern crate libc;

use battery::units::ratio::percent;


pub const WARNING_LEVEL: u8 = 20;
pub const CRITICAL_LEVEL: u8 = 10;

pub fn warning() -> battery::Result<()> {
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
