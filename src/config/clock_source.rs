#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockSource {
    Internal8MHz = 0,
    PllXGyro = 1,
    PllYGyro = 2,
    PllZGyro = 3,
    PllExt32k = 4,
    PllExt19MHz = 5,
    Stop = 7
}
