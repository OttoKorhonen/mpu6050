/// Interrupt Pin Configuration (Register 0x37)
///
/// Configures the behavior of the interrupt pin and other related settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntPinConfig {
    bits: u8,
}

impl IntPinConfig {
    /// Create default configuration
    /// - INT pin: active high, push-pull
    /// - Latch mode: 50us pulse
    /// - Clear on status read only
    /// - FSYNC: active high, disabled
    /// - I2C bypass: disabled
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    /// Set INT pin logic level
    ///
    /// - `false` (default): INT pin is active high
    /// - `true`: INT pin is active low
    ///
    /// Bit 7 of INT_PIN_CFG register
    pub const fn with_int_level_active_low(mut self, active_low: bool) -> Self {
        if active_low {
            self.bits |= 1 << 7;
        } else {
            self.bits &= !(1 << 7);
        }
        self
    }

    /// Set INT pin output mode
    ///
    /// - `false` (default): Push-pull output
    /// - `true`: Open drain output
    ///
    /// Bit 6 of INT_PIN_CFG register
    pub const fn with_int_open_drain(mut self, open_drain: bool) -> Self {
        if open_drain {
            self.bits |= 1 << 6;
        } else {
            self.bits &= !(1 << 6);
        }
        self
    }

    /// Set interrupt latch mode
    ///
    /// - `false` (default): INT pin emits 50μs pulse
    /// - `true`: INT pin held high/low until interrupt is cleared
    ///
    /// Bit 5 of INT_PIN_CFG register
    pub const fn with_latch_int(mut self, latch: bool) -> Self {
        if latch {
            self.bits |= 1 << 5;
        } else {
            self.bits &= !(1 << 5);
        }
        self
    }

    /// Set interrupt status clear mode
    ///
    /// - `false` (default): Clear only by reading INT_STATUS (Register 58)
    /// - `true`: Clear on any read operation
    ///
    /// Bit 4 of INT_PIN_CFG register
    pub const fn with_int_rd_clear(mut self, clear_on_any_read: bool) -> Self {
        if clear_on_any_read {
            self.bits |= 1 << 4;
        } else {
            self.bits &= !(1 << 4);
        }
        self
    }

    /// Set FSYNC pin logic level
    ///
    /// - `false` (default): FSYNC pin is active high
    /// - `true`: FSYNC pin is active low
    ///
    /// Bit 3 of INT_PIN_CFG register
    pub const fn with_fsync_level_active_low(mut self, active_low: bool) -> Self {
        if active_low {
            self.bits |= 1 << 3;
        } else {
            self.bits &= !(1 << 3);
        }
        self
    }

    /// Enable FSYNC pin as interrupt
    ///
    /// - `false` (default): FSYNC pin disabled as interrupt
    /// - `true`: FSYNC pin enabled as interrupt to host processor
    ///
    /// Bit 2 of INT_PIN_CFG register
    pub const fn with_fsync_int_enabled(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 2;
        } else {
            self.bits &= !(1 << 2);
        }
        self
    }

    /// Enable I2C bypass mode
    ///
    /// When enabled and I2C_MST_EN=0, allows direct access to auxiliary I2C bus.
    ///
    /// - `false` (default): Bypass disabled
    /// - `true`: Bypass enabled
    ///
    /// Bit 1 of INT_PIN_CFG register
    pub const fn with_i2c_bypass_enabled(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= 1 << 1;
        } else {
            self.bits &= !(1 << 1);
        }
        self
    }

    /// Get register value to write to INT_PIN_CFG register
    pub const fn register_value(&self) -> u8 {
        self.bits
    }

    /// Create from register value
    pub const fn from_register(bits: u8) -> Self {
        Self { bits }
    }

    /// Check if INT pin is active low
    pub const fn is_int_active_low(&self) -> bool {
        (self.bits & (1 << 7)) != 0
    }

    /// Check if INT pin is open drain
    pub const fn is_int_open_drain(&self) -> bool {
        (self.bits & (1 << 6)) != 0
    }

    /// Check if interrupt latch is enabled
    pub const fn is_latch_enabled(&self) -> bool {
        (self.bits & (1 << 5)) != 0
    }

    /// Check if interrupt clears on any read
    pub const fn is_clear_on_any_read(&self) -> bool {
        (self.bits & (1 << 4)) != 0
    }

    /// Check if FSYNC is active low
    pub const fn is_fsync_active_low(&self) -> bool {
        (self.bits & (1 << 3)) != 0
    }

    /// Check if FSYNC interrupt is enabled
    pub const fn is_fsync_int_enabled(&self) -> bool {
        (self.bits & (1 << 2)) != 0
    }

    /// Check if I2C bypass is enabled
    pub const fn is_i2c_bypass_enabled(&self) -> bool {
        (self.bits & (1 << 1)) != 0
    }
}

impl Default for IntPinConfig {
    fn default() -> Self {
        Self::new()
    }
}
