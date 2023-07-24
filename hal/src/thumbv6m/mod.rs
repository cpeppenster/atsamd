pub mod eic;

mod reset_cause;
pub use reset_cause::*;

mod serial_number;
pub use serial_number::*;

pub mod calibration;
pub mod clock;
pub mod timer;

#[cfg(feature = "unproven")]
pub mod adc;

#[cfg(all(feature = "unproven", not(feature = "samd20")))]
pub mod pwm;

#[cfg(feature = "unproven")]
pub mod watchdog;

#[cfg(all(feature = "usb", feature = "has-usb"))]
pub mod usb;
