use crate::protocol::gpio::{GPIOMode, GPIOValue, GPIOValueLevel, RemoteGPIO};
use rppal::gpio::{self, Gpio};
use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

pub enum GPIOError {
    ArgError,
    InternalError,
    PinNotFound(u8),
    PinExists(u8),
    PinUnmatch(u8, GPIOMode),
    PinUnreadable(u8),
    PinUnwriteable(u8),
}

impl std::fmt::Display for GPIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GPIOError::ArgError => "Argument Error.".to_owned(),
                GPIOError::InternalError => "Internal Error.".to_owned(),
                GPIOError::PinNotFound(pin) => format!("PinNotFound: No pin {}.", pin.to_string()),
                GPIOError::PinExists(pin) =>
                    format!("PinExists: Already exists pin {}.", pin.to_string()),
                GPIOError::PinUnmatch(pin, mode) => format!(
                    "PinUnmatch: Pin {} not matched to mode {}.",
                    pin.to_string(),
                    mode
                ),
                GPIOError::PinUnreadable(pin) =>
                    format!("PinUnreadable: Pin {} is not readable.", pin.to_string()),
                GPIOError::PinUnwriteable(pin) =>
                    format!("PinUnwriteable: Pin {} is not writeable.", pin.to_string()),
            }
        )
    }
}

struct GPIOList {
    input: LazyLock<RwLock<HashMap<u8, gpio::InputPin>>>,
    output: LazyLock<RwLock<HashMap<u8, gpio::OutputPin>>>,
}

static GPIO: LazyLock<Gpio> = LazyLock::new(|| Gpio::new().unwrap());

static GPIO_LIST: GPIOList = GPIOList {
    input: LazyLock::new(|| RwLock::new(HashMap::<u8, gpio::InputPin>::new())),
    output: LazyLock::new(|| RwLock::new(HashMap::<u8, gpio::OutputPin>::new())),
};

pub fn init_gpio_controller() {}

pub fn config(gpio: RemoteGPIO) -> Result<(), GPIOError> {
    let pin = GPIO.get(gpio.pin as u8);
    match pin {
        Err(_) => Err(GPIOError::PinNotFound(gpio.pin)),
        Ok(pin) => match gpio.mode {
            GPIOMode::UNKNOWN => Err(GPIOError::ArgError),
            GPIOMode::INPUT => {
                if let Ok(mut list) = GPIO_LIST.input.write() {
                    if list.contains_key(&gpio.pin) {
                        return Err(GPIOError::PinExists(gpio.pin));
                    }
                    list.insert(gpio.pin, pin.into_input_pulldown());
                }
                Ok(())
            }
            GPIOMode::OUTPUT => {
                if let Ok(mut list) = GPIO_LIST.output.write() {
                    if list.contains_key(&gpio.pin) {
                        return Err(GPIOError::PinExists(gpio.pin));
                    }
                    list.insert(gpio.pin, pin.into_output());
                }
                Ok(())
            }
            // TODO: Analog and pwm support
            GPIOMode::ANALOG => Ok(()),
            GPIOMode::PWM => Ok(()),
        },
    }
}

pub fn set(gpio: RemoteGPIO) -> Result<(), GPIOError> {
    match gpio.mode {
        GPIOMode::UNKNOWN | GPIOMode::INPUT => return Err(GPIOError::PinUnwriteable(gpio.pin)),
        GPIOMode::OUTPUT => {
            if let Ok(mut list) = GPIO_LIST.output.write() {
                if let Some(pin) = list.get_mut(&gpio.pin) {
                    match gpio.value {
                        GPIOValue::NONE | GPIOValue::ANALOG(_) | GPIOValue::PWM(_, _) => {
                            return Err(GPIOError::ArgError);
                        }
                        GPIOValue::LEVEL(level) => match level {
                            GPIOValueLevel::LOW => pin.set_low(),
                            GPIOValueLevel::HIGH => pin.set_high(),
                        },
                    }
                }
            }
        }
        // TODO: Write analog and pwm
        GPIOMode::ANALOG => {}
        GPIOMode::PWM => {}
    }
    // GPIO_LIST
    //     .output
    //     .write()
    //     .unwrap()
    //     .entry(gpio.pin)
    //     .and_modify(|pin| match gpio.value {
    //         // 0 => pin.set_low(),
    //         // _ => pin.set_high(),
    //     });

    // Err(GPIOError::PinNotFound(gpio.pin))
    Ok(())
}

pub fn read(gpio: RemoteGPIO) -> Result<GPIOValue, GPIOError> {
    match gpio.mode {
        GPIOMode::UNKNOWN | GPIOMode::OUTPUT => return Err(GPIOError::PinUnreadable(gpio.pin)),
        GPIOMode::INPUT => {
            if let Ok(list) = GPIO_LIST.input.write() {
                if let Some(pin) = list.get(&gpio.pin) {
                    return Ok(match pin.read() {
                        gpio::Level::Low => GPIOValue::LEVEL(GPIOValueLevel::LOW),
                        gpio::Level::High => GPIOValue::LEVEL(GPIOValueLevel::HIGH),
                    });
                }
            }
            return Err(GPIOError::InternalError);
        }
        // TODO: Read analog and pwm pin
        GPIOMode::ANALOG => {}
        GPIOMode::PWM => {}
    }
    Err(GPIOError::PinNotFound(gpio.pin))
}
pub fn reset_all() {
    GPIO_LIST.input.write().unwrap().clear();
    GPIO_LIST.output.write().unwrap().clear();
}
