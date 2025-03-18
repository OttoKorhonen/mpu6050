
pub enum GyroScaleRange {
    Degree250  = 0b0000_0000, // FS_SEL = 0
    Degree500  = 0b0000_1000, // FS_SEL = 1
    Degree1000 = 0b0001_0000, // FS_SEL = 2
    Degree2000 = 0b0001_1000, // FS_SEL = 3
}