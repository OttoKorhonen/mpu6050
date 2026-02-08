use crate::config::{AccelConfig, DlpfConfig, ExtSync, GyroConfig, IntPinConfig, InterruptEnable};
use crate::errors::MPU6050Error;
use crate::registers::Registers;
use embedded_hal::i2c::SevenBitAddress;

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

    pub fn get_value(&mut self, register: Registers) -> Result<u8, MPU6050Error<I2C::Error>> {
        let mut buffer = [0u8];
        self.i2c.write_read(
            self.address,
            &[register.get_register_address()],
            &mut buffer,
        )?;
        Ok(buffer[0])
    }
}
