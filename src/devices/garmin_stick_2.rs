use super::usb::USBDriver;

pub struct GarminStick2 {}

impl GarminStick2 {
    pub fn new() -> USBDriver {
        USBDriver::new(0x0fcf, 0x1008).0
    }
}
