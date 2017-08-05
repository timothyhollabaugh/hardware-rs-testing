
extern crate hardware;
extern crate rand;

mod hardware_pins;
mod digital_io;

use hardware::digital_io::DigitalValue;
use hardware::digital_io::DigitalInput;
use hardware::digital_io::DigitalOutput;

use hardware_pins::get_hardware_pins;
use digital_io::TestDigitalOutput;
use digital_io::TestRandomDigitalInput;

fn main() {
    println!("Hello!");

    let mut hardware_pins = get_hardware_pins();

    let mut out1 = TestDigitalOutput::new(&mut hardware_pins.pin1);
    let mut out2 = TestDigitalOutput::new(&mut hardware_pins.pin2);

    out1.write(DigitalValue::High);
    out2.write(DigitalValue::Low);

    let mut in1 = TestRandomDigitalInput::new(&hardware_pins.pin3);

    println!("{:?}", in1.read())
}
