
pub enum AccelScaleRange {
    TwoG = 0b0000_0000, // AFS_SEL = 0
    FourG  = 0b0000_1000, // AFS_SEL = 1
    EightG = 0b0001_0000, // AFS_SEL = 2
    SixteenG = 0b0001_1000, // AFS_SEL = 3
}