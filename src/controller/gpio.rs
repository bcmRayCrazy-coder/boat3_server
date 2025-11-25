use crate::protocol::gpio::{GPIOMode, RemoteGPIO};
use rppal::gpio::{self, Gpio};
use std::collections::HashMap;

pub enum GPIOError {
    ArgError,
    PinNotFound(u32),
    PinExists(u32),
    PinUnmatch(u32, GPIOMode),
}

impl std::fmt::Display for GPIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GPIOError::ArgError => "Argument Error",
                GPIOError::PinNotFound(pin) => format!("PinNotFound: No pin {}.", pin.to_string()),
                GPIOError::PinExists(pin) =>
                    format!("PinExists: Already exists pin {}.", pin.to_string()),
                GPIOError::PinUnmatch(pin, mode) => format!(
                    "PinUnmatch: Pin {} not matched to mode {}.",
                    pin.to_string(),
                    mode
                ),
            }
        )
    }
}

struct GPIOList {
    input: HashMap<u32, gpio::InputPin>,
    output: HashMap<u32, gpio::OutputPin>,
}

static GPIO: Gpio = Gpio::new().unwrap();
static GPIO_LIST: GPIOList = GPIOList {
    input: HashMap::new(),
    output: HashMap::new(),
};

pub fn config(gpio: RemoteGPIO) -> Result<_, GPIOError> {
    let pin = GPIO.get(gpio.pin);
    match pin {
        Err(_) => Err(GPIOError::PinNotFound(gpio.pin)),
        Ok(pin) => match gpio.mode {
            GPIOMode::UNKNOWN => Err(GPIOError::ArgError),
            GPIOMode::INPUT => {
                if GPIO_LIST.input.contains_key(&gpio.pin) {
                    return Err(GPIOError::PinExists(gpio.pin));
                }
                GPIO_LIST.input.insert(gpio.pin, pin.into_input());
                Ok(())
            }
            GPIOMode::OUTPUT => {
                if GPIO_LIST.output.contains_key(&gpio.pin) {
                    return Err(GPIOError::PinExists(gpio.pin));
                }
                GPIO_LIST.output.insert(gpio.pin, pin.into_output());
                Ok(())
            }
        },
    }
}

pub fn set(gpio: RemoteGPIO) -> Result<_, GPIOError> {
    if gpio.mode == GPIOMode::INPUT {
        return Err(GPIOError::PinUnmatch(gpio.pin, GPIOMode::OUTPUT));
    }
    if let Some(pin) = GPIO_LIST.output.get(&gpio.pin) {
        match gpio.value {
            0 => pin.set_low(),
            _ => pin.set_high(),
        }
        return Ok(());
    }
    Err(GPIOError::PinNotFound(gpio.pin))
}

pub fn read(gpio: RemoteGPIO) -> Result<u32, GPIOError> {
    if gpio.mode == GPIOMode::OUTPUT {
        return Err(GPIOError::PinUnmatch(gpio.pin, GPIOMode::INPUT));
    }
    if let Some(pin) = GPIO_LIST.input.get(&gpio.pin) {
        match gpio.value {
            0 => pin.set_low(),
            _ => pin.set_high(),
        }
        return Ok(pin.read());
    }
    Err(GPIOError::PinNotFound(gpio.pin))
}
pub fn reset_all() {}
