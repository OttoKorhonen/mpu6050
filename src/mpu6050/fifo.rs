
///This register determines which sensor measurements are loaded into the FIFO buffer
pub enum FIFOEnable {
    All = 0b1111_0000,
    Temp = 0b1000_0000,
    Gyro = 0b0111_0000,
    Accel = 0b0001_0000
}