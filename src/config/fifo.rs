#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FIFO {
    All = 0xF0,
    Temp = 0x80,
    Gyro = 0x70,
    Accel = 0x10,
}

impl FIFO {
    pub fn get_register_value(&self) -> u8 {
        *self as u8
    }
}
