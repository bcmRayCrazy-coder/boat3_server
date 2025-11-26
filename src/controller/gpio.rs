use crate::protocol::gpio::{GPIOMode, RemoteGPIO};
use rppal::gpio::{self, Gpio};
use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

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
                GPIOError::ArgError => "Argument Error".to_owned(),
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
    input: LazyLock<RwLock<HashMap<u32, gpio::InputPin>>>,
    output: LazyLock<RwLock<HashMap<u32, gpio::OutputPin>>>,
}

static GPIO: LazyLock<Gpio> = LazyLock::new(|| Gpio::new().unwrap());

static GPIO_LIST: GPIOList = GPIOList {
    input: LazyLock::new(|| RwLock::new(HashMap::<u32, gpio::InputPin>::new())),
    output: LazyLock::new(|| RwLock::new(HashMap::<u32, gpio::OutputPin>::new())),
};

pub fn init_gpio_controller() {}

pub fn config(gpio: RemoteGPIO) -> Result<(), GPIOError> {
    let pin = GPIO.get(gpio.pin as u8);
    match pin {
        Err(_) => Err(GPIOError::PinNotFound(gpio.pin)),
        Ok(pin) => match gpio.mode {
            GPIOMode::UNKNOWN => Err(GPIOError::ArgError),
            GPIOMode::INPUT => {
                if GPIO_LIST.input.read().unwrap().contains_key(&gpio.pin) {
                    return Err(GPIOError::PinExists(gpio.pin));
                }
                GPIO_LIST
                    .input
                    .write()
                    .unwrap()
                    .insert(gpio.pin, pin.into_input());
                Ok(())
            }
            GPIOMode::OUTPUT => {
                if GPIO_LIST.output.read().unwrap().contains_key(&gpio.pin) {
                    return Err(GPIOError::PinExists(gpio.pin));
                }
                GPIO_LIST
                    .output
                    .write()
                    .unwrap()
                    .insert(gpio.pin, pin.into_output());
                Ok(())
            }
        },
    }
}

pub fn set(gpio: RemoteGPIO) -> Result<(), GPIOError> {
    if gpio.mode == GPIOMode::INPUT {
        return Err(GPIOError::PinUnmatch(gpio.pin, GPIOMode::OUTPUT));
    }
    GPIO_LIST
        .output
        .write()
        .unwrap()
        .entry(gpio.pin)
        .and_modify(|pin| match gpio.value {
            0 => pin.set_low(),
            _ => pin.set_high(),
        });

    // Err(GPIOError::PinNotFound(gpio.pin))
    Ok(())
}

pub fn read(gpio: RemoteGPIO) -> Result<u32, GPIOError> {
    if gpio.mode == GPIOMode::OUTPUT {
        return Err(GPIOError::PinUnmatch(gpio.pin, GPIOMode::INPUT));
    }
    if let Some(pin) = GPIO_LIST.input.write().unwrap().get(&gpio.pin) {
        return Ok(match pin.read() {
            gpio::Level::Low => 0,
            gpio::Level::High => 1,
        });
    }
    Err(GPIOError::PinNotFound(gpio.pin))
}
pub fn reset_all() {
    GPIO_LIST.input.write().unwrap().clear();
    GPIO_LIST.output.write().unwrap().clear();
}
