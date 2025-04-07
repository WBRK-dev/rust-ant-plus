use super::usb::USBDriver;

pub struct GarminStick3 {}

impl GarminStick3 {
    pub fn new() -> USBDriver {
        USBDriver::new(0x0fcf, 0x1009).0
    }
}
