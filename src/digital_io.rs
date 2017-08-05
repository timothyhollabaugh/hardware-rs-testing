use rand;

use hardware::pin::Pin;
use hardware::digital_io::DigitalInput;
use hardware::digital_io::DigitalOutput;
use hardware::digital_io::DigitalValue;

pub struct TestDigitalOutput<'a> {
    pin: &'a mut Pin<u8>,
    value: DigitalValue,
}

impl<'a> TestDigitalOutput<'a> {
    pub fn new(pin: &'a mut Pin<u8>) -> TestDigitalOutput<'a> {
        TestDigitalOutput { pin: pin, value: DigitalValue::Low}
    }
}

impl<'a> DigitalOutput<u8> for TestDigitalOutput<'a> {
    fn write(&mut self, state: DigitalValue) {
        println!("Writing {:?} to {:?}", state, self.pin);
        self.value = state;
    }

    fn read(&self) -> DigitalValue {
        self.value
    }
}

pub struct TestRandomDigitalInput<'a> {
    pin: &'a Pin<u8>,
}

impl<'a> TestRandomDigitalInput<'a> {
    pub fn new(pin: &'a Pin<u8>) -> TestRandomDigitalInput<'a> {
        TestRandomDigitalInput {
            pin: pin,
        }
    }
}

impl<'a> DigitalInput<u8> for TestRandomDigitalInput<'a> {
    fn read(&self) -> DigitalValue {
        DigitalValue::High
    }
}
