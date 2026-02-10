/// Configuration for the FIFO Enable Register (0x23)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FifoConfig {
    bits: u8,
}

impl FifoConfig {
    /// Creates a new configuration with all FIFO streams disabled.
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    /// Enable Temperature FIFO (Bit 7)
    pub const fn enable_temp(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 7;
        } else {
            self.bits &= !(1 << 7);
        }
        self
    }

    /// Enable Gyroscope X-axis FIFO (Bit 6)
    pub const fn enable_gyro_x(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 6;
        } else {
            self.bits &= !(1 << 6);
        }
        self
    }

    /// Enable Gyroscope Y-axis FIFO (Bit 5)
    pub const fn enable_gyro_y(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 5;
        } else {
            self.bits &= !(1 << 5);
        }
        self
    }

    /// Enable Gyroscope Z-axis FIFO (Bit 4)
    pub const fn enable_gyro_z(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 4;
        } else {
            self.bits &= !(1 << 4);
        }
        self
    }

    /// Helper to enable all Gyro axes at once
    pub const fn enable_gyro_all(self, enable: bool) -> Self {
        self.enable_gyro_x(enable)
            .enable_gyro_y(enable)
            .enable_gyro_z(enable)
    }

    /// Enable Accelerometer FIFO (Bit 3)
    pub const fn enable_accel(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 3;
        } else {
            self.bits &= !(1 << 3);
        }
        self
    }

    /// Enable Slave 2 FIFO (Bit 2)
    pub const fn enable_slave2(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 2;
        } else {
            self.bits &= !(1 << 2);
        }
        self
    }

    /// Enable Slave 1 FIFO (Bit 1)
    pub const fn enable_slave1(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 1;
        } else {
            self.bits &= !(1 << 1);
        }
        self
    }

    /// Enable Slave 0 FIFO (Bit 0)
    pub const fn enable_slave0(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 0;
        } else {
            self.bits &= !(1 << 0);
        }
        self
    }

    pub const fn register_value(&self) -> u8 {
        self.bits
    }
}
