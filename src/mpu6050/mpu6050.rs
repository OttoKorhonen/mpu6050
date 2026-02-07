use embedded_hal::i2c::SevenBitAddress;

pub struct Mpu6050<I2C> {
    i2c: I2C,
    address: SevenBitAddress
}

impl<I2C> Mpu6050<I2C>
where
    I2C: embedded_hal::i2c::I2c
{
    pub fn new(i2c: I2C, address: SevenBitAddress) -> Self {
        Self{i2c, address}
    }

    pub fn configure(&mut self) {
        todo!()
    }

    pub fn write(&mut self) {
        todo!()
    }

    // pub fn new(i2c: I2C, address: u8) -> Self {
    //     Self {
    //         i2c,
    //         address,
    //         _error: core::marker::PhantomData,
    //     }
    // }
    //
    // ///run this first to initialize device
    // pub fn init(&mut self, fifo: FIFOEnable, gyro: GyroScaleRange, accel: AccelScaleRange) {
    //     self.power_mgmt_1();
    //     self.fifo_enable(fifo);
    //     self.config();
    //     self.set_sample_rate();
    //     self.set_gyro_scale(gyro);
    //     self.set_accel_scale(accel);
    // }
    //
    // ///read temperature value from sensor
    // pub fn read_temp(&mut self) -> f32 {
    //     let mut buf = [0u8; 2]; // buffer (MSB ja LSB)
    //
    //     // first write to the register
    //     self.i2c.write(self.address, &[0x41]).unwrap();
    //
    //     // read MSB (0x41), LSB (0x42)
    //     self.i2c.read(self.address, &mut buf).unwrap();
    //
    //     // bytes in buffer are combined
    //     let raw_temp = ((buf[0] as i16) << 8) | (buf[1] as i16);
    //
    //     self.measure_temp(raw_temp)
    // }
    //
    // ///read all axis values from sensor.
    // /// Values are returned in tuple (X,Y,Z)
    // pub fn read_all_axis(&mut self) -> (i16, i16, i16) {
    //     let x = self.read_x_axis();
    //     let y = self.read_y_axis();
    //     let z = self.read_z_axis();
    //
    //     (x, y, z)
    // }
    //
    // ///read accel from x-axis
    // pub fn read_accel_x_axis(&mut self) -> i16 {
    //     let cmd = [0x3B];
    //     let mut buffer = [0u8; 2];
    //     self.i2c
    //         .write_read(self.address, &cmd, &mut buffer)
    //         .unwrap();
    //
    //     let raw_x_accel = ((buffer[0] as i16) << 8) | (buffer[1] as i16);
    //
    //     raw_x_accel
    // }
    //
    // ///read accel from y-axis
    // pub fn read_accel_y_axis(&mut self) -> i16 {
    //     let cmd = [0x3D];
    //     let mut buffer = [0u8; 2];
    //     self.i2c
    //         .write_read(self.address, &cmd, &mut buffer)
    //         .unwrap();
    //
    //     let raw_y_accel = ((buffer[0] as i16) << 8) | (buffer[1] as i16);
    //
    //     raw_y_accel
    // }
    //
    // ///read accel from z-axis
    // pub fn read_accel_z_axis(&mut self) -> i16 {
    //     let cmd = [0x3F];
    //     let mut buffer = [0u8; 2];
    //     self.i2c
    //         .write_read(self.address, &cmd, &mut buffer)
    //         .unwrap();
    //
    //     let raw_z_accel = ((buffer[0] as i16) << 8) | (buffer[1] as i16);
    //
    //     raw_z_accel
    // }
    //
    // ///read only x axis value from sensor
    // pub fn read_x_axis(&mut self) -> i16 {
    //     let cmd = [0x43];
    //     let mut buffer = [0u8; 2];
    //     self.i2c
    //         .write_read(self.address, &cmd, &mut buffer)
    //         .unwrap();
    //
    //     let raw_x = ((buffer[0] as i16) << 8) | (buffer[1] as i16);
    //
    //     raw_x
    // }
    //
    // ///read only y axis value from sensor
    // pub fn read_y_axis(&mut self) -> i16 {
    //     let cmd = [0x45];
    //     let mut buffer = [0u8; 2];
    //     self.i2c
    //         .write_read(self.address, &cmd, &mut buffer)
    //         .unwrap();
    //
    //     let raw_y = ((buffer[0] as i16) << 8) | (buffer[1] as i16);
    //
    //     raw_y
    // }
    //
    // ///read only z axis value from sensor
    // pub fn read_z_axis(&mut self) -> i16 {
    //     let cmd = [0x47];
    //     let mut buffer = [0u8; 2];
    //     self.i2c
    //         .write_read(self.address, &cmd, &mut buffer)
    //         .unwrap();
    //
    //     let raw_z = ((buffer[0] as i16) << 8) | (buffer[1] as i16);
    //
    //     raw_z
    // }
    //
    // ///this function configures device gyroscope and accellerometer
    // pub fn configure_device(&mut self) {
    //     todo!()
    // }
    //
    // fn set_sample_rate(&mut self) {
    //     let cmd = [0x20, 0b0001_1000];
    //     self.write_to_register(&cmd);
    // }
    //
    // fn measure_temp(&self, raw_data: i16) -> f32 {
    //     raw_data as f32 / 340.0 + 36.5
    // }
    //
    // fn power_mgmt_1(&mut self) {
    //     let cmd = [0x6B, 0b0000_0000];
    //     self.i2c.write(self.address, &cmd).unwrap();
    // }
    //
    // fn power_mgmt_2(&mut self) {
    //     todo!()
    // }
    //
    // ///function to configure the device
    // fn config(&mut self) {
    //     //katso rekisteri 26 sivu 13
    //     // let binary = ((ext as u8) << 3) | (dlp as u8);
    //     let cmd = [0x1A, 0b0000_0000];
    //     self.write_to_register(&cmd);
    //
    //     //let mut res = [];
    //     self.i2c.read(self.address, &mut [0x1A]).unwrap();
    // }
    //
    // ///which registers are enabled
    // fn fifo_enable(&mut self, fifo: FIFOEnable) {
    //     let cmd = [0x23, fifo as u8];
    //     self.write_to_register(&cmd);
    // }
    //
    // fn set_accel_scale(&mut self, scale: AccelScaleRange) {
    //     let cmd = [0x1C, scale as u8];
    //     self.i2c.write(self.address, &cmd).unwrap();
    // }
    //
    // ///set gyroscope scale
    // fn set_gyro_scale(&mut self, scale: GyroScaleRange) {
    //     let cmd = [0x1B, scale as u8];
    //     self.i2c.write(self.address, &cmd).unwrap();
    // }
    //
    // ///function writes given bytes to device register
    // fn write_to_register(&mut self, bytes: &[u8]) {
    //     self.i2c.write(self.address, &bytes).unwrap();
    // }
}
