
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GyroConfig {
    Dps250 = 0x00,
    Dps500 = 0x01,
    Dps1000 = 0x02,
    Dps2000 = 0x03
}

impl GyroConfig {
    /// Get register value for GYRO_CONFIG register
    pub const fn register_value(&self) -> u8 {
        *self as u8
    }

    /// Get gyroscope full scale range in degrees per second
    pub const fn scale_range(&self) -> u16 {
        match self {
            Self::Dps250 => 250,
            Self::Dps500 => 500,
            Self::Dps1000 => 1000,
            Self::Dps2000 => 2000
        }
    }
}