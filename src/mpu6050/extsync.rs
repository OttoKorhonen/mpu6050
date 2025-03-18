
pub enum ExtSync {
    Disabled   = 0b000,
    TempLsb    = 0b001,
    GyroXLSB   = 0b010,
    GyroYLSB   = 0b011,
    GyroZLSB   = 0b100,
    AccelXLSB  = 0b101,
    AccelYLSB  = 0b110,
    AccelZLSB  = 0b111,
}
