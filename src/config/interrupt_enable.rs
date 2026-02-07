/// Interrupt Enable Configuration (Register 0x38)
///
/// Controls which interrupt sources are enabled on the MPU-6050.
/// Multiple interrupts can be enabled simultaneously by combining flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptEnable {
    bits: u8,
}

impl InterruptEnable {
    /// Create empty interrupt configuration (all interrupts disabled)
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Create configuration with all interrupts enabled
    pub const fn all() -> Self {
        Self { bits: 0b1011_0001 }
    }

    /// Enable/disable Data Ready interrupt
    ///
    /// When enabled, generates an interrupt each time a write operation
    /// to all sensor registers has been completed.
    ///
    /// Bit 0 of INT_ENABLE register
    pub const fn with_data_ready(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 0;
        } else {
            self.bits &= !(1 << 0);
        }
        self
    }

    /// Enable/disable I2C Master interrupt
    ///
    /// When enabled, any of the I2C Master interrupt sources
    /// will generate an interrupt.
    ///
    /// Bit 3 of INT_ENABLE register
    pub const fn with_i2c_master(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 3;
        } else {
            self.bits &= !(1 << 3);
        }
        self
    }

    /// Enable/disable FIFO buffer overflow interrupt
    ///
    /// When enabled, a FIFO buffer overflow will generate an interrupt.
    ///
    /// Bit 4 of INT_ENABLE register
    pub const fn with_fifo_overflow(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 4;
        } else {
            self.bits &= !(1 << 4);
        }
        self
    }

    /// Enable/disable Motion Detection interrupt (if available)
    ///
    /// Bit 6 of INT_ENABLE register (MPU-6050 specific)
    pub const fn with_motion_detection(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 6;
        } else {
            self.bits &= !(1 << 6);
        }
        self
    }

    /// Get the register value to write to INT_ENABLE register
    pub const fn register_value(&self) -> u8 {
        self.bits
    }

    /// Create from register value
    pub const fn from_register(bits: u8) -> Self {
        Self { bits }
    }

    /// Check if Data Ready interrupt is enabled
    pub const fn has_data_ready(&self) -> bool {
        (self.bits & (1 << 0)) != 0
    }

    /// Check if I2C Master interrupt is enabled
    pub const fn has_i2c_master(&self) -> bool {
        (self.bits & (1 << 3)) != 0
    }

    /// Check if FIFO overflow interrupt is enabled
    pub const fn has_fifo_overflow(&self) -> bool {
        (self.bits & (1 << 4)) != 0
    }

    /// Check if Motion Detection interrupt is enabled
    pub const fn has_motion_detection(&self) -> bool {
        (self.bits & (1 << 6)) != 0
    }
}

impl Default for InterruptEnable {
    fn default() -> Self {
        Self::empty()
    }
}

