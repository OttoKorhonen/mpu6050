#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Registers{
    SmprtDiv = 0x19,
    Config = 0x1A,
    GyroConfig = 0x18,
    AccelConfig = 0x1C,
    FifoEn = 0x23,
    InterruptPinCfg = 0x37,
    InterruptEnable = 0x38,
    InterruptStatus = 0x3A,
    UserCtrl = 0x6A,
    FifoCountH = 0x72,
    FifoCountL = 0x73,
    FifoRW = 0x74,
    WhoAmI = 0x75,
    PowerMgmt1 = 0x68,
    AccelXOutH = 0x3B,
    AccelXOutL = 0x3C,
    AccelYOutH = 0x3D,
    AccelYOutL = 0x3E,
    AccelZOutH = 0x3F,
    AccelZOutL = 0x40,
    TempOutH = 0x41,
    TempOutL = 0x42,
    GyroXOutH = 0x43,
    GyroXOutL = 0x44,
    GyroYOutH = 0x45,
    GyroYOutL = 0x46,
    GyroZOutH = 0x47,
    GyroZOutL = 0x48
}

impl Registers {
    pub fn get_register_address(&self) -> u8 {*self as u8}
}