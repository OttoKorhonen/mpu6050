mod config;
mod accel_config;
mod gyro_config;
mod fifo;
mod int_pin_config;
mod interrupt_enable;

// Public exports
pub use config::{ExtSync, DlpfConfig};
pub use accel_config::AccelConfig;
pub use gyro_config::GyroConfig;
pub use int_pin_config::IntPinConfig;
pub use interrupt_enable::InterruptEnable;
