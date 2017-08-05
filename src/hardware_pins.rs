
use hardware::pin::Pin;

pub struct HardwarePins {
    
    // Pins
    pub pin1: Pin<u8>,
    pub pin2: Pin<u8>,
    pub pin3: Pin<u8>,
    pub pin4: Pin<u8>,
    pub pin5: Pin<u8>,
    pub pin6: Pin<u8>,
    pub pin7: Pin<u8>,
    pub pin8: Pin<u8>,

}

pub fn get_hardware_pins() -> HardwarePins {
    HardwarePins {
        pin1: Pin(1),
        pin2: Pin(2),
        pin3: Pin(3),
        pin4: Pin(4),
        pin5: Pin(5),
        pin6: Pin(6),
        pin7: Pin(7),
        pin8: Pin(8),
    }
}
