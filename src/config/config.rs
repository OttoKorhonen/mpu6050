/// External Frame Synchronization (EXT_SYNC_SET)
/// Enables the FSYNC pin to be used as a synchronization signal.
/// The FSYNC signal is sampled and its edge is detected by the MPU-6050.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExtSync {
    /// Input disabled
    Disabled = 0x00,
    /// TEMP_OUT_L[0]
    TempOutL = 0x01,
    /// GYRO_XOUT_L[0]
    GyroXOutL = 0x02,
    /// GYRO_YOUT_L[0]
    GyroYOutL = 0x03,
    /// GYRO_ZOUT_L[0]
    GyroZOutL = 0x04,
    /// ACCEL_XOUT_L[0]
    AccelXOutL = 0x05,
    /// ACCEL_YOUT_L[0]
    AccelYOutL = 0x06,
    /// ACCEL_ZOUT_L[0]
    AccelZOutL = 0x07,
}

impl ExtSync {
    /// Convert to register value
    pub const fn get_register_value(&self) -> u8 {
        *self as u8
    }
}

/// Digital Low Pass Filter (DLPF) Configuration
/// Configures the Digital Low Pass Filter setting for both the gyroscope and accelerometer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DlpfConfig {
    /// Accelerometer: bandwidth=260Hz, delay=0ms
    /// Gyroscope: bandwidth=256Hz, delay=0.98ms, Fs=8kHz
    Dlpf0 = 0x00,

    /// Accelerometer: bandwidth=184Hz, delay=2.0ms
    /// Gyroscope: bandwidth=188Hz, delay=1.9ms, Fs=1kHz
    Dlpf1 = 0x01,

    /// Accelerometer: bandwidth=94Hz, delay=3.0ms
    /// Gyroscope: bandwidth=98Hz, delay=2.8ms, Fs=1kHz
    Dlpf2 = 0x02,

    /// Accelerometer: bandwidth=44Hz, delay=4.9ms
    /// Gyroscope: bandwidth=42Hz, delay=4.8ms, Fs=1kHz
    Dlpf3 = 0x03,

    /// Accelerometer: bandwidth=21Hz, delay=8.5ms
    /// Gyroscope: bandwidth=20Hz, delay=8.3ms, Fs=1kHz
    Dlpf4 = 0x04,

    /// Accelerometer: bandwidth=10Hz, delay=13.8ms
    /// Gyroscope: bandwidth=10Hz, delay=13.4ms, Fs=1kHz
    Dlpf5 = 0x05,

    /// Accelerometer: bandwidth=5Hz, delay=19.0ms
    /// Gyroscope: bandwidth=5Hz, delay=18.6ms, Fs=1kHz
    Dlpf6 = 0x06,
}

impl DlpfConfig {
    /// Get accelerometer bandwidth in Hz
    pub const fn get_accel_bandwidth_hz(&self) -> u16 {
        match self {
            Self::Dlpf0 => 260,
            Self::Dlpf1 => 184,
            Self::Dlpf2 => 94,
            Self::Dlpf3 => 44,
            Self::Dlpf4 => 21,
            Self::Dlpf5 => 10,
            Self::Dlpf6 => 5,
        }
    }

    /// Get gyroscope bandwidth in Hz
    pub const fn get_gyro_bandwidth_hz(&self) -> u16 {
        match self {
            Self::Dlpf0 => 256,
            Self::Dlpf1 => 188,
            Self::Dlpf2 => 98,
            Self::Dlpf3 => 42,
            Self::Dlpf4 => 20,
            Self::Dlpf5 => 10,
            Self::Dlpf6 => 5,
        }
    }

    /// Convert to register value
    pub const fn register_value(&self) -> u8 {
        *self as u8
    }
}

