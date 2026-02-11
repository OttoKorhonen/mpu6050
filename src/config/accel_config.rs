#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccelConfig {
    Range2G = 0x00,
    Range4G = 0x01,
    Range8G = 0x02,
    Range16G = 0x03,
}

impl AccelConfig {
    /// Get register value for ACCEL_CONFIG register
    pub const fn register_value(&self) -> u8 {
        (*self as u8) << 3
    }
    
    /// Get accelerometer full scale range in G
    pub const fn scale_range(&self) -> u8 {
        match self {
            Self::Range2G => 2,
            Self::Range4G => 4,
            Self::Range8G => 8,
            Self::Range16G => 16,
        }
    }
}
