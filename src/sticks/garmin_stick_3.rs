use super::Driver;

pub struct GarminStick3;

impl GarminStick3 {
    pub fn new() -> Driver {
        Driver::new(0x0fcf, 0x1009)
    }
}