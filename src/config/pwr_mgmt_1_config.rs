use crate::config::clock_source::ClockSource;

///This register allows the user to configure the power mode and clock source. It also provides a bit for
// resetting the entire device, and a bit for disabling the temperature sensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PwrMgmt1 {
    bits: u8,
}

impl PwrMgmt1 {
    ///function creates default configuration
    /// - internal oscillator
    /// - PLL with X axis gyroscope reference
    /// - PLL with Y axis gyroscope reference
    /// - PLL with z axis gyroscope reference
    /// - PLL with external 32.768kHz reference
    /// - PLL with external 19.2MHz reference
    /// - Reserved
    /// - Stops the clock and keeps the timing generator
    // in reset
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    ///By setting SLEEP to 1, the MPU-60X0 can be put into low power sleep mode.
    pub const fn set_sleep(mut self, sleep: bool) -> Self {
        match sleep {
            true => {
                self.bits |= 1 << 6;
            }
            false => {
                self.bits &= !(1 << 6);
            }
        }
        self
    }

    ///When set to 1, this bit resets all internal registers to their default values.
    // The bit automatically clears to 0 once the reset is done.
    pub const fn device_reset(mut self, reset: bool) -> Self {
        match reset {
            true => self.bits |= 1 << 7,
            false => self.bits &= !(1 << 7),
        }
        self
    }

    ///When CYCLE is set to
    // 1 while SLEEP is disabled, the MPU-60X0 will be put into Cycle Mode. In Cycle Mode, the device
    // cycles between sleep mode and waking up to take a single sample of data from accelerometer at a
    // rate determined by LP_WAKE_CTRL (register 108). To configure the wake frequency, use
    // LP_WAKE_CTRL within the Power Management 2 register (Register 108).
    pub const fn set_cycle(mut self, cycle: bool) -> Self {
        match cycle {
            true => self.bits |= 1 << 5,
            false => self.bits &= !(1 << 5),
        }
        self
    }

    ///When set to 1, this bit disables the temperature sensor.
    pub const fn disable_temp_sensor(mut self, enable: bool) -> Self {
        match enable {
            true => self.bits |= 1 << 3,
            false => self.bits &= !(1 << 3),
        }
        self
    }

    pub const fn register_value(&self) -> u8 {
        self.bits
    }

    pub const fn set_clock_source(mut self, clk_source: ClockSource) -> Self {
        self.bits &= 0xF8;
        self.bits |= clk_source as u8;
        self
    }
}
