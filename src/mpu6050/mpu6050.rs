use crate::config::{
    AccelConfig, DlpfConfig, ExtSync, FifoConfig, GyroConfig, IntPinConfig,
    InterruptEnable,
    PwrMgmt1,
};
use crate::errors::MPU6050Error;
use crate::registers::Registers;
use embedded_hal::i2c::SevenBitAddress;

/// Helper struct for returning 3D vector data (accel or gyro)
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub struct Mpu6050<I2C> {
    i2c: I2C,
    address: SevenBitAddress,
}

impl<I2C> Mpu6050<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    pub const fn new(i2c: I2C, address: SevenBitAddress) -> Self {
        Self { i2c, address }
    }

    pub fn configure_gyro(
        &mut self,
        gyro_conf: GyroConfig,
    ) -> Result<(), MPU6050Error<I2C::Error>> {
        self.i2c.write(
            self.address,
            &[
                Registers::GyroConfig.get_register_address(),
                gyro_conf.register_value(),
            ],
        )?;
        Ok(())
    }

    pub fn configure_accel(
        &mut self,
        accel_conf: AccelConfig,
    ) -> Result<(), MPU6050Error<I2C::Error>> {
        self.i2c.write(
            self.address,
            &[
                Registers::AccelConfig.get_register_address(),
                accel_conf.register_value(),
            ],
        )?;
        Ok(())
    }

    pub fn configure_dlpf_and_ext_sync(
        &mut self,
        dlpf_config: DlpfConfig,
        ext_sync: ExtSync,
    ) -> Result<(), MPU6050Error<I2C::Error>> {
        let config_value = (ext_sync.get_register_value() << 3) | dlpf_config.register_value();
        self.i2c.write(
            self.address,
            &[Registers::Config.get_register_address(), config_value],
        )?;
        Ok(())
    }

    pub fn configure_interrupt_enable(
        &mut self,
        interrupt_enable: InterruptEnable,
    ) -> Result<(), MPU6050Error<I2C::Error>> {
        self.i2c.write(
            self.address,
            &[
                Registers::InterruptEnable.get_register_address(),
                interrupt_enable.register_value(),
            ],
        )?;
        Ok(())
    }

    pub fn configure_interrupt_pin(
        &mut self,
        interrupt_pin_conf: IntPinConfig,
    ) -> Result<(), MPU6050Error<I2C::Error>> {
        self.i2c.write(
            self.address,
            &[
                Registers::InterruptPinCfg.get_register_address(),
                interrupt_pin_conf.register_value(),
            ],
        )?;
        Ok(())
    }

    pub fn configure_power_management(
        &mut self,
        pwr_mgmt: PwrMgmt1,
    ) -> Result<(), MPU6050Error<I2C::Error>> {
        self.i2c.write(
            self.address,
            &[
                Registers::PowerMgmt1.get_register_address(),
                pwr_mgmt.register_value(),
            ],
        )?;
        Ok(())
    }

    pub fn configure_fifo(
        &mut self,
        fifo_config: FifoConfig,
    ) -> Result<(), MPU6050Error<I2C::Error>> {
        self.i2c.write(
            self.address,
            &[
                Registers::FifoEn.get_register_address(),
                fifo_config.register_value(),
            ],
        )?;
        Ok(())
    }

    /// Enable or disable the FIFO buffer.
    /// This sets bit 6 in the USER_CTRL register.
    pub fn set_fifo_enabled(&mut self, enable: bool) -> Result<(), MPU6050Error<I2C::Error>> {
        let mut buffer = [0u8];
        self.i2c.write_read(
            self.address,
            &[Registers::UserCtrl.get_register_address()],
            &mut buffer,
        )?;
        let mut value = buffer[0];

        if enable {
            value |= 1 << 6;
        } else {
            value &= !(1 << 6);
        }

        self.i2c.write(
            self.address,
            &[Registers::UserCtrl.get_register_address(), value],
        )?;
        Ok(())
    }

    /// Reset the FIFO buffer.
    /// This sets bit 2 in the USER_CTRL register.
    /// The bit automatically clears to 0.
    pub fn reset_fifo(&mut self) -> Result<(), MPU6050Error<I2C::Error>> {
        let mut buffer = [0u8];
        self.i2c.write_read(
            self.address,
            &[Registers::UserCtrl.get_register_address()],
            &mut buffer,
        )?;
        let mut value = buffer[0];

        value |= 1 << 2;

        self.i2c.write(
            self.address,
            &[Registers::UserCtrl.get_register_address(), value],
        )?;
        Ok(())
    }

    /// Get current number of bytes in FIFO buffer.
    pub fn get_fifo_count(&mut self) -> Result<u16, MPU6050Error<I2C::Error>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(
            self.address,
            &[Registers::FifoCountH.get_register_address()],
            &mut buffer,
        )?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Read data from FIFO buffer.
    /// The buffer length determines how many bytes are read.
    pub fn read_fifo(&mut self, buffer: &mut [u8]) -> Result<(), MPU6050Error<I2C::Error>> {
        self.i2c.write_read(
            self.address,
            &[Registers::FifoRW.get_register_address()],
            buffer,
        )?;
        Ok(())
    }

    /// Reads raw accelerometer data for X, Y, and Z axes.
    pub fn read_accel(&mut self) -> Result<Vector3, MPU6050Error<I2C::Error>> {
        let mut buffer = [0u8; 6];
        self.i2c.write_read(
            self.address,
            &[Registers::AccelXOutH.get_register_address()],
            &mut buffer,
        )?;

        Ok(Vector3 {
            x: i16::from_be_bytes([buffer[0], buffer[1]]),
            y: i16::from_be_bytes([buffer[2], buffer[3]]),
            z: i16::from_be_bytes([buffer[4], buffer[5]]),
        })
    }

    /// Reads raw gyroscope data for X, Y, and Z axes.
    pub fn read_gyro(&mut self) -> Result<Vector3, MPU6050Error<I2C::Error>> {
        let mut buffer = [0u8; 6];
        self.i2c.write_read(
            self.address,
            &[Registers::GyroXOutH.get_register_address()],
            &mut buffer,
        )?;

        Ok(Vector3 {
            x: i16::from_be_bytes([buffer[0], buffer[1]]),
            y: i16::from_be_bytes([buffer[2], buffer[3]]),
            z: i16::from_be_bytes([buffer[4], buffer[5]]),
        })
    }
}
